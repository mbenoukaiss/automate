#![feature(test)]
#![feature(try_blocks)]
#![feature(async_closure)]
#![allow(where_clauses_object_safety)] //should be fixable when async traits are allowed
#![allow(clippy::identity_op)] //because clippy forbides 1 << 0 in c-like enums

extern crate self as automate;
extern crate test;
#[macro_use]
extern crate automate_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate tokio_tungstenite as tktungstenite;

pub mod http;
pub mod gateway;
pub mod encode;
mod snowflake;
mod macros;
mod errors;
mod logger;

pub use async_trait::async_trait;
pub use tokio;

pub use gateway::Listener;
pub use http::HttpAPI;
pub use gateway::{GatewayAPI, Session};
pub use logger::setup_logging;
pub use snowflake::Snowflake;
pub use errors::Error;

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
    listeners: Vec<Box<dyn Listener + Send>>,
}

impl Discord {
    /// Creates an instance of this struct
    /// with the provided token.
    /// The token can be generated on
    /// [Discord's developers portal](https://discordapp.com/developers/applications/)
    pub fn new(token: &str) -> Discord {
        Discord {
            http: HttpAPI::new(token),
            listeners: Vec::new(),
        }
    }

    /// Registers an event listener
    pub fn with_listener<L: Listener + Send + 'static>(mut self, listener: L) -> Self {
        self.listeners.push(Box::new(listener));
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
