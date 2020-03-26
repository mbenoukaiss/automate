mod models;

pub use models::*;

use crate::{map, Error, ListenerStorage, Listener, ListenerQueue};
use crate::http::HttpAPI;
use crate::events;
use crate::encode::json;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;
use futures::{SinkExt, StreamExt};
use futures::lock::Mutex;
use futures::stream;
use futures::channel::mpsc;
use futures::channel::mpsc::SendError;
use futures::channel::mpsc::UnboundedSender;

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
        async fn $fn_name(&self, payload: $type) -> Result<(), Error> {
            for listener in &mut *self.session.listeners.lock().await.trait_listeners {
                if let Err(error) = (*listener).$fn_name(&self.session, &payload).await {
                    error!("Listener to {} failed with: {}", stringify!($name), error);
                }
            }

            for listener in &mut *self.session.listeners.lock().await.$name {
                if let Err(error) = (*listener)(&self.session, &payload).await {
                    error!("Listener to {} failed with: {}", stringify!($name), error);
                }
            }

            self.session.new_listeners.lock().await.merge(self.session.listeners.clone()).await;

            Ok(())
        }
    }
}

struct Delayer {
    delay: usize
}

impl Delayer {
    const DELAYS: [u64; 10] = [5, 5, 5, 15, 30, 60, 120, 120, 300, 600];

    fn new() -> Delayer {
        Delayer {
            delay: 0
        }
    }

    fn delay(&mut self) {
        thread::sleep(Duration::from_secs(Delayer::DELAYS[self.delay]));

        if self.delay < 9 {
            self.delay += 1;
        }
    }

    fn reset(&mut self) {
        self.delay = 0
    }
}

/// Contains data about the current gateway session.
/// Provides a way to interact with Discord HTTP API
/// by differencing to [HttpAPI](automate::http::HttpAPI).
pub struct Session {
    sender: UnboundedSender<tungstenite::Message>,
    http: HttpAPI,
    bot: Option<User>,
    listeners: Arc<Mutex<ListenerStorage>>,
    new_listeners: Mutex<ListenerQueue>,
}

impl Session {
    #[inline]
    pub async fn send<M: Into<tungstenite::Message>>(&mut self, msg: M) -> Result<(), Error> {
        Ok(self.sender.send(msg.into()).await?)
    }

