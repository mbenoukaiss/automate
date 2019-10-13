#![feature(test)]
#![feature(try_blocks)]
#![feature(never_type)]
#![allow(clippy::identity_op)]

extern crate self as automatea;
extern crate test;
#[macro_use] extern crate log;
#[macro_use] extern crate automatea_proc;

mod json;
mod models;
mod gateway;
mod macros;
mod errors;

pub use gateway::GatewayClient;
pub use json::{AsJson, FromJson};
pub use errors::AutomateaError;

pub fn setup_logging() -> Result<(), AutomateaError> {
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
