#![feature(test)]
#![feature(try_blocks)]
#![feature(proc_macro_hygiene)]
#![allow(clippy::identity_op)] //because clippy forbides 1 << 0 in c-like enums
#![allow(where_clauses_object_safety)] //should be fixable when async traits are allowed

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
//! is done using the [Configuration](automate::Configuration) struct :
//! - [Configuration::new](automate::Configuration::new) : Takes the bot token as parameter. You
//! can provide a hardcoded string, take it from the environment or retrieve it from a configuration
//! file.
//! - [Configuration::from_env](automate::Configuration::from_env) : Does the same as `new` except
//! it takes the bot token from the given environment variable.
//! - [Configuration::register](automate::Configuration::register) : Registers stateful and
//! stateless listeners.
//! - [Configuration::enable_logging](automate::Configuration::enable_logging) and
//! [Configuration::disable_logging](automate::Configuration::disable_logging) : Enable or
//! disable Automate's built in logger. You can disable it and use your own logger if necessary.
//! - [Configuration::level_for](automate::Configuration::level_for) : Sets the minimum log level
//! for a line to be printed in the console output for the given module.
//! - [Configuration::intents](automate::Configuration::intents) : Sets the events which will
//! be sent to the bot using [intents](automate::Intent). Defaults to all events.
//! - [Configuration::presence](automate::Configuration::presence) : Sets the presence of the bot.
//!
//! The resulting configuration object can then be sent to
//! [Automate::launch](automate::Automate::launch) which will start the bot.
//!
//! # Listeners
//! Discord sends various events through their API about messages, guild and
//! user updates, etc. Automate will then relay these events to your bot through
//! the listeners you will define. There are two ways to create listeners :
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
//! [Configuration::register](automate::Configuration::register) method using the `stateless!` macro :
//! ```
//! # use automate::{stateless, listener, Configuration, Context, Error};
//! # use automate::gateway::MessageCreateDispatch;
//! #
//! # #[listener]
//! # async fn print_hello(ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
//! #     println!("Hello!");
//! #     Ok(())
//! # }
//! #
//! Configuration::from_env("DISCORD_API_TOKEN")
//!         .register(stateless!(print_hello));
//! ```
//!
//! More advanced examples can be found in the `examples/basic.rs` example file.
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
//! of a deconnexion that could not resume and results in the creation of a new Discord session.
//!
//! Stateful listeners work in the exact same way as stateless listeners except they're
//! declared in an impl block of a struct that derives the [State](automate::events::State) trait.
//! Structs containing stateful listeners must do 3 things :
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
//!     async fn count(&mut self, _: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
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
//! # Examples
//! ```no_run
//! use automate::{listener, stateless, Error, Context, Configuration, Automate};
//! use automate::gateway::MessageCreateDispatch;
//! use automate::http::CreateMessage;
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
//! let config = Configuration::from_env("DISCORD_API_TOKEN")
//!     .register(stateless!(say_hello));
//!
//! Automate::launch(config);
//! ```
//!

extern crate self as automate;
extern crate test;
#[macro_use]
extern crate proc_macro_hack;
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

/// Parses a list of stateless function listeners before sending them
/// to the [Configuration::register](automate::Configuration::register) method.
#[proc_macro_hack]
pub use automate_derive::stateless;

#[proc_macro_hack]
#[deprecated(since = "0.3.1", note = "Use `automate::stateless!` instead")]
pub use automate_derive::functions;

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
///     async fn say_hello(&mut self, ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
///         println!("Hello!");
///         Ok(())
///     }
///
///     #[listener]
///     async fn say_bye(&mut self, ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
///         println!("Bye");
///         Ok(())
///     }
/// }
/// ```
#[proc_macro_hack]
pub use automate_derive::methods;

#[doc(no_inline)]
pub use async_trait::async_trait;
#[doc(no_inline)]
pub use lazy_static;
#[doc(no_inline)]
pub use tokio;

#[doc(inline)]
pub use encode::json::Nullable;
#[doc(inline)]
#[allow(deprecated)]
pub use events::Listener;
#[doc(inline)]
pub use http::HttpAPI;
#[doc(inline)]
pub use gateway::Context;
#[doc(inline)]
pub use gateway::Intent;

