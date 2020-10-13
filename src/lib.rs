#![feature(test)]
#![feature(try_blocks)]
#![allow(clippy::identity_op)] //because clippy forbides 1 << 0 in c-like enums
#![allow(clippy::option_option)] //opt<opt<>> is required to properly handle nullables

//! A low level and asynchronous rust library made for interacting with Discord's API.
//!
//! This library provides all the tools that will handle setting up and maintaining
//! a connection to Discord's API in order to make a bot.
//! Before messing with the code of this library, you first need to get a bot token
//! on [Discord's developers portal](https://discordapp.com/developers/applications/).
//! Create a new application and add a bot to the newly created application.
//! You can then copy the bot's token by clicking the copy button.
//!
//! In order to build your bot, you must first provide it with the settings you'd like to use which
//! is done using the [Configuration](automate::Configuration) struct:
//! - [Configuration::new](automate::Configuration::new): Takes the bot token as parameter. You
//! can provide a hardcoded string, take it from the environment or retrieve it from a configuration
//! file.
//! - [Configuration::from_env](automate::Configuration::from_env): Does the same as `new` except
//! it takes the bot token from the given environment variable.
//! - [Configuration::register](automate::Configuration::register): Registers stateful and
//! stateless listeners.
//! - [Configuration::enable_logging](automate::Configuration::enable_logging) and
//! [Configuration::disable_logging](automate::Configuration::disable_logging): Enable or
//! disable Automate's built in logger. You can disable it and use your own logger if necessary.
//! - [Configuration::level_for](automate::Configuration::level_for): Sets the minimum log level
//! for a line to be printed in the console output for the given module.
//! - [Configuration::intents](automate::Configuration::intents): Sets the events which will
//! be sent to the bot using [intents](automate::Intent). Defaults to all events.
//! - [Configuration::presence](automate::Configuration::presence): Sets the presence of the bot.
//!
//! The resulting configuration object can then be sent to
//! [Automate::launch](automate::Automate::launch) which will start the bot.
//!
//! # Listeners
//! Discord sends various events through their API about messages, guild and
//! user updates, etc. Automate will then relay these events to your bot through
//! the listeners you will define. There are two ways to create listeners:
//!
//! ## Stateless listeners
//! The easiest way to create a listener is to use a stateless listener. A stateless listener is
//! a simple asynchronous function with the `#[listener]` attribute. As its name says, it doesn't
//! have a state and thus can't save data across calls.
//! ```
//! # use automate::{Context, Error, listener};
//! # use automate::gateway::MessageCreateDispatch;
//! #
//! #[listener]
//! async fn print_hello(ctx: &Context, data: &MessageCreateDispatch) -> Result<(), Error> {
//!     println!("Hello!");
//!     Ok(())
//! }
//! ```
//!
//! The function you declare must take two arguments as in the example above. The first
//! argument is the session, it provides information about the bot and all the methods
//! allowing you to send instructions to Discord through their HTTP API. The second
//! argument is the dispatch struct which contains all the data about the event you received.
//! Events and thus allowed types for the second argument are:
//! - [ReadyDispatch](automate::gateway::ReadyDispatch): called right after the connection with
//! discord is established. Provides data about current guilds, DMs and the bot user account.
//! - [ChannelCreateDispatch](automate::gateway::ChannelCreateDispatch): a channel (guild channel
//! or DM) was created.
//! - [ChannelUpdateDispatch](automate::gateway::ChannelUpdateDispatch): a channel (guild channel
//! or DM) was updated.
//! - [ChannelDeleteDispatch](automate::gateway::ChannelDeleteDispatch): a channel (guild channel
//! or DM) was deleted.
//! - [ChannelPinsUpdateDispatch](automate::gateway::ChannelPinsUpdateDispatch): a message was
//! pinned or unpinned.
//! - [GuildCreateDispatch](automate::gateway::GuildCreateDispatch): a guild was created, became
//! available or the bot was added to a guild.
//! - [GuildUpdateDispatch](automate::gateway::GuildUpdateDispatch): a guild was updated.
//! - [GuildDeleteDispatch](automate::gateway::GuildDeleteDispatch): a guild was deleted, became
//! unavailable or the bot was removed from the guild.
//! - [GuildBanAddDispatch](automate::gateway::GuildBanAddDispatch): a user was banned from a guild.
//! - [GuildBanRemoveDispatch](automate::gateway::GuildBanRemoveDispatch): a user was unbanned
//! from a guild.
//! - [GuildEmojisUpdateDispatch](automate::gateway::GuildEmojisUpdateDispatch): the emojis of a
//! guild were updated.
//! - [GuildIntegrationsUpdateDispatch](automate::gateway::GuildIntegrationsUpdateDispatch):
//! the integration of a guild was updated.
//! - [GuildMemberAddDispatch](automate::gateway::GuildMemberAddDispatch): a user joined a guild.
//! - [GuildMemberUpdateDispatch](automate::gateway::GuildMemberUpdateDispatch): a guild member was updated.
//! - [GuildMemberRemoveDispatch](automate::gateway::GuildMemberRemoveDispatch): a user was removed from a guild.
//! - [GuildMembersChunkDispatch](automate::gateway::GuildMembersChunkDispatch): response to a
//! request guild members (not yet implemented).
//! - [GuildRoleCreateDispatch](automate::gateway::GuildRoleCreateDispatch): a role was created.
//! - [GuildRoleUpdateDispatch](automate::gateway::GuildRoleUpdateDispatch): a role was updated.
//! - [GuildRoleDeleteDispatch](automate::gateway::GuildRoleDeleteDispatch): a role was deleted.
//! - [InviteCreateDispatch](automate::gateway::InviteCreateDispatch): an invite to a channel was created.
//! - [InviteDeleteDispatch](automate::gateway::InviteDeleteDispatch): an invited to a channel was deleted.
//! - [MessageCreateDispatch](automate::gateway::MessageCreateDispatch): a message was created
//! - [MessageUpdateDispatch](automate::gateway::MessageUpdateDispatch): a message updated.
//! - [MessageDeleteDispatch](automate::gateway::MessageDeleteDispatch): a message was deleted.
//! - [MessageDeleteBulkDispatch](automate::gateway::MessageDeleteBulkDispatch): multiple messages 
//! were deleted at once.
//! - [MessageReactionAddDispatch](automate::gateway::MessageReactionAddDispatch): a user reacted to a message.
//! - [MessageReactionRemoveDispatch](automate::gateway::MessageReactionRemoveDispatch): a user's
//! reaction was removed from a message.
//! - [MessageReactionRemoveAllDispatch](automate::gateway::MessageReactionRemoveAllDispatch): all
//! reactions were explicitly removed from a message.
//! - [MessageReactionRemoveEmojiDispatch](automate::gateway::MessageReactionRemoveEmojiDispatch):
//! all reactions for a given emoji were explicitly removed from a message.
//! - [PresenceUpdateDispatch](automate::gateway::PresenceUpdateDispatch): user was updated.
//! - [TypingStartDispatch](automate::gateway::TypingStartDispatch): user started typing in a channel.
//! - [UserUpdateDispatch](automate::gateway::UserUpdateDispatch): properties about the user changed.
//! - [VoiceStateUpdateDispatch](automate::gateway::VoiceStateUpdateDispatch): a user joined, left,
//! or moved a voice channel.
//! - [VoiceServerUpdateDispatch](automate::gateway::VoiceServerUpdateDispatch): guild's voice
//! server was updated.
//! - [WebhooksUpdateDispatch](automate::gateway::WebhooksUpdateDispatch): guild channel webhook
//! was created, update, or deleted.
//!
//! A listener function can be registered in the library by sending the name of the function to the
//! [Configuration::register](automate::Configuration::register) method using the `stateless!` macro:
//! ```
//! # use automate::{stateless, listener, Configuration, Context, Error};
//! # use automate::gateway::MessageCreateDispatch;
//! #
//! # #[listener]
//! # async fn print_hello(ctx: &Context, data: &MessageCreateDispatch) -> Result<(), Error> {
//! #     println!("Hello!");
//! #     Ok(())
//! # }
//! #
//! Configuration::from_env("DISCORD_API_TOKEN")
//!         .register(stateless!(print_hello));
//! ```
//!
//! More advanced examples can be found in the
//! [examples](https://github.com/mbenoukaiss/automate/tree/master/examples) folder.
//! If you want to keep data between calls, you probably want to use a stateful listener.
//!
//! It is possible to use `̀lazy_static!` to store data across calls but this is **probably not what
//! you want** since data in the `lazy_static` will be shared between shards and kept across
//! sessions.
//!
//! ## Stateful listeners
//! Stateless listeners provide a clean and quick way to setup a listener, but as stated earlier,
//! they do not allow keeping variables between two events which is necessary for a more
//! advanced bot.
//!
//! States will not be the same across shards and they will be destroyed and recreated in the case
//! of a disconnection that could not resume and results in the creation of a new Discord session.
//!
//! Stateful listeners work in the exact same way as stateless listeners except they're
//! declared in an impl block of a struct that derives the [State](automate::events::State) trait.
//! Structs containing stateful listeners must do 3 things:
//! - Derive the [State](automate::events::State) trait which can be done automatically using
//! the `#[derive(State)]` derive macro.
//! - Implement [Clone](std::clone::Clone) since they need to be cloned to be used between
//! different shards and sessions.
//! - Implement the [Initializable](automate::events::Initializable) trait which defines a
//! single function that should return all the listeners of the struct. This can be done using
//! the `methods!` macro which takes the name of the struct followed by a colon
//! and a comma-separated list of the listener methods.
//! 
//! ```
//! #[macro_use] extern crate automate;
//!
//! use automate::{Context, Error, Snowflake};
//! use automate::events::{Initializable, StatefulListener};
//! use automate::gateway::MessageCreateDispatch;
//! use std::collections::HashMap;
//!
//! #[derive(State, Default, Clone)]
//! struct MessageCounter {
//!     messages: i32,
//! }
//! 
//! impl Initializable for MessageCounter {
//!     fn initialize() -> Vec<StatefulListener<Self>> {
//!         methods!(MessageCounter: count)
//!     }
//! }
//! 
//! impl MessageCounter {
//!     #[listener]
//!     async fn count(&mut self, _: &Context, data: &MessageCreateDispatch) -> Result<(), Error> {
//!         self.messages += 1;
//!         println!("A total of {} messages have been sent!", self.messages);
//!
//!         Ok(())
//!     }
//! }
//! ```
//!
//! A state struct can be registered in the library by sending an instance of the struct to the
//! [Configuration::register](automate::Configuration::register) method using the `stateful!` macro.
//! ```
//! # #[macro_use] extern crate automate;
//! #
//! # use automate::{methods, stateful, Context, Error, Snowflake, Configuration};
//! # use automate::events::{Initializable, StatefulListener};
//! # use automate::gateway::MessageCreateDispatch;
//! # use std::collections::HashMap;
//! #
//! # #[derive(State, Default, Clone)]
//! # struct MessageCounter;
//! #
//! # impl Initializable for MessageCounter {
//! #     fn initialize() -> Vec<StatefulListener<Self>> {
//! #         methods!(MessageCounter)
//! #     }
//! # }
//! #
//! Configuration::from_env("DISCORD_API_TOKEN")
//!         .register(stateful!(MessageCounter::default()));
//! ```
//!
//! More advanced examples can be found in the  ̀examples/counter.rs` example file.
//!
//! # Storage API
//! When receiving events, you will usually need more data than the event sends you. For
//! example, you may need to know what the role of the user who sent a message is. This data
//! can be fetched through the storages using the [Context::storage](automate::Context::storage)
//! and [Context::storage_mut](automate::Context::storage_mut) to fetch mutable data.
//!
//! That can be achieved by fetching the data from Discord API each time you need it, but you will
//! quickly get rate limited. That is why the storage API caches some of the data discord sends.
//!
//! ## Caching storages
//! Automate creates 3 storages which you can **not** mutate, they only get mutated
//! through gateway events:
//! - [Guilds](automate::gateway::Guild)
//! - [Channels](automate::gateway::Channel)
//! - [Users](automate::gateway::User)
//!
//! ```
//! # use automate::listener;
//! use automate::{Context, Error};
//! use automate::gateway::{MessageCreateDispatch, User, Guild};
//!
//! #[listener]
//! async fn greet_multiple_roles(ctx: &Context, data: &MessageCreateDispatch) -> Result<(), Error> {
//!     if let Some(guild) = data.0.guild_id {
//!         let storage = ctx.storage::<Guild>().await;
//!         let guild = storage.get(guild);
//!         let member = guild.members.get(&data.0.author.id).unwrap();
//!
//!         //print hello if the user has at least 2 roles
//!         if member.roles.len() >= 2 {
//!             println!("Hello!");
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Custom storages
//! You can also create your own storages. Having your own custom storages will usually allow you
//! to store data without using stateful listeners and in a simpler way.
//!
//! In order to do that, you will need to create two structs
//! one being the stored struct which should implement [Stored](automate::storage::Stored) and
//! a storage struct which should keep the stored values in memory and provide ways to retrieve
//! and inserts objects. The storage struct should implement [Storage](automate::storage::Storage).
//!
//! See [examples/levels.rs](https://github.com/mbenoukaiss/automate/blob/master/examples/levels.rs)
//! for a detailed example.
//!
//! ## Deactivating storages
//! The storages API is by default enabled but you might not want it because you simply
//! do not need to cache the data sent by discord or because you do not have a lot of RAM
//! available to run the bot. In that case, you can disable the feature by setting the
//! `default-features` key to `false` for `automate` in your `Cargo.toml` file.
//!
//! # Sharding
//! Automate implements support for sharding through the [ShardManager](automate::ShardManager)
//! struct. However, you will not need to use the [ShardManager](automate::ShardManager) directly
//! in most cases since [Automate::launch](automate::Automate::launch) will automatically
//! create as many shards as Discord recommends.
//!
//! The reasons you would need to use the [ShardManager](automate::ShardManager) are if you want
//! to spread your bot across multiple servers or if you want to launch more or less
//! shards than what Discord recommends.
//!
//! # Models
//! All the data sent by discord is deserialized into [model structs and enums](automate::gateway::models).
//!
//! The data returned by discord can be of 4 kinds:
//! - Always present
//! - Nullable: Field will be included but the data can either be null or present.
//! - Optional: Field will either not be included at all or present
//! - Optional and nullable: The field can be present, null or not included.
//!
//! Both nullable and optional are handled with  [Option](std::option::Option) enum, but optional
//! nullable are wrapped in a double [Option](std::option::Option)s because in some cases you may
//! need to know whether the data was not present, null or both.
//!
//! For example, when editing a guild member, if you need to modify some fields but NOT the
//! nickname (which is optional and nullable), you will set the `nick` field to `None`.
//! But if you want to remove the nick, it needs to be set to null and you can achieve that
//! by sending `Some(None)`.
//!
//! # Examples
//! ```no_run
//! use automate::{listener, stateless, Error, Context, Configuration, Automate};
//! use automate::gateway::MessageCreateDispatch;
//! use automate::http::CreateMessage;
//!
//! #[listener]
//! async fn say_hello(ctx: &Context, data: &MessageCreateDispatch) -> Result<(), Error> {
//!     let message = &data.0;
//! 
//!     if message.author.id != ctx.bot.id {
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
//! let config = Configuration::from_env("DISCORD_API_TOKEN")
//!     .register(stateless!(say_hello));
//!
//! Automate::launch(config);
//! ```
//!

