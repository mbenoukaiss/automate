#![feature(test)]
#![feature(try_blocks)]
#![feature(proc_macro_hygiene)]
#![feature(specialization)]
#![allow(clippy::identity_op)] //because clippy forbides 1 << 0 in c-like enums
#![allow(where_clauses_object_safety)] //should be fixable when async traits are allowed

//! A low level and asynchronous rust library made for interacting with Discord's API.
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
//! - [Discord::register](automate::Discord::register) : Registers a listener struct or function
//!
//! # Listeners
//! Discord sends various events through their API about messages, guild and
//! user updates, etc. Automate will then relay these events to your bot through
//! the listeners you will define. There are two ways to create listeners :
//!
//! ## Listener function
//! The first and recommended way is to use the `#[listener]` attribute on a function.
//! ```
//! # use automate::{Context, Error, listener};
//! # use automate::gateway::MessageCreateDispatch;
//! #
//! #[listener]
//! async fn print_hello(ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
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
//! [Discord::register](automate::Discord::register) method using the `functions` macro :
//! ```
//! # use automate::{Discord, Context, Error, functions, listener};
//! # use automate::gateway::MessageCreateDispatch;
//! #
//! # #[listener]
//! # async fn print_hello(ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
//! #     println!("Hello!");
//! #     Ok(())
//! # }
//! #
//! # let api_token = std::env::var("DISCORD_API_TOKEN").expect("API token not found");
//! Discord::new(&api_token)
//!         .register(functions!(print_hello))
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
//! use automate::{Context, Error, Listener, async_trait};
//! use automate::gateway::MessageCreateDispatch;
//!
//! #[derive(Default, Clone)]
//! struct MessageCounter {
//!     messages: u32
//! }
//! 
//! #[async_trait]
//! impl Listener for MessageCounter {
//!     async fn on_message_create(&mut self, ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
//!         self.messages += 1;
//!         println!("A total of {} messages have been sent!", self.messages);
//!
//!         Ok(())
//!     }
//! }
//! ```
//!
//! A listener struct can be registered in the library by sending an instance of the struct to the
//! [Discord::register](automate::Discord::register) method using the `traits` macro :
//! ```
//! # use automate::{Discord, Listener, structs, async_trait};
//! #
//! # #[derive(Default, Clone)]
//! # struct MessageCounter;
//! #
//! # #[async_trait]
//! # impl Listener for MessageCounter { }
//! #
//! # let api_token = std::env::var("DISCORD_API_TOKEN").expect("API token not found");
//! Discord::new(&api_token)
//!         .register(structs!(MessageCounter::default()))
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
//! use automate::{Error, Discord, Context};
//! use automate::gateway::MessageCreateDispatch;
//! use automate::http::CreateMessage;
//! use std::env;
//! 
//! #[listener]
//! async fn say_hello(ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
//!     let message = &data.0;
//! 
//!     if message.author.id != ctx.bot().id {
//!         let content = Some(format!("Hello {}!", message.author.username));
//! 
//!         ctx.create_message(message.channel_id, CreateMessage {
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
//!         .register(functions!(say_hello))
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
/// to the [Discord::register](automate::Discord::register) method.
#[proc_macro_hack]
pub use automate_derive::functions;

#[doc(no_inline)]
pub use async_trait::async_trait;
#[doc(no_inline)]
pub use tokio;

