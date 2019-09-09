use automatea::{GatewayClient, AutomateaError};

fn main() -> Result<(), AutomateaError> {
    automatea::setup_logging()?;
    let client = GatewayClient::connect()?;

    Ok(())
}