#![feature(test)]
#![feature(try_blocks)]
#![feature(proc_macro_hygiene)]
#![allow(where_clauses_object_safety)] //should be fixable when async traits are allowed
#![allow(clippy::identity_op)] //because clippy forbides 1 << 0 in c-like enums

extern crate self as automate;
extern crate test;
#[macro_use]
extern crate proc_macro_hack;
#[macro_use]
extern crate automate_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate tokio_tungstenite as tktungstenite;

pub mod events;
pub mod http;
pub mod gateway;
pub mod encode;
mod snowflake;
mod macros;
mod errors;
mod logger;

pub use automate_derive::listener;
#[proc_macro_hack]
pub use automate_derive::functions;

pub use async_trait::async_trait;
pub use tokio;

pub use http::HttpAPI;
pub use gateway::Session;
pub use logger::setup_logging;
pub use snowflake::Snowflake;
pub use errors::Error;

use events::*;
use gateway::GatewayAPI;
use tokio::runtime::Runtime;

/// The struct used to register listeners, setup
/// configuration and establish connection with
/// Discord.
///
/// # Example
/// ```no_run
/// use automate::Discord;
///
/// Discord::new(&std::env::var("DISCORD_API_TOKEN").expect("API token not found"))
///     .connect_blocking()
/// ```
pub struct Discord {
    http: HttpAPI,
    listeners: ListenerStorage,
}

impl Discord {
    /// Creates an instance of this struct
    /// with the provided token.
    /// The token can be generated on
    /// [Discord's developers portal](https://discordapp.com/developers/applications/)
    pub fn new(token: &str) -> Discord {
        Discord {
            http: HttpAPI::new(token),
            listeners: ListenerStorage::default(),
        }
    }

    /// Registers an event listener struct that implements
    /// the [Listener](automate::events::Listener) trait or
    /// a listener function with the `̀#[listener]` attribute. 
    pub fn with(mut self, listener: Vec<Box<dyn ListenerMarker>>) -> Self {
        for l in listener {
            match l.downcast() {
                ListenerType::Impl(l) => self.listeners.trait_listeners.push(l),
                ListenerType::Ready(l) => self.listeners.ready.push(l),
                ListenerType::ChannelCreate(l) => self.listeners.channel_create.push(l),
                ListenerType::ChannelUpdate(l) => self.listeners.channel_update.push(l),
                ListenerType::ChannelDelete(l) => self.listeners.channel_delete.push(l),
                ListenerType::ChannelPinsUpdate(l) => self.listeners.channel_pins_update.push(l),
                ListenerType::GuildCreate(l) => self.listeners.guild_create.push(l),
                ListenerType::GuildUpdate(l) => self.listeners.guild_update.push(l),
                ListenerType::GuildDelete(l) => self.listeners.guild_delete.push(l),
                ListenerType::GuildBanAdd(l) => self.listeners.guild_ban_add.push(l),
                ListenerType::GuildBanRemove(l) => self.listeners.guild_ban_remove.push(l),
                ListenerType::GuildEmojisUpdate(l) => self.listeners.guild_emojis_update.push(l),
                ListenerType::GuildIntegrationsUpdate(l) => self.listeners.guild_integrations_update.push(l),
                ListenerType::GuildMemberAdd(l) => self.listeners.guild_member_add.push(l),
                ListenerType::GuildMemberRemove(l) => self.listeners.guild_member_remove.push(l),
                ListenerType::GuildMemberUpdate(l) => self.listeners.guild_member_update.push(l),
                ListenerType::GuildMembersChunk(l) => self.listeners.guild_members_chunk.push(l),
                ListenerType::GuildRoleCreate(l) => self.listeners.guild_role_create.push(l),
                ListenerType::GuildRoleUpdate(l) => self.listeners.guild_role_update.push(l),
                ListenerType::GuildRoleDelete(l) => self.listeners.guild_role_delete.push(l),
                ListenerType::InviteCreate(l) => self.listeners.invite_create.push(l),
                ListenerType::InviteDelete(l) => self.listeners.invite_delete.push(l),
                ListenerType::MessageCreate(l) => self.listeners.message_create.push(l),
                ListenerType::MessageUpdate(l) => self.listeners.message_update.push(l),
                ListenerType::MessageDelete(l) => self.listeners.message_delete.push(l),
                ListenerType::MessageDeleteBulk(l) => self.listeners.message_delete_bulk.push(l),
                ListenerType::MessageReactionAdd(l) => self.listeners.reaction_add.push(l),
                ListenerType::MessageReactionRemove(l) => self.listeners.reaction_remove.push(l),
                ListenerType::MessageReactionRemoveAll(l) => self.listeners.reaction_remove_all.push(l),
                ListenerType::PresenceUpdate(l) => self.listeners.presence_update.push(l),
                ListenerType::TypingStart(l) => self.listeners.typing_start.push(l),
                ListenerType::UserUpdate(l) => self.listeners.user_update.push(l),
                ListenerType::VoiceStateUpdate(l) => self.listeners.voice_state_update.push(l),
                ListenerType::VoiceServerUpdate(l) => self.listeners.voice_server_update.push(l),
                ListenerType::WebhooksUpdate(l) => self.listeners.webhooks_update.push(l),
            }
        }
        self
    }

