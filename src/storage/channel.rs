use std::collections::HashMap;
use crate::Snowflake;
use crate::gateway::Channel;
use crate::storage::{Stored, Storage};

#[derive(Default)]
pub struct ChannelStorage {
    channels: HashMap<Snowflake, Channel>
}

impl Stored for Channel {
    type Storage = ChannelStorage;
}

impl Storage for ChannelStorage {
    type Key = Snowflake;
    type Stored = Channel;

    fn get(&self, id: &Self::Key) -> &Self::Stored {
        self.find(id).unwrap()
    }

    fn find(&self, id: &Self::Key) -> Option<&Self::Stored> {
        self.channels.get(&id)
    }

    fn insert(&mut self, key: &Self::Key, val: &Self::Stored) {
        self.channels.insert((*key).clone(), (*val).clone());
    }
}

impl ChannelStorage {
    fn find_by<P>(&self, mut filter: P) -> Vec<&Channel>
        where P: FnMut(&Channel) -> bool {
        self.channels.values()
            .filter(|u| filter(u))
            .collect()
    }

    fn find_one_by<P>(&self, mut filter: P) -> Option<&Channel>
        where P: FnMut(&Channel) -> bool {
        for channel in self.channels.values() {
            if filter(channel) {
                return Some(channel);
            }
        }

        None
    }
}