#![feature(test)]
#![feature(try_blocks)]
#![feature(proc_macro_hygiene)]
#![allow(clippy::identity_op)] //because clippy forbides 1 << 0 in c-like enums
#![allow(where_clauses_object_safety)] //should be fixable when async traits are allowed

//! A low level and asynchronous rust library for interacting with the Discord API
//!
//! This library provides all the tools that will handle setting up and maintaining
//! a connection to Discord's API in order to make a bot.
//! Before messing with the code of this library and make a bot, you first need to get a bot
//! token on [Discord's developers portal](https://discordapp.com/developers/applications/).
//! Create a new application and add a bot to the newly created application.
//! You can then copy the bot's token by clicking the copy button.
//!
//! Everything is handled through the [Discord](automate::Discord) struct :
//! - [Discord::new](automate::Discord::new) : Takes the bot token as parameter. You can provide a
//! hardcoded string, take it from the environment or retrieve it from a configuration file.
//! - [Discord::with](automate::Discord::with) : Registers a listener struct or function
//!
//! # Listeners
//! Discord sends various events through their API about messages, guild and
//! user updates, etc. Automate will then relay these events to your bot through
//! the listeners you will define. There are two ways to create listeners :
//!
//! ## Listener function
//! The first and recommended way is to use the `#[listener]` attribute on a function.
//! ```
//! # use automate::{Session, Error, listener};
//! # use automate::gateway::MessageCreateDispatch;
//! #
//! #[listener]
//! async fn print_hello(session: &Session, data: &MessageCreateDispatch) -> Result<(), Error> {
//!     println!("Hello!");
//!     Ok(())
//! }
//! ```
//!
//! The function you declare must take two arguments as in the example above. The first
//! argument is the session, it provides information about the bot and all the methods
//! allowing you to send instructions to Discord through their HTTP API. The second
//! argument is the dispatch struct which contains all the data about the event you received.
//! Events and thus allowed types for the second argument are :
//! - [ReadyDispatch](automate::gateway::ReadyDispatch) : called right after the connection with
//! discord is established. Provides data about current guilds, DMs and the bot user account.
//! - [ChannelCreateDispatch](automate::gateway::ChannelCreateDispatch) : a channel (guild channel
//! or DM) was created.
//! - [ChannelUpdateDispatch](automate::gateway::ChannelUpdateDispatch) : a channel (guild channel
//! or DM) was updated.
//! - [ChannelDeleteDispatch](automate::gateway::ChannelDeleteDispatch) : a channel (guild channel
//! or DM) was deleted.
//! - [ChannelPinsUpdateDispatch](automate::gateway::ChannelPinsUpdateDispatch) : a message was
//! pinned or unpinned.
//! - [GuildCreateDispatch](automate::gateway::GuildCreateDispatch) : a guild was created, became
//! available or the bot was added to a guild.
//! - [GuildUpdateDispatch](automate::gateway::GuildUpdateDispatch) : a guild was updated.
//! - [GuildDeleteDispatch](automate::gateway::GuildDeleteDispatch) : a guild was deleted, became
//! unavailable or the bot was removed from the guild.
//! - [GuildBanAddDispatch](automate::gateway::GuildBanAddDispatch) : a user was banned from a guild.
//! - [GuildBanRemoveDispatch](automate::gateway::GuildBanRemoveDispatch) : a user was unbanned
//! from a guild.
//! - [GuildEmojisUpdateDispatch](automate::gateway::GuildEmojisUpdateDispatch) : the emojis of a
//! guild were updated.
//! - [GuildIntegrationsUpdateDispatch](automate::gateway::GuildIntegrationsUpdateDispatch) :
//! the integration of a guild was updated.
//! - [GuildMemberAddDispatch](automate::gateway::GuildMemberAddDispatch) : a user joined a guild.
//! - [GuildMemberUpdateDispatch](automate::gateway::GuildMemberUpdateDispatch) : a guild member was updated.
//! - [GuildMemberRemoveDispatch](automate::gateway::GuildMemberRemoveDispatch) : a user was removed from a guild.
//! - [GuildMembersChunkDispatch](automate::gateway::GuildMembersChunkDispatch) : response to a
//! request guild members (not yet implemented).
//! - [GuildRoleCreateDispatch](automate::gateway::GuildRoleCreateDispatch) : a role was created.
//! - [GuildRoleUpdateDispatch](automate::gateway::GuildRoleUpdateDispatch) : a role was updated.
//! - [GuildRoleDeleteDispatch](automate::gateway::GuildRoleDeleteDispatch) : a role was deleted.
//! - [InviteCreateDispatch](automate::gateway::InviteCreateDispatch) : an invite to a channel was created.
//! - [InviteDeleteDispatch](automate::gateway::InviteDeleteDispatch) : an invited to a channel was deleted.
//! - [MessageCreateDispatch](automate::gateway::MessageCreateDispatch) : a message was created
//! - [MessageUpdateDispatch](automate::gateway::MessageUpdateDispatch) : a message updated.
//! - [MessageDeleteDispatch](automate::gateway::MessageDeleteDispatch) : a message was deleted.
//! - [MessageDeleteBulkDispatch](automate::gateway::MessageDeleteBulkDispatch) : multiple messages 
//! were deleted at once.
//! - [MessageReactionAddDispatch](automate::gateway::MessageReactionAddDispatch) : a user reacted to a message.
//! - [MessageReactionRemoveDispatch](automate::gateway::MessageReactionRemoveDispatch) : a user's
//! reaction was removed from a message.
//! - [MessageReactionRemoveAllDispatch](automate::gateway::MessageReactionRemoveAllDispatch) : all
//! reactions were explicitly removed from a message.
//! - [MessageReactionRemoveEmojiDispatch](automate::gateway::MessageReactionRemoveEmojiDispatch) :
//! all reactions for a given emoji were explicitly removed from a message.
//! - [PresenceUpdateDispatch](automate::gateway::PresenceUpdateDispatch) : user was updated.
//! - [TypingStartDispatch](automate::gateway::TypingStartDispatch) : user started typing in a channel.
//! - [UserUpdateDispatch](automate::gateway::UserUpdateDispatch) : properties about the user changed.
//! - [VoiceStateUpdateDispatch](automate::gateway::VoiceStateUpdateDispatch) : a user joined, left,
//! or moved a voice channel.
//! - [VoiceServerUpdateDispatch](automate::gateway::VoiceServerUpdateDispatch) : guild's voice
//! server was updated.
//! - [WebhooksUpdateDispatch](automate::gateway::WebhooksUpdateDispatch) : guild channel webhook
//! was created, update, or deleted.
//!
//! A listener function can be registered in the library by sending the name of the function to the
//! [Discord::with](automate::Discord::with) method using the `functions` macro :
//! ```
//! # use automate::{Discord, Session, Error, functions, listener};
//! # use automate::gateway::MessageCreateDispatch;
//! #
//! # #[listener]
//! # async fn print_hello(session: &Session, data: &MessageCreateDispatch) -> Result<(), Error> {
//! #     println!("Hello!");
//! #     Ok(())
//! # }
//! #
//! # let api_token = std::env::var("DISCORD_API_TOKEN").expect("API token not found");
//! Discord::new(&api_token)
//!         .with(functions!(print_hello))
//!         .connect();
//! ```
//!
//! More advanced examples can be found in the  ̀examples/basic.rs` example file.
//! It is not (yet) possible to create a listener function in an impl block. If you want to use
//! functions in an impl block in order to keep variables, you should use the 
//! [Listener](automate::Listener) trait.
//!
//! ## Listener struct
//! Listener attribute functions provide a clean and quick way to setup a listener, 
//! however they do not allow keeping variables between two events.
//! 
//! ```
//! use automate::{Session, Error, Listener, async_trait};
//! use automate::gateway::MessageCreateDispatch;
//!
//! #[derive(Default)]
//! struct MessageCounter {
//!     messages: u32
//! }
//! 
//! #[async_trait]
//! impl Listener for MessageCounter {
//!     async fn on_message_create(&mut self, session: &Session, data: &MessageCreateDispatch) -> Result<(), Error> {
//!         self.messages += 1;
//!         println!("A total of {} messages have been sent!", self.messages);
//!
//!         Ok(())
//!     }
//! }
//! ```
//!
//! A listener struct can be registered in the library by sending an instance of the struct to the
//! [Discord::with](automate::Discord::with) method using the `traits` macro :
//! ```
//! # use automate::{Discord, Listener, structs, async_trait};
//! #
//! # #[derive(Default)]
//! # struct MessageCounter;
//! #
//! # #[async_trait]
//! # impl Listener for MessageCounter { }
//! #
//! # let api_token = std::env::var("DISCORD_API_TOKEN").expect("API token not found");
//! Discord::new(&api_token)
//!         .with(structs!(MessageCounter::default()))
//!         .connect();
//! ```
//!
//! More advanced examples can be found in the  ̀examples/counter.rs` example file.
//!
//! # Examples
//! ```no_run
//! #[macro_use]
//! extern crate automate;
//! 
//! use automate::{Error, Discord, Session};
//! use automate::gateway::MessageCreateDispatch;
//! use automate::http::CreateMessage;
//! use std::env;
//! 
//! #[listener]
//! async fn say_hello(session: &Session, data: &MessageCreateDispatch) -> Result<(), Error> {
//!     let message = &data.0;
//! 
//!     if message.author.id != session.bot().id {
//!         let content = Some(format!("Hello {}!", message.author.username));
//! 
//!         session.create_message(message.channel_id, CreateMessage {
//!             content,
//!             ..Default::default()
//!         }).await?;
//!     }
//! 
//!     Ok(())
//! }
//! 
//! fn main() {
//!     automate::setup_logging();
//! 
//!     Discord::new(&env::var("DISCORD_API_TOKEN").expect("API token not found"))
//!         .with(functions!(say_hello))
//!         .connect_blocking()
//! }
//! ```
//!

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
pub mod encode;
pub mod gateway;
mod snowflake;
mod macros;
mod errors;
mod logger;

