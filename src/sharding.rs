use crate::{HttpAPI, Snowflake, Configuration, logger, Error};
use crate::gateway::GatewayAPI;
use futures::future;
use std::time::Duration;

/// Helps setting up a bot with multiple shards.
///
/// Using this struct is probably not necessary and
/// [Automate::launch](automate::Automate::launch)
/// will do the work.
/// The only reason to use this instead of
/// [Automate::launch](automate::Automate::launch)
/// would be if you want to spread the shards across
/// multiple servers or if you wanted to have more
/// or less shards than the amount recommended
/// by Discord.
pub struct ShardManager {
    config: Configuration,
    total_shards: u32,
    recommended_shards: u32,
    gateway_url: String,
    managed_shards: Vec<u32>,
}

impl ShardManager {
    /// Creates a shard manager where all the shards
    /// will use the given config.
    pub async fn with_config(config: Configuration) -> Result<ShardManager, Error> {
        let http = HttpAPI::new(&config.token);
        let gateway_bot = http.gateway_bot().await?;

        Ok(ShardManager {
            config,
            total_shards: gateway_bot.shards,
            recommended_shards: gateway_bot.shards,
            gateway_url: gateway_bot.url,
            managed_shards: Vec::new(),
        })
    }

    pub fn setup(&mut self, shard_id: u32) {
        self.managed_shards.push(shard_id);
    }

    /// Sets up as many shards as Discord recommends.
    pub fn auto_setup(&mut self) -> &mut Self {
        for i in 0..self.recommended_shards {
            self.setup(i);
        }

        self
    }

    /// Launches all the previously set up shards
    pub async fn launch(&mut self) {
        if self.config.logging {
            logger::__internal_setup_logging(self.config.log_levels.clone());
        }

        if self.recommended_shards > self.total_shards {
            warn!("Discord recommends using {} shards, you should use at least this many shards", self.recommended_shards);
        }

        let mut handles = Vec::new();

        for (position, shard_id) in self.managed_shards.iter().enumerate() {
            let shard_id = *shard_id;

            let url = self.gateway_url.clone();
            let mut config = self.config.clone();
            config.shard(shard_id, self.total_shards);

            let handle = tokio::spawn(async move {
                //there must be at least 5 seconds between each identify call
                //so wait 5.5 seconds to make sure we don't hit rate limit
                tokio::time::sleep(Duration::from_millis(position as u64 * 5500)).await;

                automate::logger::setup_for_task(format!("shard-{}", shard_id),  async move {
                    GatewayAPI::connect(config, url).await
                }).await
            });

            handles.push(handle);
        }

        for result in future::join_all(handles).await {
            if let Err(err) = result {
                error!("Failed to join shard: {}", err);
            }
        }
    }

    /// The amount of shards recommended by discord
    pub fn recommended_shards(&self) -> u32 {
        self.recommended_shards
    }

    /// The amount of shards this bot will have.
    /// Defaults to the recommended shards value.
    pub fn total_shards(&self) -> u32 {
        self.total_shards
    }

    /// Sets the amount of shards the bot will have.
    pub fn set_total_shards(&mut self, total_shards: u32) -> &mut Self {
        if !self.managed_shards.is_empty() {
            panic!("Changing total shards count after a shard has been launched is not possible");
        }

        self.total_shards = total_shards;
        self
    }
}

/// Calculates the id of the shard a guild will
/// be handled by.
pub fn shard_id(total_shards: u64, guild_id: Snowflake) -> u32 {
    ((guild_id.0 >> 22) % total_shards) as u32
}
