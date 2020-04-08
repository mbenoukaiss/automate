use crate::{HttpAPI, Snowflake, Configuration, logger};
use crate::gateway::GatewayAPI;
use futures::future::join_all;
use tokio::task::JoinHandle;
use std::time::Duration;

pub struct ShardManager {
    config: Configuration,
    total_shards: i32,
    recommended_shards: i32,
    gateway_url: String,
    managed_shards: Vec<JoinHandle<()>>,
}

impl ShardManager {
    pub async fn with_config(config: Configuration) -> ShardManager {
        let http = HttpAPI::new(&config.token);
        let gateway_bot = http.gateway_bot().await.expect("Failed to get gateway information from Discord");

        ShardManager {
            config,
            total_shards: gateway_bot.shards,
            recommended_shards: gateway_bot.shards,
            gateway_url: gateway_bot.url,
            managed_shards: Vec::new(),
        }
    }

    pub fn set_total_shards(&mut self, total_shards: i32) -> &mut Self {
        if !self.managed_shards.is_empty() {
            panic!("Changing total shards count after a shard has been launched is forbidden");
        }

        self.total_shards = total_shards;
        self
    }

    pub fn setup(&mut  self, shard_id: i32) -> &mut Self {
        let url = self.gateway_url.clone();
        let mut config = self.config.clone();
        config.shard(shard_id, self.total_shards);

        let position = self.managed_shards.len();

        let handle = tokio::spawn(async move {
            //there must be at least 5 seconds between each identify call
            //so wait 5.5 seconds to make sure we don't hit rate limit
            tokio::time::delay_for(Duration::from_millis(position as u64 * 5500)).await;

            info!("Establishing connection with gateway for shard {}", shard_id);

            GatewayAPI::connect(config, url).await
        });

        self.managed_shards.push(handle);
        self
    }

    pub async fn launch(&mut self) {
        if self.config.logging {
            logger::__internal_setup_logging(self.config.log_level.clone());
        }

        let mut shards = Vec::new();
        shards.append(&mut self.managed_shards);

        for result in join_all(shards).await {
            if let Err(err) = result {
                error!("Failed to join shard: {}", err);
            }
        }
    }

    pub fn recommended_shards(&self) -> i32 {
        self.recommended_shards
    }

    pub fn total_shards(&self) -> i32 {
        self.total_shards
    }

    pub fn shard_id(&self, guild_id: &Snowflake) -> i32 {
        ((guild_id.0 >> 22) % self.total_shards as u64) as i32
    }

}