pub use automate_derive::listener;

/// Parses a list of function listeners before sending them
/// to the [Discord::with](automate::Discord::with) method.
#[proc_macro_hack]
pub use automate_derive::functions;

pub use async_trait::async_trait;
pub use tokio;

pub use events::Listener;
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
    /// the [Listener](automate::Listener) trait or
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
                ListenerType::MessageReactionRemoveEmoji(l) => self.listeners.reaction_remove_emoji.push(l),
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

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_ready(mut self, listener: impl Ready) -> Self {
        self.listeners.ready.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_channel_create(mut self, listener: impl ChannelCreate) -> Self {
        self.listeners.channel_create.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_channel_update(mut self, listener: impl ChannelUpdate) -> Self {
        self.listeners.channel_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_channel_delete(mut self, listener: impl ChannelDelete) -> Self {
        self.listeners.channel_delete.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_channel_pins_update(mut self, listener: impl ChannelPinsUpdate) -> Self {
        self.listeners.channel_pins_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_guild_create(mut self, listener: impl GuildCreate) -> Self {
        self.listeners.guild_create.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_guild_update(mut self, listener: impl GuildUpdate) -> Self {
        self.listeners.guild_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_guild_delete(mut self, listener: impl GuildDelete) -> Self {
        self.listeners.guild_delete.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_guild_ban_add(mut self, listener: impl GuildBanAdd) -> Self {
        self.listeners.guild_ban_add.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_guild_ban_remove(mut self, listener: impl GuildBanRemove) -> Self {
        self.listeners.guild_ban_remove.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_guild_emojis_update(mut self, listener: impl GuildEmojisUpdate) -> Self {
        self.listeners.guild_emojis_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_guild_integrations_update(mut self, listener: impl GuildIntegrationsUpdate) -> Self {
        self.listeners.guild_integrations_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_guild_member_add(mut self, listener: impl GuildMemberAdd) -> Self {
        self.listeners.guild_member_add.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_guild_member_remove(mut self, listener: impl GuildMemberRemove) -> Self {
        self.listeners.guild_member_remove.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_guild_member_update(mut self, listener: impl GuildMemberUpdate) -> Self {
        self.listeners.guild_member_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_guild_members_chunk(mut self, listener: impl GuildMembersChunk) -> Self {
        self.listeners.guild_members_chunk.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_guild_role_create(mut self, listener: impl GuildRoleCreate) -> Self {
        self.listeners.guild_role_create.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_guild_role_update(mut self, listener: impl GuildRoleUpdate) -> Self {
        self.listeners.guild_role_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_guild_role_delete(mut self, listener: impl GuildRoleDelete) -> Self {
        self.listeners.guild_role_delete.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_invite_create(mut self, listener: impl InviteCreate) -> Self {
        self.listeners.invite_create.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_invite_delete(mut self, listener: impl InviteDelete) -> Self {
        self.listeners.invite_delete.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_message_create(mut self, listener: impl MessageCreate) -> Self {
        self.listeners.message_create.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_message_update(mut self, listener: impl MessageUpdate) -> Self {
        self.listeners.message_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_message_delete(mut self, listener: impl MessageDelete) -> Self {
        self.listeners.message_delete.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_message_delete_bulk(mut self, listener: impl MessageDeleteBulk) -> Self {
        self.listeners.message_delete_bulk.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_reaction_add(mut self, listener: impl MessageReactionAdd) -> Self {
        self.listeners.reaction_add.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_reaction_remove(mut self, listener: impl MessageReactionRemove) -> Self {
        self.listeners.reaction_remove.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_reaction_remove_all(mut self, listener: impl MessageReactionRemoveAll) -> Self {
        self.listeners.reaction_remove_all.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_presence_update(mut self, listener: impl PresenceUpdate) -> Self {
        self.listeners.presence_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_typing_start(mut self, listener: impl TypingStart) -> Self {
        self.listeners.typing_start.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_user_update(mut self, listener: impl UserUpdate) -> Self {
        self.listeners.user_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_voice_state_update(mut self, listener: impl VoiceStateUpdate) -> Self {
        self.listeners.voice_state_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
    pub fn on_voice_server_update(mut self, listener: impl VoiceServerUpdate) -> Self {
        self.listeners.voice_server_update.push(Box::new(listener));
        self
    }

    #[deprecated(since = "0.3.0", note = "Please use `Discord::with` instead")]
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
    pub(crate) reaction_remove_emoji: Vec<Box<dyn MessageReactionRemoveEmoji>>,
    pub(crate) presence_update: Vec<Box<dyn PresenceUpdate>>,
    pub(crate) typing_start: Vec<Box<dyn TypingStart>>,
    pub(crate) user_update: Vec<Box<dyn UserUpdate>>,
    pub(crate) voice_state_update: Vec<Box<dyn VoiceStateUpdate>>,
    pub(crate) voice_server_update: Vec<Box<dyn VoiceServerUpdate>>,
    pub(crate) webhooks_update: Vec<Box<dyn WebhooksUpdate>>,
}