extern crate self as automate;
extern crate test;
#[macro_use]
extern crate automate_derive;
#[macro_use]
pub extern crate log;
#[macro_use]
extern crate serde;
extern crate tokio_tungstenite as tktungstenite;

pub mod events;
pub mod http;
pub mod encode;
pub mod gateway;
pub mod sharding;
#[cfg(feature = "storage")]
pub mod storage;
mod snowflake;
mod macros;
mod errors;
mod logger;

pub use automate_derive::listener;

/// Derive macro for a state struct.
///
/// Stateful listeners (methods annotated with  ̀#[listener]`)
/// need a bit of extra code to work properly. The necessary
/// code is generated by this derive macro.
/// It implements the [State](automate::events::State)
/// in order to relay the events to the methods annotated
/// with `#[listener]`
pub use automate_derive::State;

/// Derive macro for a stored struct.
///
/// Simply implements the Stored trait on the given type.
/// If not specified, the storage will be assumed to be
/// the name of the struct concatenated with `Storage`.
/// So a stored `Count` struct would define its storage
/// to be `CountStorage`.
///
/// It is possible to change the storage struct by
/// using the storage helper attribute like this :
/// ```
/// use automate::{Stored, Storage};
///
/// #[derive(Stored)]
/// #[storage(Counter)] //the storage is now the `Counter` struct
/// struct Count(i32);
///
/// #[derive(Storage)]
/// struct Counter;
/// ```
#[cfg(feature = "storage")]
pub use automate_derive::Stored;

