use std::collections::HashMap;
use crate::Snowflake;
use crate::gateway::*;
use crate::storage::{Stored, Storage};

#[derive(Default, Debug, Clone)]
pub struct GuildStorage {
    guilds: HashMap<Snowflake, Guild>
}

impl Storage for GuildStorage {}

impl Stored for Guild {
    type Storage = GuildStorage;
}

impl GuildStorage {
    pub fn all(&self) -> Vec<&Guild> {
        self.guilds.values().collect()
    }

    pub fn get(&self, id: Snowflake) -> &Guild {
        self.get_opt(id).unwrap()
    }

    pub fn get_opt(&self, id: Snowflake) -> Option<&Guild> {
        self.guilds.get(&id)
    }

    pub(crate) fn get_mut(&mut self, id: Snowflake) -> &mut Guild {
        self.guilds.get_mut(&id).unwrap()
    }

    pub(crate) fn insert(&mut self, guild: Guild) {
        self.guilds.insert(guild.id, guild);
    }

    pub(crate) fn remove(&mut self, guild: Snowflake) {
        self.guilds.remove(&guild);
    }
}