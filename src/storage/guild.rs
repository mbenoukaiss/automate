use std::collections::HashMap;
use crate::Snowflake;
use crate::gateway::Guild;
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
        self.find(id).unwrap()
    }

    pub fn find(&self, id: Snowflake) -> Option<&Guild> {
        self.guilds.get(&id)
    }

    pub fn find_by<P>(&self, mut filter: P) -> Vec<&Guild>
        where P: FnMut(&Guild) -> bool {
        self.guilds.values()
            .filter(|u| filter(u))
            .collect()
    }

    pub fn find_one_by<P>(&self, mut filter: P) -> Option<&Guild>
        where P: FnMut(&Guild) -> bool {
        for guild in self.guilds.values() {
            if filter(guild) {
                return Some(guild);
            }
        }

        None
    }

    pub fn insert(&mut self, guild: &Guild) {
        self.guilds.insert(guild.id, Clone::clone(guild));
    }
}