/// Derive macro for a storage struct.
#[cfg(feature = "storage")]
pub use automate_derive::Storage;

/// Parses a list of stateless function listeners before sending them
/// to the [Configuration::register](automate::Configuration::register) method.
pub use automate_derive::stateless;

/// Parses a list of method listeners in the initialize method
/// of a state struct.
///
/// Input should be the name of the struct followed by a colon
/// and a comma-separated list of the listener functions that
/// should be registered.
///
/// ```
/// use automate::{methods, listener, Context, Error};
/// use automate::events::{Initializable, StatefulListener};
/// use automate::gateway::MessageCreateDispatch;
///
/// struct MessageCounter;
///
/// impl Initializable for MessageCounter {
///     fn initialize() -> Vec<StatefulListener<Self>> {
///         methods!(MessageCounter: say_hello, say_bye)
///     }
/// }
///
/// impl MessageCounter {
///     #[listener]
///     async fn say_hello(&mut self, ctx: &Context, data: &MessageCreateDispatch) -> Result<(), Error> {
///         println!("Hello!");
///         Ok(())
///     }
///
///     #[listener]
///     async fn say_bye(&mut self, ctx: &Context, data: &MessageCreateDispatch) -> Result<(), Error> {
///         println!("Bye");
///         Ok(())
///     }
/// }
/// ```
pub use automate_derive::methods;