    /// Registers a struct event listener that implements
    /// the [Listener](automate::Listener) trait.
    pub async fn add_listener<L: Listener + Send + 'static>(&self, listener: L) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.trait_listeners.push(Box::new(listener));
    }

    /// Registers an event that listens to [Ready](automate::gateway::ReadyDispatch) events
    pub async fn on_ready(&self, listener: events::Ready) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.ready.push(listener);
    }

    /// Registers an event that listens to [ChannelCreate](automate::gateway::ChannelCreateDispatch) events
    pub async fn on_channel_create(&self, listener: events::ChannelCreate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.channel_create.push(listener);
    }

    /// Registers an event that listens to [ChannelUpdate](automate::gateway::ChannelUpdateDispatch) events
    pub async fn on_channel_update(&self, listener: events::ChannelUpdate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.channel_update.push(listener);
    }

    /// Registers an event that listens to [ChannelDelete](automate::gateway::ChannelDeleteDispatch) events
    pub async fn on_channel_delete(&self, listener: events::ChannelDelete) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.channel_delete.push(listener);
    }

    /// Registers an event that listens to [ChannelPinsUpdate](automate::gateway::ChannelPinsUpdateDispatch) events
    pub async fn on_channel_pins_update(&self, listener: events::ChannelPinsUpdate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.channel_pins_update.push(listener);
    }

    /// Registers an event that listens to [GuildCreate](automate::gateway::GuildCreateDispatch) events
    pub async fn on_guild_create(&self, listener: events::GuildCreate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.guild_create.push(listener);
    }

    /// Registers an event that listens to [GuildUpdate](automate::gateway::GuildUpdateDispatch) events
    pub async fn on_guild_update(&self, listener: events::GuildUpdate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.guild_update.push(listener);
    }

    /// Registers an event that listens to [GuildDelete](automate::gateway::GuildDeleteDispatch) events
    pub async fn on_guild_delete(&self, listener: events::GuildDelete) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.guild_delete.push(listener);
    }

    /// Registers an event that listens to [GuildBanAdd](automate::gateway::GuildBanAddDispatch) events
    pub async fn on_guild_ban_add(&self, listener: events::GuildBanAdd) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.guild_ban_add.push(listener);
    }

    /// Registers an event that listens to [GuildBanRemove](automate::gateway::GuildBanRemoveDispatch) events
    pub async fn on_guild_ban_remove(&self, listener: events::GuildBanRemove) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.guild_ban_remove.push(listener);
    }

    /// Registers an event that listens to [GuildEmojisUpdate](automate::gateway::GuildEmojisUpdateDispatch) events
    pub async fn on_guild_emojis_update(&self, listener: events::GuildEmojisUpdate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.guild_emojis_update.push(listener);
    }

    /// Registers an event that listens to [GuildIntegrationsUpdate](automate::gateway::GuildIntegrationsUpdateDispatch) events
    pub async fn on_guild_integrations_update(&self, listener: events::GuildIntegrationsUpdate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.guild_integrations_update.push(listener);
    }

    /// Registers an event that listens to [GuildMemberAdd](automate::gateway::GuildMemberAddDispatch) events
    pub async fn on_guild_member_add(&self, listener: events::GuildMemberAdd) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.guild_member_add.push(listener);
    }

    /// Registers an event that listens to [GuildMemberRemove](automate::gateway::GuildMemberRemoveDispatch) events
    pub async fn on_guild_member_remove(&self, listener: events::GuildMemberRemove) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.guild_member_remove.push(listener);
    }

    /// Registers an event that listens to [GuildMemberUpdate](automate::gateway::GuildMemberUpdateDispatch) events
    pub async fn on_guild_member_update(&self, listener: events::GuildMemberUpdate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.guild_member_update.push(listener);
    }

    /// Registers an event that listens to [GuildMembersChunk](automate::gateway::GuildMembersChunkDispatch) events
    pub async fn on_guild_members_chunk(&self, listener: events::GuildMembersChunk) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.guild_members_chunk.push(listener);
    }

    /// Registers an event that listens to [GuildRoleCreate](automate::gateway::GuildRoleCreateDispatch) events
    pub async fn on_guild_role_create(&self, listener: events::GuildRoleCreate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.guild_role_create.push(listener);
    }

    /// Registers an event that listens to [GuildRoleUpdate](automate::gateway::GuildRoleUpdateDispatch) events
    pub async fn on_guild_role_update(&self, listener: events::GuildRoleUpdate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.guild_role_update.push(listener);
    }

    /// Registers an event that listens to [GuildRoleDelete](automate::gateway::GuildRoleDeleteDispatch) events
    pub async fn on_guild_role_delete(&self, listener: events::GuildRoleDelete) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.guild_role_delete.push(listener);
    }

    /// Registers an event that listens to [InviteCreate](automate::gateway::InviteCreateDispatch) events
    pub async fn on_invite_create(&self, listener: events::InviteCreate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.invite_create.push(listener);
    }

    /// Registers an event that listens to [InviteDelete](automate::gateway::InviteDeleteDispatch) events
    pub async fn on_invite_delete(&self, listener: events::InviteDelete) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.invite_delete.push(listener);
    }

    /// Registers an event that listens to [MessageCreate](automate::gateway::MessageCreateDispatch) events
    pub async fn on_message_create(&self, listener: events::MessageCreate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.message_create.push(listener);
    }

    /// Registers an event that listens to [MessageUpdate](automate::gateway::MessageUpdateDispatch) events
    pub async fn on_message_update(&self, listener: events::MessageUpdate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.message_update.push(listener);
    }

    /// Registers an event that listens to [MessageDelete](automate::gateway::MessageDeleteDispatch) events
    pub async fn on_message_delete(&self, listener: events::MessageDelete) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.message_delete.push(listener);
    }

    /// Registers an event that listens to [MessageDeleteBulk](automate::gateway::MessageDeleteBulkDispatch) events
    pub async fn on_message_delete_bulk(&self, listener: events::MessageDeleteBulk) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.message_delete_bulk.push(listener);
    }

    /// Registers an event that listens to [MessageReactionAdd](automate::gateway::MessageReactionAddDispatch) events
    pub async fn on_reaction_add(&self, listener: events::MessageReactionAdd) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.reaction_add.push(listener);
    }

    /// Registers an event that listens to [MessageReactionRemove](automate::gateway::MessageReactionRemoveDispatch) events
    pub async fn on_reaction_remove(&self, listener: events::MessageReactionRemove) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.reaction_remove.push(listener);
    }

    /// Registers an event that listens to [MessageReactionRemoveAll](automate::gateway::MessageReactionRemoveAllDispatch) events
    pub async fn on_reaction_remove_all(&self, listener: events::MessageReactionRemoveAll) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.reaction_remove_all.push(listener);
    }

    /// Registers an event that listens to [PresenceUpdate](automate::gateway::PresenceUpdateDispatch) events
    pub async fn on_presence_update(&self, listener: events::PresenceUpdate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.presence_update.push(listener);
    }

    /// Registers an event that listens to [TypingStart](automate::gateway::TypingStartDispatch) events
    pub async fn on_typing_start(&self, listener: events::TypingStart) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.typing_start.push(listener);
    }

    /// Registers an event that listens to [UserUpdate](automate::gateway::UserUpdateDispatch) events
    pub async fn on_user_update(&self, listener: events::UserUpdate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.user_update.push(listener);
    }

    /// Registers an event that listens to [VoiceStateUpdate](automate::gateway::VoiceStateUpdateDispatch) events
    pub async fn on_voice_state_update(&self, listener: events::VoiceStateUpdate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.voice_state_update.push(listener);
    }

    /// Registers an event that listens to [VoiceServerUpdate](automate::gateway::VoiceServerUpdateDispatch) events
    pub async fn on_voice_server_update(&self, listener: events::VoiceServerUpdate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.voice_server_update.push(listener);
    }

    /// Registers an event that listens to [WebhooksUpdate](automate::gateway::WebhooksUpdateDispatch) events
    pub async fn on_webhooks_update(&self, listener: events::WebhooksUpdate) {
        let mut queue = self.new_listeners.lock().await;

queue.updated = true;
queue.webhooks_update.push(listener);
    }

    #[inline]
    pub fn bot(&self) -> &User {
        self.bot.as_ref().unwrap()
    }

    #[inline]
    pub fn invite_bot(&self, permission: u32) -> String {
        format!("https://discordapp.com/oauth2/authorize?client_id={}&scope=bot&permissions={}", self.bot().id, permission)
    }
}

