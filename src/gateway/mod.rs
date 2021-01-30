//! Tools to interact with Discord's gateway API

pub mod models;

pub use models::*;

use crate::{map, Error, Configuration, logger, http};
use crate::http::HttpAPI;
use crate::encode::json;
use std::env;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::ops::Deref;
use futures::{stream, future, SinkExt, StreamExt};
use futures::lock::Mutex;
use futures::channel::mpsc;
use futures::channel::mpsc::{SendError, UnboundedSender};
use tktungstenite::tungstenite::Message as TkMessage;
use chrono::{NaiveDateTime, Utc, Duration as ChronoDuration};

#[cfg(feature = "storage")]
use crate::storage::{StorageContainer, Stored};
#[cfg(feature = "storage")]
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};

macro_rules! call_dispatcher {
    ($data:ident as $payload:ty => $self:ident.$method:ident) => {{
        let payload: $payload = serde_json::from_str(&$data)?;

        if let Some(val) = payload.s {
            *$self.sequence_number.lock().await = Some(val);
        }

        $self.$method(payload.d).await?
    }};
}

macro_rules! dispatcher {
    ($fn_name:ident: $type:ty => $name:ident) => {
        async fn $fn_name(&mut self, payload: $type) -> Result<(), Error> {
            #[cfg(feature = "storage")]
            self.config.storages.$fn_name(&payload).await;

            let context = Context {
                sender: &self.msg_sender,
                #[cfg(feature = "storage")]
                storage: &self.config.storages,
                http: &self.http,
                bot: self.bot.as_ref().unwrap()
            };

            let stateless = self.config.listeners.$name.iter()
                .map(|l| (*l)(&context, &payload));

            let stateful = self.config.listeners.stateful_listeners.iter_mut()
                .map(|l| (*l).$fn_name(&context, &payload));

            future::join_all(stateless.chain(stateful)).await
                .into_iter()
                .filter_map(|r| r.err())
                .for_each(|err| error!("Listener to `{}` failed with: {}", stringify!($name), err));

            Ok(())
        }
    }
}

/// Helps avoid spamming connections to Discord
/// in case the bots is constantly getting
/// disconnected by the gateway API since the bot
/// is only allowed 1000 connections per day or
/// a bit more than 41 per hour.
struct Delayer {
    attempts: u32
}

impl Delayer {
    fn new() -> Delayer {
        Delayer {
            attempts: 0
        }
    }

    /// Delays based on the amount of previous failed
    /// connection attempts.
    async fn delay(&self, session_id: &Option<String>) {
        let delay = match self.attempts {
            0 => 0,
            1 => 5,
            2 | 3 => 30,
            4 | 5 => 60,
            6..=10 => 120,
            _ => 600
        };

        if let Some(sid) = session_id.as_ref() {
            info!("Attempting to resume session {} in {} seconds", sid, delay);
        } else {
            info!("Attempting to reconnect in {} seconds", delay);
        }

        tokio::time::sleep(Duration::from_millis(delay * 1000)).await
    }

    /// Call this function when the previous connection
    /// returned an error or was judged invalid.
    fn failure(&mut self) {
        self.attempts += 1;
    }

    /// Call this when the previous connection was disconnected
    /// in order to reconnect.
    fn reset(&mut self) {
        self.attempts = 0;
    }
}

/// Context about the current gateway session.
/// Provides a way to interact with Discord HTTP API
/// by dereferencing to [HttpAPI](automate::http::HttpAPI).
pub struct Context<'a> {
    sender: &'a UnboundedSender<Instruction>,
    #[cfg(feature = "storage")]
    storage: &'a StorageContainer,
    http: &'a HttpAPI,
    pub bot: &'a User,
}

impl<'a> Context<'a> {
    /// Sends a command to the gateway.
    /// The message must be a valid payload.
    #[inline]
    async fn send_command<M: Into<TkMessage>>(&self, msg: M) -> Result<(), Error> {
        Ok(self.sender.unbounded_send(Instruction::Send(msg.into(), false))?)
    }

    /// Indicate a presence or status update.
    #[inline]
    pub async fn update_status(&self, data: UpdateStatus) -> Result<(), Error> {
        self.send_command(data).await
    }

