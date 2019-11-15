#![feature(test)]
#![feature(try_blocks)]
#![feature(never_type)]
#![allow(where_clauses_object_safety)] //should be fixable when async traits are allowed
#![allow(clippy::identity_op)] //because clippy forbides 1 << 0 in c-like enums

extern crate self as automate;
extern crate test;
#[macro_use]
extern crate automate_derive;
#[macro_use]
extern crate log;

pub mod models;
mod json;
mod events;
mod http;
mod gateway;
mod macros;
mod errors;
mod logger;

pub use async_trait::async_trait;
pub use tokio;

pub use events::Listener;
pub use http::HttpAPI;
pub use gateway::{GatewayAPI, Session};
pub use json::{AsJson, FromJson};
pub use logger::setup_logging;
pub use errors::Error;
use std::ops::Deref;
use std::sync::{Mutex, Arc};

pub struct Discord {
    http: HttpAPI,
    listeners: Arc<Mutex<Vec<Box<dyn Listener + Send>>>>,
}

impl Discord {
    pub fn new(token: &str) -> Discord {
        Discord {
            http: HttpAPI::new(token),
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn with_listener(self, listener: Box<dyn Listener + Send>) -> Self {
        self.listeners.lock().unwrap().push(listener);
        self
    }

    pub async fn connect(self) -> Result<!, Error> {
        GatewayAPI::connect(self.http.clone(), self.listeners.clone()).await
    }
}

impl Deref for Discord {
    type Target = HttpAPI;

    fn deref(&self) -> &Self::Target {
        &self.http
    }
}