/// Used internally for procedural macros, don't
/// rely on it's presence and import it manually in
/// your `Cargo.toml` instead
#[doc(no_inline)]
pub use async_trait::async_trait;

/// Used internally for procedural macros, don't
/// rely on it's presence and import it manually in
/// your `Cargo.toml` instead
#[doc(no_inline)]
pub use chrono;

/// Used internally for procedural macros, don't
/// rely on it's presence and import it manually in
/// your `Cargo.toml` instead
#[doc(no_inline)]
pub use lazy_static;

#[doc(no_inline)]
pub use tokio;

#[doc(inline)]
pub use http::HttpAPI;
#[doc(inline)]
pub use gateway::Context;
#[doc(inline)]
pub use gateway::Intent;

pub use sharding::ShardManager;
pub use snowflake::{Identifiable, Snowflake};
pub use errors::Error;

use events::*;
use tokio::runtime::Builder;
use std::env;
use log::LevelFilter;
use std::future::Future;
use crate::gateway::UpdateStatus;
#[cfg(feature = "storage")]
use crate::storage::StorageContainer;

/// Allows specifying API token, registering
/// stateful and stateless listeners, stating
/// the shard id, intents and configuring logger.
///
/// The resulting object can then be used in
/// [Automate::launch](automate::Automate::launch)
/// or the [ShardManager](automate::ShardManager).
///
/// # Example
/// ```
/// # use automate::{stateless, listener, Context, Configuration, Error};
/// # use automate::gateway::MessageCreateDispatch;
///
/// # #[listener]
/// # async fn kick_spammer(ctx: &Context, data: &MessageCreateDispatch) -> Result<(), Error> {
/// #     Ok(())
/// # }
/// #
/// let config = Configuration::from_env("DISCORD_API_TOKEN")
///        .disable_logging()
///        .register(stateless!(kick_spammer));
/// ```
#[derive(Clone)]
pub struct Configuration {
    shard_id: Option<u32>,
    total_shards: Option<u32>,
    token: String,
    logging: bool,
    log_levels: Vec<(String, LevelFilter)>,
    listeners: ListenerContainer,
    #[cfg(feature = "storage")]
    storages: StorageContainer,
    intents: u32,
    member_threshold: Option<u32>,
    presence: Option<UpdateStatus>,
    guild_subscriptions: Option<bool>,
    collector_period: u64
}

