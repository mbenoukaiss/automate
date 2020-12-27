//! Example that demonstrates the sharding API.

use automate::{Error, Configuration, Automate, ShardManager, Threading};

fn main() -> Result<(), Error> {
    let config = Configuration::from_env("DISCORD_API_TOKEN");

    Automate::block_on(Threading::Multi, async move {
        ShardManager::with_config(config).await?
            .set_total_shards(2)
            .auto_setup()
            .launch().await;

        Ok(())
    })
}
