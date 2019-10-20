#![feature(test)]
#![feature(try_blocks)]
#![feature(never_type)]
#![allow(clippy::identity_op)]

extern crate self as automatea;
extern crate test;
#[macro_use]
extern crate automatea_proc;
#[macro_use]
extern crate log;

mod json;
mod models;
mod http;
mod gateway;
mod macros;
mod errors;

pub use http::HttpAPI;
pub use gateway::GatewayAPI;
pub use json::{AsJson, FromJson};
pub use errors::Error;

use log::{Log, Metadata, Record, LevelFilter, Level};
use chrono::{Local, Timelike};

struct QuickLogger;

impl Log for QuickLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() == Level::Error || metadata.target().starts_with(env!("CARGO_PKG_NAME"))
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let time = Local::now();

        println!(
            "{:02}:{:02}:{:02} in {} [{}]: {}",
            time.hour(),
            time.minute(),
            time.second(),
            record.target(),
            record.level(),
            record.args()
        );
    }

    fn flush(&self) {}
}

static QUICK_LOGGER: QuickLogger = QuickLogger;

pub fn setup_logging() {
    log::set_logger(&QUICK_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);
}