#[doc(inline)]
pub use events::Listener;
#[doc(inline)]
pub use http::HttpAPI;
#[doc(inline)]
pub use gateway::Context;
#[doc(inline)]
pub use gateway::Intent;

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
    intents: Option<u32>
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
            intents: None
        }
    }

    /// Registers an event listener struct that implements
    /// the [Listener](automate::Listener) trait or
    /// a listener function with the `̀#[listener]` attribute. 
    pub fn register(mut self, listeners: Vec<ListenerType>) -> Self {
        self.listeners.register(listeners);
        self
    }

    /// [Intents](automate::Intent) are a system to help you
    /// lower the amount of data you need to process by
    /// specifying the events Discord should relay to the library.
    ///
    /// An [Intents](automate::Intent) concerns one or more
    /// event. By default, intents are not specified thus the bot
    /// is subscribed to all events.
    ///
    /// When specifying a single [], you must explicitly cast
    /// the intent to u32. When specifying multiple intents,
    /// you can aggregate them using the bitwise or operator.
    ///
    /// # Example
    /// The bot in the following example will only receive events
    /// about message creation, update and deletion and when a
    /// user starts typing in a guild channel:
    /// ```
    /// # let api_token = std::env::var("DISCORD_API_TOKEN").expect("API token not found");
    /// use automate::{Discord, Intent::*};
    ///
    /// Discord::new(&api_token)
    ///         .set_intents(GuildMessages | GuildMessageTyping)
    ///         .connect();
    /// ```
    ///
    /// If you want to only listen to one intent type, you must
    /// explicitly cast the intent to u32 like the following example
    /// which only listens to events about guild members:
    /// ```
    /// # let api_token = std::env::var("DISCORD_API_TOKEN").expect("API token not found");
    /// use automate::{Discord, Intent::*};
    ///
    /// Discord::new(&api_token)
    ///         .set_intents(GuildMembers as u32)
    ///         .connect();
    /// ```
    pub fn set_intents(mut self, intents: u32) -> Self {
        self.intents = Some(intents);
        self
    }

    /// Registers an event listener struct that implements
    /// the [Listener](automate::events::Listener) trait
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn with<L: Listener + Send + 'static>(mut self, listener: L) -> Self {
        self.listeners.trait_listeners.push(Box::new(listener));
        self
    }

    /// Registers an event that listens to [Ready](automate::gateway::ReadyDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_ready(mut self, listener: Ready) -> Self {
        self.listeners.ready.push(listener);
        self
    }

    /// Registers an event that listens to [ChannelCreate](automate::gateway::ChannelCreateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_channel_create(mut self, listener: ChannelCreate) -> Self {
        self.listeners.channel_create.push(listener);
        self
    }

    /// Registers an event that listens to [ChannelUpdate](automate::gateway::ChannelUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_channel_update(mut self, listener: ChannelUpdate) -> Self {
        self.listeners.channel_update.push(listener);
        self
    }

    /// Registers an event that listens to [ChannelDelete](automate::gateway::ChannelDeleteDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_channel_delete(mut self, listener: ChannelDelete) -> Self {
        self.listeners.channel_delete.push(listener);
        self
    }

    /// Registers an event that listens to [ChannelPinsUpdate](automate::gateway::ChannelPinsUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_channel_pins_update(mut self, listener: ChannelPinsUpdate) -> Self {
        self.listeners.channel_pins_update.push(listener);
        self
    }

    /// Registers an event that listens to [GuildCreate](automate::gateway::GuildCreateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_guild_create(mut self, listener: GuildCreate) -> Self {
        self.listeners.guild_create.push(listener);
        self
    }

    /// Registers an event that listens to [GuildUpdate](automate::gateway::GuildUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_guild_update(mut self, listener: GuildUpdate) -> Self {
        self.listeners.guild_update.push(listener);
        self
    }

    /// Registers an event that listens to [GuildDelete](automate::gateway::GuildDeleteDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_guild_delete(mut self, listener: GuildDelete) -> Self {
        self.listeners.guild_delete.push(listener);
        self
    }

    /// Registers an event that listens to [GuildBanAdd](automate::gateway::GuildBanAddDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_guild_ban_add(mut self, listener: GuildBanAdd) -> Self {
        self.listeners.guild_ban_add.push(listener);
        self
    }

    /// Registers an event that listens to [GuildBanRemove](automate::gateway::GuildBanRemoveDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_guild_ban_remove(mut self, listener: GuildBanRemove) -> Self {
        self.listeners.guild_ban_remove.push(listener);
        self
    }

    /// Registers an event that listens to [GuildEmojisUpdate](automate::gateway::GuildEmojisUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_guild_emojis_update(mut self, listener: GuildEmojisUpdate) -> Self {
        self.listeners.guild_emojis_update.push(listener);
        self
    }

    /// Registers an event that listens to [GuildIntegrationsUpdate](automate::gateway::GuildIntegrationsUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_guild_integrations_update(mut self, listener: GuildIntegrationsUpdate) -> Self {
        self.listeners.guild_integrations_update.push(listener);
        self
    }

    /// Registers an event that listens to [GuildMemberAdd](automate::gateway::GuildMemberAddDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_guild_member_add(mut self, listener: GuildMemberAdd) -> Self {
        self.listeners.guild_member_add.push(listener);
        self
    }

    /// Registers an event that listens to [GuildMemberRemove](automate::gateway::GuildMemberRemoveDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_guild_member_remove(mut self, listener: GuildMemberRemove) -> Self {
        self.listeners.guild_member_remove.push(listener);
        self
    }

    /// Registers an event that listens to [GuildMemberUpdate](automate::gateway::GuildMemberUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_guild_member_update(mut self, listener: GuildMemberUpdate) -> Self {
        self.listeners.guild_member_update.push(listener);
        self
    }

    /// Registers an event that listens to [GuildMembersChunk](automate::gateway::GuildMembersChunkDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_guild_members_chunk(mut self, listener: GuildMembersChunk) -> Self {
        self.listeners.guild_members_chunk.push(listener);
        self
    }

    /// Registers an event that listens to [GuildRoleCreate](automate::gateway::GuildRoleCreateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_guild_role_create(mut self, listener: GuildRoleCreate) -> Self {
        self.listeners.guild_role_create.push(listener);
        self
    }

    /// Registers an event that listens to [GuildRoleUpdate](automate::gateway::GuildRoleUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_guild_role_update(mut self, listener: GuildRoleUpdate) -> Self {
        self.listeners.guild_role_update.push(listener);
        self
    }

    /// Registers an event that listens to [GuildRoleDelete](automate::gateway::GuildRoleDeleteDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_guild_role_delete(mut self, listener: GuildRoleDelete) -> Self {
        self.listeners.guild_role_delete.push(listener);
        self
    }

    /// Registers an event that listens to [InviteCreate](automate::gateway::InviteCreateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_invite_create(mut self, listener: InviteCreate) -> Self {
        self.listeners.invite_create.push(listener);
        self
    }

    /// Registers an event that listens to [InviteDelete](automate::gateway::InviteDeleteDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_invite_delete(mut self, listener: InviteDelete) -> Self {
        self.listeners.invite_delete.push(listener);
        self
    }

    /// Registers an event that listens to [MessageCreate](automate::gateway::MessageCreateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_message_create(mut self, listener: MessageCreate) -> Self {
        self.listeners.message_create.push(listener);
        self
    }

    /// Registers an event that listens to [MessageUpdate](automate::gateway::MessageUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_message_update(mut self, listener: MessageUpdate) -> Self {
        self.listeners.message_update.push(listener);
        self
    }

    /// Registers an event that listens to [MessageDelete](automate::gateway::MessageDeleteDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_message_delete(mut self, listener: MessageDelete) -> Self {
        self.listeners.message_delete.push(listener);
        self
    }

    /// Registers an event that listens to [MessageDeleteBulk](automate::gateway::MessageDeleteBulkDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_message_delete_bulk(mut self, listener: MessageDeleteBulk) -> Self {
        self.listeners.message_delete_bulk.push(listener);
        self
    }

    /// Registers an event that listens to [MessageReactionAdd](automate::gateway::MessageReactionAddDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_reaction_add(mut self, listener: MessageReactionAdd) -> Self {
        self.listeners.reaction_add.push(listener);
        self
    }

    /// Registers an event that listens to [MessageReactionRemove](automate::gateway::MessageReactionRemoveDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_reaction_remove(mut self, listener: MessageReactionRemove) -> Self {
        self.listeners.reaction_remove.push(listener);
        self
    }

    /// Registers an event that listens to [MessageReactionRemoveAll](automate::gateway::MessageReactionRemoveAllDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_reaction_remove_all(mut self, listener: MessageReactionRemoveAll) -> Self {
        self.listeners.reaction_remove_all.push(listener);
        self
    }

    /// Registers an event that listens to [PresenceUpdate](automate::gateway::PresenceUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_presence_update(mut self, listener: PresenceUpdate) -> Self {
        self.listeners.presence_update.push(listener);
        self
    }

    /// Registers an event that listens to [TypingStart](automate::gateway::TypingStartDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_typing_start(mut self, listener: TypingStart) -> Self {
        self.listeners.typing_start.push(listener);
        self
    }

    /// Registers an event that listens to [UserUpdate](automate::gateway::UserUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_user_update(mut self, listener: UserUpdate) -> Self {
        self.listeners.user_update.push(listener);
        self
    }

    /// Registers an event that listens to [VoiceStateUpdate](automate::gateway::VoiceStateUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_voice_state_update(mut self, listener: VoiceStateUpdate) -> Self {
        self.listeners.voice_state_update.push(listener);
        self
    }

    /// Registers an event that listens to [VoiceServerUpdate](automate::gateway::VoiceServerUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_voice_server_update(mut self, listener: VoiceServerUpdate) -> Self {
        self.listeners.voice_server_update.push(listener);
        self
    }

    /// Registers an event that listens to [WebhooksUpdate](automate::gateway::WebhooksUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Discord::register` instead")]
    pub fn on_webhooks_update(mut self, listener: WebhooksUpdate) -> Self {
        self.listeners.webhooks_update.push(listener);
        self
    }

    /// Asynchronous function setup the connection
    /// with Discord.
    /// Will block forever unless the bot crashes.
    pub async fn connect(self) {
        GatewayAPI::connect(self.http, self.listeners, self.intents).await
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

#[derive(Default, Clone)]
pub(crate) struct ListenerStorage {
    pub(crate) trait_listeners: Vec<Box<dyn Listener>>,
    pub(crate) ready: Vec<Ready>,
    pub(crate) channel_create: Vec<ChannelCreate>,
    pub(crate) channel_update: Vec<ChannelUpdate>,
    pub(crate) channel_delete: Vec<ChannelDelete>,
    pub(crate) channel_pins_update: Vec<ChannelPinsUpdate>,
    pub(crate) guild_create: Vec<GuildCreate>,
    pub(crate) guild_update: Vec<GuildUpdate>,
    pub(crate) guild_delete: Vec<GuildDelete>,
    pub(crate) guild_ban_add: Vec<GuildBanAdd>,
    pub(crate) guild_ban_remove: Vec<GuildBanRemove>,
    pub(crate) guild_emojis_update: Vec<GuildEmojisUpdate>,
    pub(crate) guild_integrations_update: Vec<GuildIntegrationsUpdate>,
    pub(crate) guild_member_add: Vec<GuildMemberAdd>,
    pub(crate) guild_member_remove: Vec<GuildMemberRemove>,
    pub(crate) guild_member_update: Vec<GuildMemberUpdate>,
    pub(crate) guild_members_chunk: Vec<GuildMembersChunk>,
    pub(crate) guild_role_create: Vec<GuildRoleCreate>,
    pub(crate) guild_role_update: Vec<GuildRoleUpdate>,
    pub(crate) guild_role_delete: Vec<GuildRoleDelete>,
    pub(crate) invite_create: Vec<InviteCreate>,
    pub(crate) invite_delete: Vec<InviteDelete>,
    pub(crate) message_create: Vec<MessageCreate>,
    pub(crate) message_update: Vec<MessageUpdate>,
    pub(crate) message_delete: Vec<MessageDelete>,
    pub(crate) message_delete_bulk: Vec<MessageDeleteBulk>,
    pub(crate) reaction_add: Vec<MessageReactionAdd>,
    pub(crate) reaction_remove: Vec<MessageReactionRemove>,
    pub(crate) reaction_remove_all: Vec<MessageReactionRemoveAll>,
    pub(crate) reaction_remove_emoji: Vec<MessageReactionRemoveEmoji>,
    pub(crate) presence_update: Vec<PresenceUpdate>,
    pub(crate) typing_start: Vec<TypingStart>,
    pub(crate) user_update: Vec<UserUpdate>,
    pub(crate) voice_state_update: Vec<VoiceStateUpdate>,
    pub(crate) voice_server_update: Vec<VoiceServerUpdate>,
    pub(crate) webhooks_update: Vec<WebhooksUpdate>,
}

impl ListenerStorage {
    pub(crate) fn register(&mut self, listeners: Vec<ListenerType>) {
        for l in listeners {
            match l {
                ListenerType::Impl(l) => self.trait_listeners.push(l),
                ListenerType::Ready(l) => self.ready.push(l),
                ListenerType::ChannelCreate(l) => self.channel_create.push(l),
                ListenerType::ChannelUpdate(l) => self.channel_update.push(l),
                ListenerType::ChannelDelete(l) => self.channel_delete.push(l),
                ListenerType::ChannelPinsUpdate(l) => self.channel_pins_update.push(l),
                ListenerType::GuildCreate(l) => self.guild_create.push(l),
                ListenerType::GuildUpdate(l) => self.guild_update.push(l),
                ListenerType::GuildDelete(l) => self.guild_delete.push(l),
                ListenerType::GuildBanAdd(l) => self.guild_ban_add.push(l),
                ListenerType::GuildBanRemove(l) => self.guild_ban_remove.push(l),
                ListenerType::GuildEmojisUpdate(l) => self.guild_emojis_update.push(l),
                ListenerType::GuildIntegrationsUpdate(l) => self.guild_integrations_update.push(l),
                ListenerType::GuildMemberAdd(l) => self.guild_member_add.push(l),
                ListenerType::GuildMemberRemove(l) => self.guild_member_remove.push(l),
                ListenerType::GuildMemberUpdate(l) => self.guild_member_update.push(l),
                ListenerType::GuildMembersChunk(l) => self.guild_members_chunk.push(l),
                ListenerType::GuildRoleCreate(l) => self.guild_role_create.push(l),
                ListenerType::GuildRoleUpdate(l) => self.guild_role_update.push(l),
                ListenerType::GuildRoleDelete(l) => self.guild_role_delete.push(l),
                ListenerType::InviteCreate(l) => self.invite_create.push(l),
                ListenerType::InviteDelete(l) => self.invite_delete.push(l),
                ListenerType::MessageCreate(l) => self.message_create.push(l),
                ListenerType::MessageUpdate(l) => self.message_update.push(l),
                ListenerType::MessageDelete(l) => self.message_delete.push(l),
                ListenerType::MessageDeleteBulk(l) => self.message_delete_bulk.push(l),
                ListenerType::MessageReactionAdd(l) => self.reaction_add.push(l),
                ListenerType::MessageReactionRemove(l) => self.reaction_remove.push(l),
                ListenerType::MessageReactionRemoveAll(l) => self.reaction_remove_all.push(l),
                ListenerType::MessageReactionRemoveEmoji(l) => self.reaction_remove_emoji.push(l),
                ListenerType::PresenceUpdate(l) => self.presence_update.push(l),
                ListenerType::TypingStart(l) => self.typing_start.push(l),
                ListenerType::UserUpdate(l) => self.user_update.push(l),
                ListenerType::VoiceStateUpdate(l) => self.voice_state_update.push(l),
                ListenerType::VoiceServerUpdate(l) => self.voice_server_update.push(l),
                ListenerType::WebhooksUpdate(l) => self.webhooks_update.push(l),
            }
        }
    }
}