impl Configuration {
    /// Creates a configuration with the specified
    /// API token.
    ///
    /// This configuration will by default have
    /// logging enabled and outputting all logs with
    /// a level higher or equal to `LevelFiler::Info`
    pub fn new<S: Into<String>>(token: S) -> Configuration {
        let mut default_levels = Vec::new();
        default_levels.push((String::from("automate"), LevelFilter::Info));

        Configuration {
            shard_id: None,
            total_shards: None,
            token: token.into(),
            logging: true,
            log_levels: default_levels,
            listeners: ListenerContainer::default(),
            #[cfg(feature = "storage")]
            storages: StorageContainer::for_initialization(),
            intents: Intent::all(),
            member_threshold: None,
            presence: None,
            guild_subscriptions: None,
            collector_period: 3600
        }
    }

    /// Creates a configuration by taking the API
    /// token from the specified environment variable.
    /// The environment variable is retrieved using
    /// [env::var](std::env::var) and thus needs to
    /// be specified at runtime and not compile time.
    ///
    /// This configuration will by default have
    /// logging enabled and outputting all logs with
    /// a level higher or equal to `LevelFiler::Info`
    pub fn from_env<S: Into<String>>(env: S) -> Configuration {
        Configuration::new(env::var(&env.into()).expect("API token not found"))
    }

