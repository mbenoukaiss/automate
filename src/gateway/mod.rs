//! Tools to interact with Discord's gateway API

mod models;

pub use models::*;

use crate::{map, Error, Configuration};
use crate::http::HttpAPI;
use crate::encode::json;
use crate::events::ListenerType;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::ops::Deref;
use futures::{SinkExt, StreamExt};
use futures::lock::Mutex;
use futures::stream;
use futures::channel::mpsc;
use futures::channel::mpsc::SendError;
use futures::channel::mpsc::UnboundedSender;
use tungstenite::Message as TkMessage;
use chrono::NaiveDateTime;
use std::cell::RefCell;

macro_rules! call_dispatcher {
    ($data:ident as $payload:ty => $self:ident.$method:ident) => {{
        let payload: $payload = ::serde_json::from_str(&$data)?;

        if let Some(val) = payload.s {
            *$self.sequence_number.lock().await = Some(val);
        }

        $self.$method(payload.d).await?
    }};
}

macro_rules! dispatcher {
    ($fn_name:ident: $type:ty => $name:ident) => {
        async fn $fn_name(&mut self, payload: $type) -> Result<(), Error> {
            let mut context = Context::new(&self);

            for listener in &mut *self.config.listeners.trait_listeners {
                if let Err(error) = (*listener).$fn_name(&mut context, &payload).await {
                    error!("Listener to {} failed with: {}", stringify!($name), error);
                }
            }

            for listener in &*self.config.listeners.$name {
                if let Err(error) = (*listener)(&mut context, &payload).await {
                    error!("Listener to {} failed with: {}", stringify!($name), error);
                }
            }

            //TODO: can be improved by splitting calls to mutable and immutable versions?
            for listener in &mut *self.config.listeners.stateful_listeners {
                (*listener).$fn_name(&mut context, &payload).await;
            }

            self.config.listeners.register(context.new_listeners);

            Ok(())
        }
    }
}

tokio::task_local! {
    /// Rate-limit for gateway
    /// Since gateway allow 120 events every 60 seconds, we're
    /// storing the number of remaining events and the time at
    /// which it will reset.
    static RATE_LIMIT: RefCell<(i32, NaiveDateTime)>;
}

/// Context about the current gateway session.
/// Provides a way to interact with Discord HTTP API
/// by dereferencing to [HttpAPI](automate::http::HttpAPI).
pub struct Context {
    sender: UnboundedSender<TkMessage>,
    http: Arc<HttpAPI>,
    bot: Option<Arc<User>>,
    new_listeners: Vec<ListenerType>,
}

impl Context {
    fn new(gateway: &GatewayAPI) -> Context {
        Context {
            sender: gateway.msg_sender.clone(),
            http: Arc::clone(&gateway.http),
            bot: gateway.bot.clone(),
            new_listeners: Vec::new()
        }    
    }

    /// Sends a message to Discord through the websocket.
    /// The message must be a valid payload.
    #[inline]
    async fn send<M: Into<TkMessage>>(&mut self, msg: M) -> Result<(), Error> {
        Ok(self.sender.send(msg.into()).await?)
    }

    /// Registers an event listener struct that implements
    /// the [Listener](automate::Listener) trait or
    /// a listener function with the `̀#[listener]` attribute.
    pub async fn register(&mut self, mut listeners: Vec<ListenerType>) {
        self.new_listeners.append(&mut listeners);
    }

    /// The user object of the bot.
    #[inline]
    pub fn bot(&self) -> &User {
        self.bot.as_ref().unwrap()
    }

    /// Creates a link to invite the bot to a discord server
    /// and give him the specified permissions.
    #[inline]
    pub fn invite_bot(&self, permission: u32) -> String {
        format!("https://discordapp.com/oauth2/authorize?client_id={}&scope=bot&permissions={}", self.bot().id, permission)
    }
}

impl Deref for Context {
    type Target = Arc<HttpAPI>;

    #[inline]
    fn deref(&self) -> &Arc<HttpAPI> {
        &self.http
    }
}

#[derive(Debug)]
enum Direction {
    Receive(Result<TkMessage, tungstenite::Error>),
    Send(TkMessage),
}

/// Communicates with Discord's gateway
pub(crate) struct GatewayAPI<'a> {
    config: &'a mut Configuration,
    session_id: Option<String>,
    sequence_number: Arc<Mutex<Option<i32>>>,
    heartbeat_confirmed: Arc<AtomicBool>,
    msg_sender: UnboundedSender<TkMessage>,
    http: Arc<HttpAPI>,
    bot: Option<Arc<User>>,
}

