mod models;

pub use models::*;

use crate::{map, Error, Listener};
use crate::http::HttpAPI;
use crate::encode::json;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;
use url::Url;
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
    ($name:ident: $type:ty) => {
        async fn $name(&self, payload: $type) -> Result<(), Error> {
            for listener in &mut *self.session.listeners.lock().await {
                (*listener).$name(&self.session, &payload).await?
            }

            Ok(())
        }
    }
}

struct Delayer {
    delay: usize
}

impl Delayer {
    const DELAYS: [u64; 10] = [5, 5, 5, 15, 30, 60, 120, 120, 300, 600];

    pub fn new() -> Delayer {
        Delayer {
            delay: 0
        }
    }

    pub fn delay(&mut self) {
        thread::sleep(Duration::from_secs(Delayer::DELAYS[self.delay]));

        if self.delay < 9 {
            self.delay += 1;
        }
    }

    pub fn reset(&mut self) {
        self.delay = 0
    }
}

pub struct Session {
    sender: UnboundedSender<tungstenite::Message>,
    http: HttpAPI,
    bot: Option<User>,
    listeners: Arc<Mutex<Vec<Box<dyn Listener + Send>>>>,
}

impl Session {
    #[inline]
    pub async fn send<M: Into<tungstenite::Message>>(&mut self, msg: M) -> Result<(), Error> {
        Ok(self.sender.send(msg.into()).await?)
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
pub struct GatewayAPI {
    session: Session,
    session_id: Rc<RefCell<Option<String>>>,
    sequence_number: Arc<Mutex<Option<i32>>>,
    heartbeat_confirmed: Arc<AtomicBool>,
}

impl GatewayAPI {
    /// Establishes a connection to Discord's
    /// gateway and calls the provided listeners
    /// when receiving an event.
    pub async fn connect(http: HttpAPI, listeners: Vec<Box<dyn Listener + Send>>) {
        let listeners = Arc::new(Mutex::new(listeners));

        let mut delayer = Delayer::new();
        let session_id = Rc::new(RefCell::new(None));
        let sequence_number = Arc::new(Mutex::new(None));

        loop {
            let execution: Result<(), Error> = try {
                let gateway_bot = http.gateway_bot().await?;

                let (tx, rx) = mpsc::unbounded();
                let (socket, _) = tktungstenite::connect_async(Url::parse(&gateway_bot.url).unwrap()).await?;

                let mut gateway = GatewayAPI {
                    session: Session {
                        sender: tx,
                        http: http.clone(),
                        bot: None,
                        listeners: listeners.clone(),
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

    /*async fn connect(&mut self) -> Result<(), Error> {
        while let Some(message) = self.session.socket.next().await {
            if self.heartbeat.is_some() {
                break;
            }

            self.on_message(message?).await;
        }

        if let Some(mut rx) = self.heartbeat.take() {
            let mut select = stream::select(
                self.session.socket.map(|m| Direction::Receive(m)),
                rx.map(|m| Direction::Send(m)),
            );

            let test = select.get_mut().0;

            while let Some(message) = select.next().await {
                match message {
                    Direction::Receive(message) => self.on_message(message?).await,
                    Direction::Send(message) => self.session.send(message).await?
                }
            }
        }

        self.heartbeat.as_mut()
            .expect("No heartbeat sender")
            .close();

        self.session.socket.close(None).await?;

        Ok(())
    }*/

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
            MessageCreateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageCreateDispatch> => self.on_message_create),
            MessageUpdateDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageUpdateDispatch> => self.on_message_update),
            MessageDeleteDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageDeleteDispatch> => self.on_message_delete),
            MessageDeleteBulkDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageDeleteBulkDispatch> => self.on_message_delete_bulk),
            MessageReactionAddDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageReactionAddDispatch> => self.on_reaction_add),
            MessageReactionRemoveDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageReactionRemoveDispatch> => self.on_reaction_remove),
            MessageReactionRemoveAllDispatch::EVENT_NAME => call_dispatcher!(data as Payload<MessageReactionRemoveAllDispatch> => self.on_reaction_remove_all),
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

    dispatcher!(on_channel_create: ChannelCreateDispatch);
    dispatcher!(on_channel_update: ChannelUpdateDispatch);
    dispatcher!(on_channel_delete: ChannelDeleteDispatch);
    dispatcher!(on_channel_pins_update: ChannelPinsUpdateDispatch);
    dispatcher!(on_guild_create: GuildCreateDispatch);
    dispatcher!(on_guild_update: GuildUpdateDispatch);
    dispatcher!(on_guild_delete: GuildDeleteDispatch);
    dispatcher!(on_guild_ban_add: GuildBanAddDispatch);
    dispatcher!(on_guild_ban_remove: GuildBanRemoveDispatch);
    dispatcher!(on_guild_emojis_update: GuildEmojisUpdateDispatch);
    dispatcher!(on_guild_integrations_update: GuildIntegrationsUpdateDispatch);
    dispatcher!(on_guild_member_add: GuildMemberAddDispatch);
    dispatcher!(on_guild_member_remove: GuildMemberRemoveDispatch);
    dispatcher!(on_guild_member_update: GuildMemberUpdateDispatch);
    dispatcher!(on_guild_members_chunk: GuildMembersChunkDispatch);
    dispatcher!(on_guild_role_create: GuildRoleCreateDispatch);
    dispatcher!(on_guild_role_update: GuildRoleUpdateDispatch);
    dispatcher!(on_guild_role_delete: GuildRoleDeleteDispatch);
    dispatcher!(on_message_create: MessageCreateDispatch);
    dispatcher!(on_message_update: MessageUpdateDispatch);
    dispatcher!(on_message_delete: MessageDeleteDispatch);
    dispatcher!(on_message_delete_bulk: MessageDeleteBulkDispatch);
    dispatcher!(on_reaction_add: MessageReactionAddDispatch);
    dispatcher!(on_reaction_remove: MessageReactionRemoveDispatch);
    dispatcher!(on_reaction_remove_all: MessageReactionRemoveAllDispatch);
    dispatcher!(on_presence_update: PresenceUpdateDispatch);
    dispatcher!(on_typing_start: TypingStartDispatch);
    dispatcher!(on_user_update: UserUpdateDispatch);
    dispatcher!(on_voice_state_update: VoiceStateUpdateDispatch);
    dispatcher!(on_voice_server_update: VoiceServerUpdateDispatch);
    dispatcher!(on_webhooks_update: WebhooksUpdateDispatch);

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

                    info!("Successfully sent heartbeat");
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

        info!("Received heartbeat acknowledgement");
        Ok(())
    }
}
