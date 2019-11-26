use automate::{Error, Discord};
use std::env;

fn main() -> Result<!, Error> {
    automate::setup_logging();

    Discord::new(&env::var("DISCORD_API_TOKEN").expect("API token not found"))
        .connect_blocking()
}