    /// Sets the shard id of this configuration and
    /// the total amount of shards.
    ///
    /// If using the [ShardManager](automate::ShardManager)
    /// or [Automate::launch](automate::Automate::launch) to
    /// launch the bot, you should not use this function
    /// since it is done automatically.
    pub fn shard(&mut self, shard_id: u32, total_shards: u32) -> &mut Self {
        self.shard_id = Some(shard_id);
        self.total_shards = Some(total_shards);
        self
    }

    /// Sets the API token.
    pub fn token<S: Into<String>>(mut self, token: S) -> Self {
        self.token = token.into();
        self
    }

    /// Enables logging. Logger is enabled by default.
    pub fn enable_logging(mut self) -> Self {
        self.logging = true;
        self
    }

    /// Disables logging.
    pub fn disable_logging(mut self) -> Self {
        self.logging = false;
        self
    }

    /// Sets the minimum log level for the given module.
    ///
    /// By default, automate will be set to
    /// [LevelFilter::Info](log::LevelFilter::Info) and its
    /// dependencies won't log anything.
    pub fn level_for<S: Into<String>>(mut self, module: S, min: LevelFilter) -> Self {
        let mut module = module.into().replace('-', "_");
        module.push_str("::");

        let existing = self.log_levels.iter()
            .enumerate()
            .filter(|(_, (m, _))| *m == module)
            .map(|(i, _)| i)
            .next();

        if let Some(existing) = existing {
            self.log_levels.remove(existing);
        }

        self.log_levels.push((module, min));
        self
    }