impl Deref for Session {
    type Target = HttpAPI;

    fn deref(&self) -> &Self::Target {
        &self.http
    }
}

#[derive(Debug)]
enum Direction {
    Receive(Result<tungstenite::Message, tungstenite::Error>),
    Send(tungstenite::Message),
}

/// Communicates with Discord's gateway
pub(crate) struct GatewayAPI {
    session: Session,
    session_id: Rc<RefCell<Option<String>>>,
    sequence_number: Arc<Mutex<Option<i32>>>,
    heartbeat_confirmed: Arc<AtomicBool>,
}

impl GatewayAPI {
    /// Establishes a connection to Discord's
    /// gateway and calls the provided listeners
    /// when receiving an event.
    pub(crate) async fn connect(http: HttpAPI, listeners: ListenerStorage) {
        let listeners = Arc::new(Mutex::new(listeners));

        let mut delayer = Delayer::new();
        let session_id = Rc::new(RefCell::new(None));
        let sequence_number = Arc::new(Mutex::new(None));

        loop {
            let execution: Result<(), Error> = try {
                let gateway_bot = http.gateway_bot().await?;

                let (tx, rx) = mpsc::unbounded();
                let (socket, _) = tktungstenite::connect_async(&gateway_bot.url).await?;

                let mut gateway = GatewayAPI {
                    session: Session {
                        sender: tx,
                        http: http.clone(),
                        bot: None,
                        listeners: listeners.clone(),
                        new_listeners: Mutex::default()
                    },
                    session_id: Rc::clone(&session_id),
                    sequence_number: Arc::clone(&sequence_number),
                    heartbeat_confirmed: Arc::new(AtomicBool::new(true)),
                };

                let mut select = stream::select(
                    socket.map(Direction::Receive),
                    rx.map(Direction::Send),
                );

                while let Some(message) = select.next().await {
                    match message {
                        Direction::Receive(message) => gateway.on_message(message?).await,
                        Direction::Send(message) => select.get_mut().0.send(message).await?
                    }
                }

                select.get_mut().0.close().await?;

                delayer.reset();
            };

            if let Err(err) = execution {
                error!("Connection was interrupted with '{}'", err.msg);
            } else {
                error!("Connection interrupted");
            }

            delayer.delay();

            if let Some(sid) = &*session_id.borrow() {
                info!("Attempting to resume session {} with sequence_number: {}", sid, sequence_number.lock().await.unwrap());
            } else {
                info!("Attempting to reconnect");
            }
        }
    }

