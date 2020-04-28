use std::collections::HashMap;
use crate::Snowflake;
use crate::gateway::Guild;
use crate::storage::{Stored, StorageContainer, Storage};

#[derive(Default)]
pub struct GuildStorage {
    guilds: HashMap<Snowflake, Guild>
}

impl Stored for Guild {
    type Storage = GuildStorage;

    fn read(container: &StorageContainer) -> &Self::Storage {
        &container.guild_storage
    }

    fn write(container: &mut StorageContainer) -> &mut Self::Storage {
        &mut container.guild_storage
    }
}

impl Storage for GuildStorage {
    type Stored = Guild;

    fn get(&self, id: Snowflake) -> &Self::Stored {
        self.find(id).unwrap()
    }

    fn find(&self, id: Snowflake) -> Option<&Self::Stored> {
        self.guilds.get(&id)
    }

    fn find_by<P>(&self, mut filter: P) -> Vec<&Self::Stored>
        where P: FnMut(&Self::Stored) -> bool {
        self.guilds.values()
            .filter(|u| filter(u))
            .collect()
    }

    fn find_one_by<P>(&self, mut filter: P) -> Option<&Self::Stored>
        where P: FnMut(&Self::Stored) -> bool {
        for guild in self.guilds.values() {
            if filter(guild) {
                return Some(guild);
            }
        }

        None
    }

    fn insert(&mut self, val: Self::Stored) {
        self.guilds.insert(val.id, val);
    }
}