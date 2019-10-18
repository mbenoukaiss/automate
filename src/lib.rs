#![feature(test)]
#![feature(try_blocks)]
#![feature(never_type)]
#![allow(clippy::identity_op)]

extern crate self as automatea;
extern crate test;
#[macro_use] extern crate automatea_proc;
#[macro_use] extern crate log;

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

pub fn setup_logging() -> Result<(), Error> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{date} in {target} [{level}]: {message}",
                date = chrono::Local::now().format("%H:%M:%S"),
                target = record.target(),
                level = record.level(),
                message = message
            ))
        })
        .level(log::LevelFilter::Warn)
        .level_for("automatea", log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}