    async fn on_message(&mut self, msg: tungstenite::Message) {
        if let tungstenite::Message::Text(data) = msg {
            if let Err(err) = self.dispatch_payload(&data).await {
                error!("An error occurred while reading message: {}\n{}", err.msg, data);
            }
        } else {
            error!("Unknown message type received");
        }
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
        if let Some(sid) = &*self.session_id.borrow() {
            let resume = Resume {
                token: self.session.http.token().clone(),
                session_id: sid.to_owned(),
                seq: self.sequence_number.lock().await.unwrap(),
            };

            self.session.send(resume).await?;
            info!("Requested to resume session");
        } else {
            let identify = Identify {
                token: self.session.http.token().clone(),
                properties: map! {
                    "$os" => "linux",
                    "$browser" => "automate",
                    "$device" => "automate"
                },
                compress: None,
                large_threshold: None,
                shard: None,
                presence: None,
                guild_subscriptions: Some(true),
            };

            self.session.send(identify).await?;
        }

        let mut sender = self.session.sender.clone();
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
        for listener in &mut *self.session.listeners.lock().await.trait_listeners {
            if let Err(error) = (*listener).on_ready(&self.session, &payload).await {
                error!("Listener on_ready failed with: {}", error);
            }
        }

        for listener in &mut *self.session.listeners.lock().await.ready {
            if let Err(error) = (*listener)(&self.session, &payload).await {
                error!("Listener to on_ready failed with: {}", error);
            }
        }

        self.session.bot = Some(payload.user);
        *self.session_id.borrow_mut() = Some(payload.session_id);

        info!("Successfully established connection with Discord. Invite the bot in your guild using this link {}", self.session.invite_bot(8));

        Ok(())
    }

    async fn on_resumed(&mut self, _payload: ResumedDispatch) -> Result<(), Error> {
        info!("Successfully resumed session");
        Ok(())
    }

    async fn on_reconnect(&mut self) -> Result<(), Error> {
        warn!("Received reconnect payload, disconnecting");
        self.session.sender.close().await?;

        Ok(())
    }

    async fn on_invalid_session(&mut self, payload: InvalidSession) -> Result<(), Error> {
        if !payload.0 {
            *self.session_id.borrow_mut() = None;

            warn!("Invalid session, shutting down connection");
            self.session.sender.close().await?;
        }

        Ok(())
    }

    async fn on_heartbeat_ack(&mut self) -> Result<(), Error> {
        self.heartbeat_confirmed.store(true, Ordering::Relaxed);

        trace!("Received heartbeat acknowledgement");
        Ok(())
    }
}
