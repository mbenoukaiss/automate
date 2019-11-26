#![feature(test)]
#![feature(try_blocks)]
#![allow(where_clauses_object_safety)] //should be fixable when async traits are allowed
#![allow(clippy::identity_op)] //because clippy forbides 1 << 0 in c-like enums

extern crate self as automate;
extern crate test;
#[macro_use]
extern crate automate_derive;
#[macro_use]
extern crate log;

pub mod json;
pub mod http;
pub mod gateway;
mod snowflake;
mod events;
mod macros;
mod errors;
mod logger;

pub use async_trait::async_trait;
pub use tokio;

pub use events::Listener;
pub use http::HttpAPI;
pub use gateway::{GatewayAPI, Session};
pub use json::Nullable;
pub use logger::setup_logging;
pub use snowflake::Snowflake;
pub use errors::Error;

use tokio::runtime::Runtime;
use std::sync::{Mutex, Arc};
use std::thread::JoinHandle;
use std::thread;

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
///     .expect("Bot crashed")
/// ```
pub struct Discord {
    http: HttpAPI,
    listeners: Arc<Mutex<Vec<Box<dyn Listener + Send>>>>,
}

impl Discord {
    /// Creates an instance of this struct
    /// with the provided token.
    /// The token can be generated on
    /// [Discord's developers portal](https://discordapp.com/developers/applications/)
    pub fn new(token: &str) -> Discord {
        Discord {
            http: HttpAPI::new(token),
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Registers an event listener
    pub fn with_listener(self, listener: Box<dyn Listener + Send>) -> Self {
        self.listeners.lock().unwrap().push(listener);
        self
    }

    /// Asynchronous function setup the connection
    /// with Discord.
    /// Will block forever unless the bot crashes.
    pub async fn connect(self) -> Result<!, Error> {
        GatewayAPI::connect(self.http.clone(), self.listeners.clone()).await
    }

    /// Non asynchronous equivalent for the connect
    /// function to setup the connection with discord.
    /// Creates a tokio runtime.
    ///
    /// Will block forever unless the bot crashes.
    pub fn connect_blocking(self) -> Result<!, Error> {
        Runtime::new().unwrap().block_on(self.connect())
    }

    /// Non asynchronous equivalent for the connect
    /// function to setup the connection with discord.
    /// This function establishes the connection and runs
    /// the event loop in a separate thread whose
    /// [JoinHandle](std::thread::JoinHandle) is returned.
    pub fn connect_detached(self) -> JoinHandle<Result<!, Error>> {
        thread::spawn(move || {
            Runtime::new().unwrap().block_on(self.connect())
        })
    }
}