#[allow(deprecated)]
pub use logger::setup_logging;
pub use sharding::ShardManager;
pub use snowflake::Snowflake;
pub use errors::Error;

use events::*;
use gateway::GatewayAPI;
use tokio::runtime::Runtime;
use std::env;
use log::LevelFilter;
use std::future::Future;
use crate::gateway::UpdateStatus;

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
/// # async fn kick_spammer(ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
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
    listeners: ListenerStorage,
    intents: Option<u32>,
    member_threshold: Option<u32>,
    presence: Option<UpdateStatus>,
    guild_subscriptions: Option<bool>
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
            listeners: ListenerStorage::default(),
            intents: None,
            member_threshold: None,
            presence: None,
            guild_subscriptions: None,
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

        self.log_levels.push((module, min));
        self
    }

    /// Registers a listener state or a stateless
    /// listener function with the `̀#[listener]` attribute.
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
    /// use automate::{Discord, Configuration, Intent::*};
    ///
    /// Configuration::from_env("DISCORD_API_TOKEN")
    ///         .intents(GuildMessages | GuildMessageTyping);
    /// ```
    ///
    /// If you want to only listen to one intent type, you must
    /// explicitly cast the intent to u32 like the following example
    /// which only listens to events about guild members:
    /// ```
    /// use automate::{Discord, Configuration, Intent::*};
    ///
    /// Configuration::from_env("DISCORD_API_TOKEN")
    ///         .intents(GuildMembers as u32);
    /// ```
    pub fn intents(mut self, intents: u32) -> Self {
        self.intents = Some(intents);
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
}

/// Defines utility functions.
pub struct Automate;

impl Automate {
    /// Launches a basic bot with the given configuration
    /// and the amount of shards recommended by Discord.
    pub fn launch(config: Configuration) {
        Automate::block_on(async move {
            ShardManager::with_config(config).await
                .auto_setup()
                .launch().await;
        });
    }

    /// Creates a tokio runtime and runs the
    /// given future inside.
    pub fn block_on<F: Future>(future: F) -> F::Output {
        Runtime::new().unwrap().block_on(logger::setup_for_task("main", future))
    }
}

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
#[deprecated(since = "0.3.1", note = "Please use `Configuration` instead and `Automate::launch` or `ShardManager` instead")]
pub struct Discord {
    config: Configuration
}

#[allow(deprecated)]
impl Discord {
    /// Creates an instance of this struct
    /// with the provided token.
    /// The token can be generated on
    /// [Discord's developers portal](https://discordapp.com/developers/applications/)
    #[deprecated(since = "0.3.1", note = "Please use `Configuration::new` instead")]
    pub fn new(token: &str) -> Discord {
        Discord {
            config: Configuration::new(token)
        }
    }

    /// Registers an event listener struct that implements
    /// the [Listener](automate::Listener) trait or
    /// a listener function with the `̀#[listener]` attribute. 
    #[deprecated(since = "0.3.1", note = "Please use `Configuration::register` instead")]
    pub fn register(mut self, listeners: Vec<ListenerType>) -> Self {
        self.config.listeners.register(listeners);
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
    #[deprecated(since = "0.3.1", note = "Please use `Configuration::intents` instead")]
    pub fn set_intents(mut self, intents: u32) -> Self {
        self.config.intents = Some(intents);
        self
    }

    /// Registers an event listener struct that implements
    /// the [Listener](automate::events::Listener) trait
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn with<L: Listener + Send + 'static>(mut self, listener: L) -> Self {
        self.config.listeners.trait_listeners.push(Box::new(listener));
        self
    }