    /// Registers a listener state or a stateless
    /// listener function with the `̀#[listener]` attribute.
    pub fn register(mut self, listeners: Vec<ListenerType>) -> Self {
        self.listeners.register(listeners);
        self
    }

    /// Registers a function that initializes a storage by either calling
    /// [StorageContainer::initialize](automate::storage::StorageContainer)
    /// which accepts an existing storage or
    /// [StorageContainer::write](automate::storage::StorageContainer)
    /// which creates an empty storage and calls the provided callback function.
    #[cfg(feature = "storage")]
    pub fn add_initializer<F: Fn(&mut StorageContainer) + Send + Sync + 'static>(mut self, initializer: F) -> Self {
        self.storages.add_initializer::<F>(initializer);
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
    /// use automate::{Configuration, Intent::*};
    ///
    /// Configuration::from_env("DISCORD_API_TOKEN")
    ///         .intents(GuildMessages | GuildMessageTyping);
    /// ```
    ///
    /// If you want to only listen to one intent type, you must
    /// explicitly cast the intent to u32 like the following example
    /// which only listens to events about guild members:
    /// ```
    /// use automate::{Configuration, Intent::*};
    ///
    /// Configuration::from_env("DISCORD_API_TOKEN")
    ///         .intents(GuildMembers as u32);
    /// ```
    pub fn intents(mut self, intents: u32) -> Self {
        self.intents = intents;
        self
    }

    /// Number of members where the gateway will stop sending offline members
    /// in the guild member list. The value must be between 50 and 250.
    ///
    /// If not set, the value will default to 50.
    pub fn member_threshold(mut self, threshold: u32) -> Self {
        self.member_threshold = Some(threshold);
        self
    }

    /// Sets the presence of the bot.
    ///
    /// This can later be modified using the
    /// [update_status](automate::Context::update_status)
    /// gateway command.
    pub fn presence(mut self, presence: UpdateStatus) -> Self {
        self.presence = Some(presence);
        self
    }

    /// Enables dispatching of guild subscription events
    /// (presence and typing events).
    ///
    /// Defaults to true.
    pub fn guild_subscriptions(mut self, enabled: bool) -> Self {
        self.guild_subscriptions = Some(enabled);
        self
    }

    /// Sets the bucket collector period in seconds. Defaults
    /// to one hour.
    ///
    /// A bucket is a structure specifying the state of the
    /// rate-limit of a specified endpoint. It contains the
    /// amount of remaining API calls for the endpoint and
    /// the time at which it will reset.
    ///
    /// After each HTTP API call and in order to not reach the
    /// rate-limit set by Discord, the library keeps a hashmap
    /// associating the routes to their bucket. However, when
    /// these buckets reset, it is not necessary to keep them
    /// in memory anymore.
    /// Keeping buckets as long as the program runs would
    /// gradually use more and more memory as the bot joins
    /// and leaves guilds and sends DMs to users.
    ///
    /// The bucket collector was thus created to clean
    /// up memory every `period` seconds.
    ///
    /// For bots that are only in a few guilds and are meant to
    /// stay that way, it is not necessary to run the collector
    /// often. Once every few days will suffice.
    pub fn collector_period(mut self, period: u64) -> Self {
        self.collector_period = period;
        self
    }
}

/// Defines utility functions.
pub struct Automate;

impl Automate {
    /// Launches a basic bot with the given configuration
    /// and the amount of shards recommended by Discord.
    pub fn launch(config: Configuration) {
        Automate::block_on(async move {
            ShardManager::with_config(config).await
                .unwrap()
                .auto_setup()
                .launch().await
        })
    }

    /// Creates a tokio runtime and runs the
    /// given future inside.
    pub fn block_on<F: Future>(future: F) -> F::Output {
        let mut runtime = Builder::new()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(future)
    }
}