    /// Registers an event listener
    #[deprecated(since = "0.2.1", note = "Please use `Discord::with` instead")]
    pub fn with_listener<L: Listener + Send + 'static>(mut self, listener: L) -> Self {
        self.listeners.trait_listeners.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_ready(mut self, listener: impl Ready) -> Self {
        self.listeners.ready.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_channel_create(mut self, listener: impl ChannelCreate) -> Self {
        self.listeners.channel_create.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_channel_update(mut self, listener: impl ChannelUpdate) -> Self {
        self.listeners.channel_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_channel_delete(mut self, listener: impl ChannelDelete) -> Self {
        self.listeners.channel_delete.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_channel_pins_update(mut self, listener: impl ChannelPinsUpdate) -> Self {
        self.listeners.channel_pins_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_guild_create(mut self, listener: impl GuildCreate) -> Self {
        self.listeners.guild_create.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_guild_update(mut self, listener: impl GuildUpdate) -> Self {
        self.listeners.guild_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_guild_delete(mut self, listener: impl GuildDelete) -> Self {
        self.listeners.guild_delete.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_guild_ban_add(mut self, listener: impl GuildBanAdd) -> Self {
        self.listeners.guild_ban_add.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_guild_ban_remove(mut self, listener: impl GuildBanRemove) -> Self {
        self.listeners.guild_ban_remove.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_guild_emojis_update(mut self, listener: impl GuildEmojisUpdate) -> Self {
        self.listeners.guild_emojis_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_guild_integrations_update(mut self, listener: impl GuildIntegrationsUpdate) -> Self {
        self.listeners.guild_integrations_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_guild_member_add(mut self, listener: impl GuildMemberAdd) -> Self {
        self.listeners.guild_member_add.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_guild_member_remove(mut self, listener: impl GuildMemberRemove) -> Self {
        self.listeners.guild_member_remove.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_guild_member_update(mut self, listener: impl GuildMemberUpdate) -> Self {
        self.listeners.guild_member_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_guild_members_chunk(mut self, listener: impl GuildMembersChunk) -> Self {
        self.listeners.guild_members_chunk.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_guild_role_create(mut self, listener: impl GuildRoleCreate) -> Self {
        self.listeners.guild_role_create.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_guild_role_update(mut self, listener: impl GuildRoleUpdate) -> Self {
        self.listeners.guild_role_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_guild_role_delete(mut self, listener: impl GuildRoleDelete) -> Self {
        self.listeners.guild_role_delete.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_invite_create(mut self, listener: impl InviteCreate) -> Self {
        self.listeners.invite_create.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_invite_delete(mut self, listener: impl InviteDelete) -> Self {
        self.listeners.invite_delete.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_message_create(mut self, listener: impl MessageCreate) -> Self {
        self.listeners.message_create.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_message_update(mut self, listener: impl MessageUpdate) -> Self {
        self.listeners.message_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_message_delete(mut self, listener: impl MessageDelete) -> Self {
        self.listeners.message_delete.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_message_delete_bulk(mut self, listener: impl MessageDeleteBulk) -> Self {
        self.listeners.message_delete_bulk.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_reaction_add(mut self, listener: impl MessageReactionAdd) -> Self {
        self.listeners.reaction_add.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_reaction_remove(mut self, listener: impl MessageReactionRemove) -> Self {
        self.listeners.reaction_remove.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_reaction_remove_all(mut self, listener: impl MessageReactionRemoveAll) -> Self {
        self.listeners.reaction_remove_all.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_presence_update(mut self, listener: impl PresenceUpdate) -> Self {
        self.listeners.presence_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_typing_start(mut self, listener: impl TypingStart) -> Self {
        self.listeners.typing_start.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_user_update(mut self, listener: impl UserUpdate) -> Self {
        self.listeners.user_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_voice_state_update(mut self, listener: impl VoiceStateUpdate) -> Self {
        self.listeners.voice_state_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_voice_server_update(mut self, listener: impl VoiceServerUpdate) -> Self {
        self.listeners.voice_server_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.2.2", note = "Please use `Discord::with` instead")]
    pub fn on_webhooks_update(mut self, listener: impl WebhooksUpdate) -> Self {
        self.listeners.webhooks_update.push(Box::new(listener));
        self
    }

    /// Asynchronous function setup the connection
    /// with Discord.
    /// Will block forever unless the bot crashes.
    pub async fn connect(self) {
        GatewayAPI::connect(self.http, self.listeners).await
    }

    /// Non asynchronous equivalent for the connect
    /// function to setup the connection with discord.
    /// Creates a tokio runtime.
    ///
    /// Will block forever unless the bot crashes.
    pub fn connect_blocking(self) {
        Runtime::new().unwrap().block_on(self.connect())
    }
}

#[derive(Default)]
pub(crate) struct ListenerStorage {
    pub(crate) trait_listeners: Vec<Box<dyn Listener>>,
    pub(crate) ready: Vec<Box<dyn Ready>>,
    pub(crate) channel_create: Vec<Box<dyn ChannelCreate>>,
    pub(crate) channel_update: Vec<Box<dyn ChannelUpdate>>,
    pub(crate) channel_delete: Vec<Box<dyn ChannelDelete>>,
    pub(crate) channel_pins_update: Vec<Box<dyn ChannelPinsUpdate>>,
    pub(crate) guild_create: Vec<Box<dyn GuildCreate>>,
    pub(crate) guild_update: Vec<Box<dyn GuildUpdate>>,
    pub(crate) guild_delete: Vec<Box<dyn GuildDelete>>,
    pub(crate) guild_ban_add: Vec<Box<dyn GuildBanAdd>>,
    pub(crate) guild_ban_remove: Vec<Box<dyn GuildBanRemove>>,
    pub(crate) guild_emojis_update: Vec<Box<dyn GuildEmojisUpdate>>,
    pub(crate) guild_integrations_update: Vec<Box<dyn GuildIntegrationsUpdate>>,
    pub(crate) guild_member_add: Vec<Box<dyn GuildMemberAdd>>,
    pub(crate) guild_member_remove: Vec<Box<dyn GuildMemberRemove>>,
    pub(crate) guild_member_update: Vec<Box<dyn GuildMemberUpdate>>,
    pub(crate) guild_members_chunk: Vec<Box<dyn GuildMembersChunk>>,
    pub(crate) guild_role_create: Vec<Box<dyn GuildRoleCreate>>,
    pub(crate) guild_role_update: Vec<Box<dyn GuildRoleUpdate>>,
    pub(crate) guild_role_delete: Vec<Box<dyn GuildRoleDelete>>,
    pub(crate) invite_create: Vec<Box<dyn InviteCreate>>,
    pub(crate) invite_delete: Vec<Box<dyn InviteDelete>>,
    pub(crate) message_create: Vec<Box<dyn MessageCreate>>,
    pub(crate) message_update: Vec<Box<dyn MessageUpdate>>,
    pub(crate) message_delete: Vec<Box<dyn MessageDelete>>,
    pub(crate) message_delete_bulk: Vec<Box<dyn MessageDeleteBulk>>,
    pub(crate) reaction_add: Vec<Box<dyn MessageReactionAdd>>,
    pub(crate) reaction_remove: Vec<Box<dyn MessageReactionRemove>>,
    pub(crate) reaction_remove_all: Vec<Box<dyn MessageReactionRemoveAll>>,
    pub(crate) presence_update: Vec<Box<dyn PresenceUpdate>>,
    pub(crate) typing_start: Vec<Box<dyn TypingStart>>,
    pub(crate) user_update: Vec<Box<dyn UserUpdate>>,
    pub(crate) voice_state_update: Vec<Box<dyn VoiceStateUpdate>>,
    pub(crate) voice_server_update: Vec<Box<dyn VoiceServerUpdate>>,
    pub(crate) webhooks_update: Vec<Box<dyn WebhooksUpdate>>,
}