    /// Registers an event that listens to [Ready](automate::gateway::ReadyDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_ready(mut self, listener: Ready) -> Self {
        self.config.listeners.ready.push(listener);
        self
    }

    /// Registers an event that listens to [ChannelCreate](automate::gateway::ChannelCreateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_channel_create(mut self, listener: ChannelCreate) -> Self {
        self.config.listeners.channel_create.push(listener);
        self
    }

    /// Registers an event that listens to [ChannelUpdate](automate::gateway::ChannelUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_channel_update(mut self, listener: ChannelUpdate) -> Self {
        self.config.listeners.channel_update.push(listener);
        self
    }

    /// Registers an event that listens to [ChannelDelete](automate::gateway::ChannelDeleteDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_channel_delete(mut self, listener: ChannelDelete) -> Self {
        self.config.listeners.channel_delete.push(listener);
        self
    }

    /// Registers an event that listens to [ChannelPinsUpdate](automate::gateway::ChannelPinsUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_channel_pins_update(mut self, listener: ChannelPinsUpdate) -> Self {
        self.config.listeners.channel_pins_update.push(listener);
        self
    }

    /// Registers an event that listens to [GuildCreate](automate::gateway::GuildCreateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_guild_create(mut self, listener: GuildCreate) -> Self {
        self.config.listeners.guild_create.push(listener);
        self
    }

    /// Registers an event that listens to [GuildUpdate](automate::gateway::GuildUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_guild_update(mut self, listener: GuildUpdate) -> Self {
        self.config.listeners.guild_update.push(listener);
        self
    }

    /// Registers an event that listens to [GuildDelete](automate::gateway::GuildDeleteDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_guild_delete(mut self, listener: GuildDelete) -> Self {
        self.config.listeners.guild_delete.push(listener);
        self
    }

    /// Registers an event that listens to [GuildBanAdd](automate::gateway::GuildBanAddDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_guild_ban_add(mut self, listener: GuildBanAdd) -> Self {
        self.config.listeners.guild_ban_add.push(listener);
        self
    }

    /// Registers an event that listens to [GuildBanRemove](automate::gateway::GuildBanRemoveDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_guild_ban_remove(mut self, listener: GuildBanRemove) -> Self {
        self.config.listeners.guild_ban_remove.push(listener);
        self
    }

    /// Registers an event that listens to [GuildEmojisUpdate](automate::gateway::GuildEmojisUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_guild_emojis_update(mut self, listener: GuildEmojisUpdate) -> Self {
        self.config.listeners.guild_emojis_update.push(listener);
        self
    }

    /// Registers an event that listens to [GuildIntegrationsUpdate](automate::gateway::GuildIntegrationsUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_guild_integrations_update(mut self, listener: GuildIntegrationsUpdate) -> Self {
        self.config.listeners.guild_integrations_update.push(listener);
        self
    }

    /// Registers an event that listens to [GuildMemberAdd](automate::gateway::GuildMemberAddDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_guild_member_add(mut self, listener: GuildMemberAdd) -> Self {
        self.config.listeners.guild_member_add.push(listener);
        self
    }

    /// Registers an event that listens to [GuildMemberRemove](automate::gateway::GuildMemberRemoveDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_guild_member_remove(mut self, listener: GuildMemberRemove) -> Self {
        self.config.listeners.guild_member_remove.push(listener);
        self
    }

    /// Registers an event that listens to [GuildMemberUpdate](automate::gateway::GuildMemberUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_guild_member_update(mut self, listener: GuildMemberUpdate) -> Self {
        self.config.listeners.guild_member_update.push(listener);
        self
    }

    /// Registers an event that listens to [GuildMembersChunk](automate::gateway::GuildMembersChunkDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_guild_members_chunk(mut self, listener: GuildMembersChunk) -> Self {
        self.config.listeners.guild_members_chunk.push(listener);
        self
    }

    /// Registers an event that listens to [GuildRoleCreate](automate::gateway::GuildRoleCreateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_guild_role_create(mut self, listener: GuildRoleCreate) -> Self {
        self.config.listeners.guild_role_create.push(listener);
        self
    }

    /// Registers an event that listens to [GuildRoleUpdate](automate::gateway::GuildRoleUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_guild_role_update(mut self, listener: GuildRoleUpdate) -> Self {
        self.config.listeners.guild_role_update.push(listener);
        self
    }

    /// Registers an event that listens to [GuildRoleDelete](automate::gateway::GuildRoleDeleteDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_guild_role_delete(mut self, listener: GuildRoleDelete) -> Self {
        self.config.listeners.guild_role_delete.push(listener);
        self
    }

    /// Registers an event that listens to [InviteCreate](automate::gateway::InviteCreateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_invite_create(mut self, listener: InviteCreate) -> Self {
        self.config.listeners.invite_create.push(listener);
        self
    }

    /// Registers an event that listens to [InviteDelete](automate::gateway::InviteDeleteDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_invite_delete(mut self, listener: InviteDelete) -> Self {
        self.config.listeners.invite_delete.push(listener);
        self
    }

    /// Registers an event that listens to [MessageCreate](automate::gateway::MessageCreateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_message_create(mut self, listener: MessageCreate) -> Self {
        self.config.listeners.message_create.push(listener);
        self
    }

    /// Registers an event that listens to [MessageUpdate](automate::gateway::MessageUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_message_update(mut self, listener: MessageUpdate) -> Self {
        self.config.listeners.message_update.push(listener);
        self
    }

    /// Registers an event that listens to [MessageDelete](automate::gateway::MessageDeleteDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_message_delete(mut self, listener: MessageDelete) -> Self {
        self.config.listeners.message_delete.push(listener);
        self
    }

    /// Registers an event that listens to [MessageDeleteBulk](automate::gateway::MessageDeleteBulkDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_message_delete_bulk(mut self, listener: MessageDeleteBulk) -> Self {
        self.config.listeners.message_delete_bulk.push(listener);
        self
    }

    /// Registers an event that listens to [MessageReactionAdd](automate::gateway::MessageReactionAddDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_reaction_add(mut self, listener: MessageReactionAdd) -> Self {
        self.config.listeners.reaction_add.push(listener);
        self
    }

    /// Registers an event that listens to [MessageReactionRemove](automate::gateway::MessageReactionRemoveDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_reaction_remove(mut self, listener: MessageReactionRemove) -> Self {
        self.config.listeners.reaction_remove.push(listener);
        self
    }

    /// Registers an event that listens to [MessageReactionRemoveAll](automate::gateway::MessageReactionRemoveAllDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_reaction_remove_all(mut self, listener: MessageReactionRemoveAll) -> Self {
        self.config.listeners.reaction_remove_all.push(listener);
        self
    }

    /// Registers an event that listens to [PresenceUpdate](automate::gateway::PresenceUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_presence_update(mut self, listener: PresenceUpdate) -> Self {
        self.config.listeners.presence_update.push(listener);
        self
    }

    /// Registers an event that listens to [TypingStart](automate::gateway::TypingStartDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_typing_start(mut self, listener: TypingStart) -> Self {
        self.config.listeners.typing_start.push(listener);
        self
    }

    /// Registers an event that listens to [UserUpdate](automate::gateway::UserUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_user_update(mut self, listener: UserUpdate) -> Self {
        self.config.listeners.user_update.push(listener);
        self
    }

    /// Registers an event that listens to [VoiceStateUpdate](automate::gateway::VoiceStateUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_voice_state_update(mut self, listener: VoiceStateUpdate) -> Self {
        self.config.listeners.voice_state_update.push(listener);
        self
    }

    /// Registers an event that listens to [VoiceServerUpdate](automate::gateway::VoiceServerUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_voice_server_update(mut self, listener: VoiceServerUpdate) -> Self {
        self.config.listeners.voice_server_update.push(listener);
        self
    }

    /// Registers an event that listens to [WebhooksUpdate](automate::gateway::WebhooksUpdateDispatch) events
    #[deprecated(since = "0.3.0", note = "Please use `Configuration::register` instead")]
    pub fn on_webhooks_update(mut self, listener: WebhooksUpdate) -> Self {
        self.config.listeners.webhooks_update.push(listener);
        self
    }

    /// Asynchronous function setup the connection
    /// with Discord.
    /// Will block forever unless the bot crashes.
    #[deprecated(since = "0.3.1", note = "Please use a `ShardManager` or `Automate::launch` instead")]
    pub async fn connect(self) {
        let http = HttpAPI::new(&self.config.token);
        let gateway_bot = http.gateway_bot().await.expect("Failed to get gateway information from Discord");

        GatewayAPI::connect(self.config, gateway_bot.url).await
    }

    /// Non asynchronous equivalent for the connect
    /// function to setup the connection with discord.
    /// Creates a tokio runtime.
    ///
    /// Will block forever unless the bot crashes.
    #[deprecated(since = "0.3.1", note = "Please use a `ShardManager` or `Automate::launch` instead")]
    pub fn connect_blocking(self) {
        Runtime::new().unwrap().block_on(self.connect())
    }
}
