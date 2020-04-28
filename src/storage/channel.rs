use std::collections::HashMap;
use crate::Snowflake;
use crate::gateway::Channel;
use crate::storage::{Stored, StorageContainer, Storage};

#[derive(Default)]
pub struct ChannelStorage {
    channels: HashMap<Snowflake, Channel>
}

impl Stored for Channel {
    type Storage = ChannelStorage;

    fn read(container: &StorageContainer) -> &Self::Storage {
        &container.channel_storage
    }

    fn write(container: &mut StorageContainer) -> &mut Self::Storage {
        &mut container.channel_storage
    }
}

impl Storage for ChannelStorage {
    type Stored = Channel;

    fn get(&self, id: Snowflake) -> &Self::Stored {
        self.find(id).unwrap()
    }

    fn find(&self, id: Snowflake) -> Option<&Self::Stored> {
        self.channels.get(&id)
    }

    fn find_by<P>(&self, mut filter: P) -> Vec<&Self::Stored>
        where P: FnMut(&Self::Stored) -> bool {
        self.channels.values()
            .filter(|u| filter(u))
            .collect()
    }

    fn find_one_by<P>(&self, mut filter: P) -> Option<&Self::Stored>
        where P: FnMut(&Self::Stored) -> bool {
        for channel in self.channels.values() {
            if filter(channel) {
                return Some(channel);
            }
        }

        None
    }

    fn insert(&mut self, val: Self::Stored) {
        self.channels.insert(val.id, val);
    }
}