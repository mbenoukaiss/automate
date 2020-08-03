use std::collections::HashMap;
use crate::Snowflake;
use crate::gateway::PrivateChannel;
use crate::storage::{Stored, Storage};

#[derive(Default, Debug, Clone)]
pub struct PrivateChannelStorage {
    channels: HashMap<Snowflake, PrivateChannel>
}

impl Storage for PrivateChannelStorage {}

impl Stored for PrivateChannel {
    type Storage = PrivateChannelStorage;
}

impl PrivateChannelStorage {
    pub fn all(&self) -> Vec<&PrivateChannel> {
        self.channels.values().collect()
    }

    pub fn get(&self, id: Snowflake) -> &PrivateChannel {
        self.find(id).unwrap()
    }

    pub fn find(&self, id: Snowflake) -> Option<&PrivateChannel> {
        self.channels.get(&id)
    }

    pub fn find_by<P>(&self, mut filter: P) -> Vec<&PrivateChannel>
        where P: FnMut(&PrivateChannel) -> bool {
        self.channels.values()
            .filter(|u| filter(u))
            .collect()
    }

    pub fn find_one_by<P>(&self, mut filter: P) -> Option<&PrivateChannel>
        where P: FnMut(&PrivateChannel) -> bool {
        for channel in self.channels.values() {
            if filter(channel) {
                return Some(channel);
            }
        }

        None
    }

    pub fn insert(&mut self, channel: PrivateChannel) {
        self.channels.insert(channel.id, channel);
    }

    pub fn remove(&mut self, channel: &Snowflake) {
        self.channels.remove(channel);
    }
}