    /// Join, move or disconnect from a voice channel.
    #[inline]
    pub async fn update_voice_state(&self, data: UpdateVoiceState) -> Result<(), Error> {
        self.send_command(data).await
    }

    /// Request members of a guild.
    #[inline]
    pub async fn request_guild_members(&self, data: RequestGuildMembers) -> Result<(), Error> {
        self.send_command(data).await
    }

    /// Read only reference to the storage of the
    /// specified type.
    #[inline]
    #[cfg(feature = "storage")]
    pub async fn storage<T: Stored + 'static>(&self) -> RwLockReadGuard<'_, T::Storage> {
        self.storage.read::<T>().await
    }

    /// Writable reference to the storage of the specified
    /// type. Getting a writable version of
    /// [GuildStorage](automate::storage::GuildStorage),
    /// [ChannelStorage](automate::storage::ChannelStorage) or
    /// [UserStorage](automate::storage::UserStorage)
    /// is useless since they are not mutable
    #[inline]
    #[cfg(feature = "storage")]
    pub async fn storage_mut<T: Stored + 'static>(&self) -> RwLockWriteGuard<'_, T::Storage> {
        self.storage.write::<T>().await
    }

    /// Creates a link to invite the bot to a discord server
    /// and give him the specified permissions.
    #[inline]
    pub fn invite_bot(&self, permission: u32) -> String {
        format!("https://discordapp.com/oauth2/authorize?client_id={}&scope=bot&permissions={}", self.bot.id, permission)
    }
}

impl<'a> Deref for Context<'a> {
    type Target = HttpAPI;

    #[inline]
    fn deref(&self) -> &HttpAPI {
        &self.http
    }
}

#[derive(Debug)]
enum Instruction {
    /// Receive a message sent by the gateway
    Receive(Result<TkMessage, tktungstenite::tungstenite::Error>),
    /// The message and whether it is a necessary message or not.
    /// Necessary messages have reserved slots to keep sending
    /// them even when it's about to reach rate-limit since they
    /// are necessary to keep the gateway connection alive.
    Send(TkMessage, bool),
    /// Close the connection with the gateway
    Close,
}

/// Communicates with Discord's gateway
pub(crate) struct GatewayAPI<'a> {
    config: &'a mut Configuration,
    session_id: Option<String>,
    msg_sender: UnboundedSender<Instruction>,
    http: &'a HttpAPI,
    bot: Option<User>,

    sequence_number: Arc<Mutex<Option<i32>>>,
    heartbeat_confirmed: Arc<AtomicBool>,
}

impl<'a> GatewayAPI<'a> {
    /// Establishes a connection to Discord's
    /// gateway and calls the provided listeners
    /// when receiving an event.
    pub(crate) async fn connect(mut config: Configuration, url: String) -> ! {
        let mut delayer = Delayer::new();

        let http = HttpAPI::new(&config.token);
        let sequence_number = Arc::new(Mutex::new(None));
        let mut session_id = None;

        loop {
            let execution: Result<(), Error> = try {
                let (tx, rx) = mpsc::unbounded();
                let (socket, _) = tktungstenite::connect_async(&url).await?;

                let mut remaining_commands: Option<(i32, NaiveDateTime)> = None;

                let mut gateway = GatewayAPI {
                    config: &mut config,
                    session_id: None,
                    msg_sender: tx,
                    http: &http,
                    bot: None,
                    sequence_number: Arc::clone(&sequence_number),
                    heartbeat_confirmed: Arc::new(AtomicBool::new(true)),
                };

                let mut select = stream::select(
                    socket.map(Instruction::Receive), //gateway events
                    rx,                                  //commands and close
                );

                while let Some(message) = select.next().await {
                    match message {
                        Instruction::Receive(m) => gateway.on_message(m?).await?,
                        Instruction::Send(m, n) => if check_remaining(&mut remaining_commands, n).await {
                            select.get_mut().0.send(m).await?;
                        },
                        Instruction::Close => break
                    }
                }

                select.get_mut().0.close().await?;
                select.get_mut().1.close();

                session_id = gateway.session_id;
            };

            // if there was an error, there's probably a problem with the bot and it should
            // therefore not try to reconnect immediately. if the session_id is empty, either
            // the bot didn't make it to the end of the identify or it received an invalid session
            // in both cases it should try to delay because the bot is probably doing something
            // wrong
            if execution.is_err() || session_id.is_none() {
                delayer.failure();
            } else {
                delayer.reset(); //else everything went correctly and it's probably just a reconnect
            }

            if let Err(err) = execution {
                error!("Connection was interrupted: {}", err.to_string());
            }

            delayer.delay(&session_id).await
        }
    }

