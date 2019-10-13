#![feature(never_type)]

use automatea::{GatewayClient, AutomateaError};

fn main() -> Result<!, AutomateaError> {
    automatea::setup_logging()?;

    GatewayClient::connect()
}