impl<'a> GatewayAPI<'a> {
    /// Establishes a connection to Discord's
    /// gateway and calls the provided listeners
    /// when receiving an event.
    pub(crate) async fn connect(mut config: Configuration, url: String) {
        let http = Arc::new(HttpAPI::new(&config.token));
        let sequence_number = Arc::new(Mutex::new(None));
        let mut session_id = None;

        loop {
            let execution: Result<(), Error> = try {
                let (tx, rx) = mpsc::unbounded();
                let (socket, _) = tktungstenite::connect_async(&url).await?;

                let mut gateway = GatewayAPI {
                    config: &mut config,
                    session_id: None,
                    sequence_number: Arc::clone(&sequence_number),
                    heartbeat_confirmed: Arc::new(AtomicBool::new(true)),
                    msg_sender: tx,
                    http: http.clone(),
                    bot: None,
                };

                let mut select = stream::select(
                    socket.map(Direction::Receive),
                    rx.map(Direction::Send),
                );

                while let Some(message) = select.next().await {
                    match message {
                        Direction::Receive(message) => gateway.on_message(message?).await?,
                        Direction::Send(message) => select.get_mut().0.send(message).await?
                    }
                }

                select.get_mut().0.close().await?;

                session_id = gateway.session_id;
            };

            if let Err(err) = execution {
                error!("Connection was interrupted with '{}'", err.msg);
            } else {
                error!("Connection interrupted");
            }

            if let Some(sid) = session_id.as_ref() {
                info!("Attempting to resume session {} with sequence_number: {}", sid, sequence_number.lock().await.unwrap());
            } else {
                info!("Attempting to reconnect");
            }
        }
    }

    #[inline]
    async fn send<M: Into<TkMessage>>(&mut self, msg: M) -> Result<(), Error> {
        Ok(self.msg_sender.send(msg.into()).await?)
    }

    #[inline]
    pub fn invite_bot(&self, permission: u32) -> String {
        format!("https://discordapp.com/oauth2/authorize?client_id={}&scope=bot&permissions={}", self.bot.as_ref().unwrap().id, permission)
    }

    async fn on_message(&mut self, msg: TkMessage) -> Result<(), Error> {
        match msg {
            TkMessage::Text(data) => {
                if let Err(err) = self.dispatch_payload(&data).await {
                    error!("An error occurred while reading message: {}\n{}", err.msg, data);
                }
            },
            TkMessage::Close(_close) => return Error::err("Websocket connection closed"),
            _ => error!("Unknown message type received")
        };

        Ok(())
    }

    async fn dispatch_payload(&mut self, data: &str) -> Result<(), Error> {
        match json::root_search::<u8>("op", data)? {
            0 => self.dispatch_event(data).await?,
            7 => self.on_reconnect().await?,
            9 => call_dispatcher!(data as Payload<InvalidSession> => self.on_invalid_session),
            10 => call_dispatcher!(data as Payload<Hello> => self.on_hello),
            11 => self.on_heartbeat_ack().await?,
            unknown_op => warn!("Received unknown opcode '{}': \n{}", unknown_op, data)
        }

        Ok(())
    }