    /// Sends a command to the gateway.
    #[inline]
    async fn send_command<M: Into<TkMessage>>(&mut self, msg: M, necessary: bool) -> Result<(), Error> {
        Ok(self.msg_sender.send(Instruction::Send(msg.into(), necessary)).await?)
    }

    /// Shuts down the connection with the gateway.
    #[inline]
    async fn disconnect(&mut self) -> Result<(), Error> {
        self.msg_sender.send(Instruction::Close).await?;
        self.msg_sender.close().await?;

        Ok(())
    }

    async fn on_message(&mut self, msg: TkMessage) -> Result<(), Error> {
        match msg {
            TkMessage::Text(data) => {
                if let Err(err) = self.dispatch_payload(&data).await {
                    error!("An error occurred while reading message: {}", err.to_string());
                }
            }
            TkMessage::Close(close) => {
                return if let Some(cf) = close {
                    Error::gateway(format!("Gateway unexpectedly closed with code {}: {}", Into::<u16>::into(cf.code), cf.reason))
                } else {
                    Error::gateway("Gateway unexpectedly closed")
                };
            }
            unknown => trace!("Unknown message type received: {:?}", unknown)
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
    async fn dispatch_event(&mut self, data: &str) -> Result<(), Error> {
        let event_name = json::root_search::<String>("t", data)?;
        trace!("Received gateway event `{}`: {}", event_name, data);

        match event_name.as_str() {
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
            unknown_event => return Error::gateway(format!("Unknown event {}", unknown_event))
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

            self.send_command(resume, true).await?;
            info!("Requested to resume session");
        } else {
            let identify = Identify {
                token: self.http.token().clone(),
                properties: map! {
                    "$os" => env::consts::OS,
                    "$browser" => "automate",
                    "$device" => "automate"
                },
                compress: false,
                shard: [self.config.shard_id.unwrap(), self.config.total_shards.unwrap()],
                large_threshold: self.config.member_threshold,
                presence: self.config.presence.clone(),
                guild_subscriptions: self.config.guild_subscriptions,
                intents: self.config.intents,
            };

            self.send_command(identify, true).await?;
        }

        let sender: UnboundedSender<Instruction> = self.msg_sender.clone();
        let sequence_number = self.sequence_number.clone();
        let heartbeat_confirmed = self.heartbeat_confirmed.clone();
        let shard_id = self.config.shard_id.clone().unwrap();

        tokio::spawn(logger::setup_for_task(format!("hearbeat-{}", shard_id), async move {
            heartbeat_task(sender, sequence_number, payload.heartbeat_interval as u64, heartbeat_confirmed).await;
        }));

        let interval = self.config.collector_period;

        tokio::spawn(logger::setup_for_task(format!("collector-{}", shard_id), async move {
            bucket_collector_task(interval).await;
        }));

        Ok(())
    }

    async fn on_ready(&mut self, payload: ReadyDispatch) -> Result<(), Error> {
        let shard_id = self.config.shard_id.unwrap();
        if shard_id == 0 && self.bot.is_none() {
            let i = format!("https://discordapp.com/oauth2/authorize?client_id={}&scope=bot&permissions=8", payload.user.id);
            info!("You can invite the bot in your guild using this link: {}", i);
        }

        info!("Established connection for shard {}", shard_id);

        self.bot = Some(payload.user.clone());
        self.session_id.replace(payload.session_id.clone());

        #[cfg(feature = "storage")]
            self.config.storages.on_ready(&payload).await;

        let context = Context {
            sender: &self.msg_sender,
            #[cfg(feature = "storage")]
            storage: &self.config.storages,
            http: &self.http,
            bot: &payload.user,
        };

        let stateless = self.config.listeners.ready.iter()
            .map(|l| (*l)(&context, &payload));

        let stateful = self.config.listeners.stateful_listeners.iter_mut()
            .map(|l| (*l).on_ready(&context, &payload));

        future::join_all(stateless.chain(stateful)).await
            .into_iter()
            .filter_map(|r| r.err())
            .for_each(|err| error!("Listener to `ready` failed with: {}", err));

        Ok(())
    }

    async fn on_resumed(&mut self, _payload: ResumedDispatch) -> Result<(), Error> {
        info!("Successfully resumed session");
        Ok(())
    }

    async fn on_reconnect(&mut self) -> Result<(), Error> {
        info!("Received reconnect payload, disconnecting");
        self.disconnect().await?;

        Ok(())
    }

    async fn on_invalid_session(&mut self, payload: InvalidSession) -> Result<(), Error> {
        if !payload.0 {
            self.session_id = None;

            warn!("Invalid session, shutting down connection");
            self.disconnect().await?;
        }

        Ok(())
    }

    async fn on_heartbeat_ack(&mut self) -> Result<(), Error> {
        self.heartbeat_confirmed.store(true, Ordering::Relaxed);

        trace!("Received heartbeat acknowledgement");
        Ok(())
    }
}

async fn check_remaining(remaining_commands: &mut Option<(i32, NaiveDateTime)>, necessary: bool) -> bool {
    if remaining_commands.is_none() {
        let until: NaiveDateTime = Utc::now().naive_utc() + ChronoDuration::minutes(1);

        *remaining_commands = Some((120, until));
    }

    let mut remaining_commands = remaining_commands.as_mut().unwrap();

    //if now is after the given time, reset the rate-limit
    if ::chrono::Utc::now().naive_utc() > remaining_commands.1 {
        remaining_commands.0 = 120;
        remaining_commands.1 += ChronoDuration::minutes(1);
    }

    if necessary || remaining_commands.0 > 5 {
        remaining_commands.0 -= 1;

        trace!("{} gateway command calls remaining until (reset at {} UTC)", remaining_commands.0, remaining_commands.1.format("%Y-%m-%d %H:%M:%S"));
        true
    } else {
        trace!("Reached gateway rate limit ({} calls left, reset at {} UTC)", remaining_commands.0, remaining_commands.1.format("%Y-%m-%d %H:%M:%S"));
        false
    }
}

async fn heartbeat_task(
    mut sender: UnboundedSender<Instruction>,
    sequence_number: Arc<Mutex<Option<i32>>>,
    interval: u64,
    heartbeat_confirmed: Arc<AtomicBool>,
) {
    let rs: Result<(), SendError> = try {
        loop {
            tokio::time::sleep(Duration::from_millis(interval)).await;

            //if the channel was closed, it means the shard closed and dropped the receiver
            //therefore this heartbeat task is not needed anymore and a new one will be created
            //since the channel is already closed, we directly return from the function
            if sender.is_closed() {
                trace!("Channel was closed, stopping heartbeat thread");
                return;
            }

            if !heartbeat_confirmed.load(Ordering::Relaxed) {
                warn!("Zombied connection detected, shutting down connection");
                break;
            }

            sender.send(Instruction::Send(Heartbeat(*sequence_number.lock().await).into(), true)).await?;
            heartbeat_confirmed.store(false, Ordering::Relaxed);

            trace!("Successfully sent heartbeat");
        }
    };

    if let Err(err) = rs {
        error!("Heartbeat thread failed ({}), shutting down connection", err.to_string());
    }

    let close: Result<(), SendError> = try {
        sender.send(Instruction::Close).await?;
        sender.close().await?;
    };

    if let Err(err) = close {
        error!("Failed to close channel: {}", err.to_string());
    }
}

async fn bucket_collector_task(interval: u64) {
    loop {
        tokio::time::sleep(Duration::from_millis(interval * 1000)).await;
        http::collect_outdated_buckets().await;
    }
}