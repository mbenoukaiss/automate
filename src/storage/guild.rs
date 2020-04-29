use std::collections::HashMap;
use crate::Snowflake;
use crate::gateway::Guild;
use crate::storage::{Stored, Storage};

#[derive(Default)]
pub struct GuildStorage {
    guilds: HashMap<Snowflake, Guild>
}

impl Stored for Guild {
    type Storage = GuildStorage;
}

impl Storage for GuildStorage {
    type Key = Snowflake;
    type Stored = Guild;

    fn get(&self, id: &Self::Key) -> &Self::Stored {
        self.find(id).unwrap()
    }

    fn find(&self, id: &Self::Key) -> Option<&Self::Stored> {
        self.guilds.get(&id)
    }

    fn insert(&mut self, key: &Self::Key, val: &Self::Stored) {
        self.guilds.insert((*key).clone(), (*val).clone());
    }
}

impl GuildStorage {
    fn find_by<P>(&self, mut filter: P) -> Vec<&Guild>
        where P: FnMut(&Guild) -> bool {
        self.guilds.values()
            .filter(|u| filter(u))
            .collect()
    }

    fn find_one_by<P>(&self, mut filter: P) -> Option<&Guild>
        where P: FnMut(&Guild) -> bool {
        for guild in self.guilds.values() {
            if filter(guild) {
                return Some(guild);
            }
        }

        None
    }
}