    /// Takes a full payload, deserializes it and sends
    /// it to the right method.
    /// Returns an error when receiving an unknown event.
    #[allow(clippy::cognitive_complexity)]
    // Currently disabling cognitive complexity since clippy
    // expands the macros (bug) before calculating CoC.
    async fn dispatch_event(&mut self, data: &str) -> Result<(), Error> {
        match json::root_search::<String>("t", data)?.as_str() {
            ReadyDispatch::EVENT_NAME => call_dispatcher!(data as Payload<ReadyDispatch> => self.on_ready),
            ResumedDispatch::EVENT_NAME => call_dispatcher!(data as Payload<ResumedDispatch> => self.on_resumed),
            ChannelCreateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<ChannelCreateDispatch> => self.on_channel_create),
            ChannelUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<ChannelUpdateDispatch> => self.on_channel_update),
            ChannelDeleteDispatch::EVENT_NAME => call_dispatcher!(data as Payload<ChannelDeleteDispatch> => self.on_channel_delete),
            ChannelPinsUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<ChannelPinsUpdateDispatch> => self.on_channel_pins_update),
            GuildCreateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildCreateDispatch> => self.on_guild_create),
            GuildUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildUpdateDispatch> => self.on_guild_update),
            GuildDeleteDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildDeleteDispatch> => self.on_guild_delete),
            GuildBanAddDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildBanAddDispatch> => self.on_guild_ban_add),
            GuildBanRemoveDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildBanRemoveDispatch> => self.on_guild_ban_remove),
            GuildEmojisUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildEmojisUpdateDispatch> => self.on_guild_emojis_update),
            GuildIntegrationsUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildIntegrationsUpdateDispatch> => self.on_guild_integrations_update),
            GuildMemberAddDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildMemberAddDispatch> => self.on_guild_member_add),
            GuildMemberRemoveDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildMemberRemoveDispatch> => self.on_guild_member_remove),
            GuildMemberUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildMemberUpdateDispatch> => self.on_guild_member_update),
            GuildMembersChunkDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildMembersChunkDispatch> => self.on_guild_members_chunk),
            GuildRoleCreateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildRoleCreateDispatch> => self.on_guild_role_create),
            GuildRoleUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildRoleUpdateDispatch> => self.on_guild_role_update),
            GuildRoleDeleteDispatch::EVENT_NAME => call_dispatcher!(data as Payload<GuildRoleDeleteDispatch> => self.on_guild_role_delete),
            InviteCreateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<InviteCreateDispatch> => self.on_invite_create),
            InviteDeleteDispatch::EVENT_NAME => call_dispatcher!(data as Payload<InviteDeleteDispatch> => self.on_invite_delete),
            MessageCreateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageCreateDispatch> => self.on_message_create),
            MessageUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageUpdateDispatch> => self.on_message_update),
            MessageDeleteDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageDeleteDispatch> => self.on_message_delete),
            MessageDeleteBulkDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageDeleteBulkDispatch> => self.on_message_delete_bulk),
            MessageReactionAddDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageReactionAddDispatch> => self.on_reaction_add),
            MessageReactionRemoveDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageReactionRemoveDispatch> => self.on_reaction_remove),
            MessageReactionRemoveAllDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageReactionRemoveAllDispatch> => self.on_reaction_remove_all),
            MessageReactionRemoveEmojiDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageReactionRemoveEmojiDispatch> => self.on_reaction_remove_emoji),
            PresencesReplaceDispatch::EVENT_NAME => info!("Ignoring presence replace event"),
            PresenceUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<PresenceUpdateDispatch> => self.on_presence_update),
            TypingStartDispatch::EVENT_NAME => call_dispatcher!(data as Payload<TypingStartDispatch> => self.on_typing_start),
            UserUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<UserUpdateDispatch> => self.on_user_update),
            VoiceStateUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<VoiceStateUpdateDispatch> => self.on_voice_state_update),
            VoiceServerUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<VoiceServerUpdateDispatch> => self.on_voice_server_update),
            WebhooksUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<WebhooksUpdateDispatch> => self.on_webhooks_update),
            unknown_event => return Error::err(format!("Unknown event {}", unknown_event))
        }

        Ok(())
    }

    dispatcher!(on_channel_create: ChannelCreateDispatch => channel_create);
    dispatcher!(on_channel_update: ChannelUpdateDispatch => channel_update);
    dispatcher!(on_channel_delete: ChannelDeleteDispatch => channel_delete);
    dispatcher!(on_channel_pins_update: ChannelPinsUpdateDispatch => channel_pins_update);
    dispatcher!(on_guild_create: GuildCreateDispatch => guild_create);
    dispatcher!(on_guild_update: GuildUpdateDispatch => guild_update);
    dispatcher!(on_guild_delete: GuildDeleteDispatch => guild_delete);
    dispatcher!(on_guild_ban_add: GuildBanAddDispatch => guild_ban_add);
    dispatcher!(on_guild_ban_remove: GuildBanRemoveDispatch => guild_ban_remove);
    dispatcher!(on_guild_emojis_update: GuildEmojisUpdateDispatch => guild_emojis_update);
    dispatcher!(on_guild_integrations_update: GuildIntegrationsUpdateDispatch => guild_integrations_update);
    dispatcher!(on_guild_member_add: GuildMemberAddDispatch => guild_member_add);
    dispatcher!(on_guild_member_remove: GuildMemberRemoveDispatch => guild_member_remove);
    dispatcher!(on_guild_member_update: GuildMemberUpdateDispatch => guild_member_update);
    dispatcher!(on_guild_members_chunk: GuildMembersChunkDispatch => guild_members_chunk);
    dispatcher!(on_guild_role_create: GuildRoleCreateDispatch => guild_role_create);
    dispatcher!(on_guild_role_update: GuildRoleUpdateDispatch => guild_role_update);
    dispatcher!(on_guild_role_delete: GuildRoleDeleteDispatch => guild_role_delete);
    dispatcher!(on_invite_create: InviteCreateDispatch => invite_create);
    dispatcher!(on_invite_delete: InviteDeleteDispatch => invite_delete);
    dispatcher!(on_message_create: MessageCreateDispatch => message_create);
    dispatcher!(on_message_update: MessageUpdateDispatch => message_update);
    dispatcher!(on_message_delete: MessageDeleteDispatch => message_delete);
    dispatcher!(on_message_delete_bulk: MessageDeleteBulkDispatch => message_delete_bulk);
    dispatcher!(on_reaction_add: MessageReactionAddDispatch => reaction_add);
    dispatcher!(on_reaction_remove: MessageReactionRemoveDispatch => reaction_remove);
    dispatcher!(on_reaction_remove_all: MessageReactionRemoveAllDispatch => reaction_remove_all);
    dispatcher!(on_reaction_remove_emoji: MessageReactionRemoveEmojiDispatch => reaction_remove_emoji);
    dispatcher!(on_presence_update: PresenceUpdateDispatch => presence_update);
    dispatcher!(on_typing_start: TypingStartDispatch => typing_start);
    dispatcher!(on_user_update: UserUpdateDispatch => user_update);
    dispatcher!(on_voice_state_update: VoiceStateUpdateDispatch => voice_state_update);
    dispatcher!(on_voice_server_update: VoiceServerUpdateDispatch => voice_server_update);
    dispatcher!(on_webhooks_update: WebhooksUpdateDispatch => webhooks_update);

    async fn on_hello(&mut self, payload: Hello) -> Result<(), Error> {
        if self.session_id.is_some() {
            let resume = Resume {
                token: self.http.token().clone(),
                session_id: self.session_id.as_ref().unwrap().clone(),
                seq: self.sequence_number.lock().await.unwrap(),
            };

            self.send(resume).await?;
            info!("Requested to resume session");
        } else {
            let identify = Identify {
                token: self.http.token().clone(),
                properties: map! {
                    "$os" => "linux",
                    "$browser" => "automate",
                    "$device" => "automate"
                },
                compress: false,
                shard: [self.config.shard_id.unwrap(), self.config.total_shards.unwrap()],
                large_threshold: None,
                presence: None,
                guild_subscriptions: Some(true),
                intents: self.config.intents
            };

            self.send(identify).await?;
        }

        let mut sender = self.msg_sender.clone();
        let sequence_number = self.sequence_number.clone();
        let heartbeat_confirmed = self.heartbeat_confirmed.clone();

        tokio::spawn(async move {
            let rs: Result<(), SendError> = try {
                loop {
                    tokio::time::delay_for(Duration::from_millis(u64::from(payload.heartbeat_interval))).await;

                    if !heartbeat_confirmed.load(Ordering::Relaxed) {
                        warn!("Zombied connection detected, shutting down connection");
                        sender.close().await?;
                        break;
                    }

                    sender.send(Heartbeat(*sequence_number.lock().await).into()).await?;
                    heartbeat_confirmed.store(false, Ordering::Relaxed);

                    trace!("Successfully sent heartbeat");
                }
            };

            if let Err(err) = rs {
                error!("Heartbeat thread failed ({}), shutting down connection", err.to_string());

                if let Err(err) = sender.close().await {
                    error!("Failed to close channel: {}", err.to_string());
                }
            }
        });

        Ok(())
    }

    async fn on_ready(&mut self, payload: ReadyDispatch) -> Result<(), Error> {
        self.bot = Some(Arc::new(payload.user.clone()));
        self.session_id.replace(payload.session_id.clone());

        let mut context = Context::new(&self);

        for listener in &mut self.config.listeners.trait_listeners {
            if let Err(error) = (*listener).on_ready(&mut context, &payload).await {
                error!("Listener on_ready failed with: {}", error);
            }
        }

        for listener in &*self.config.listeners.ready {
            if let Err(error) = (*listener)(&mut context, &payload).await {
                error!("Listener to on_ready failed with: {}", error);
            }
        }

        for listener in &mut *self.config.listeners.stateful_listeners {
            (*listener).on_ready(&mut context, &payload).await
        }

        self.config.listeners.register(context.new_listeners);

        let shard_id = self.config.shard_id.unwrap();

        if shard_id == 0 {
            info!("You can invite the bot in your guild using this link: {}", self.invite_bot(8));
        }

        info!("Established connection for shard {}", self.config.shard_id.unwrap());

        Ok(())
    }

    async fn on_resumed(&mut self, _payload: ResumedDispatch) -> Result<(), Error> {
        info!("Successfully resumed session");
        Ok(())
    }

    async fn on_reconnect(&mut self) -> Result<(), Error> {
        warn!("Received reconnect payload, disconnecting");
        self.msg_sender.close().await?;

        Ok(())
    }

    async fn on_invalid_session(&mut self, payload: InvalidSession) -> Result<(), Error> {
        if !payload.0 {
            self.session_id = None;

            warn!("Invalid session, shutting down connection");
            self.msg_sender.close().await?;
        }

        Ok(())
    }

    async fn on_heartbeat_ack(&mut self) -> Result<(), Error> {
        self.heartbeat_confirmed.store(true, Ordering::Relaxed);

        trace!("Received heartbeat acknowledgement");
        Ok(())
    }
}
