use std::collections::HashMap;
use crate::Snowflake;
use crate::gateway::Channel;
use crate::storage::{Stored, Storage};

#[derive(Default, Debug, Clone)]
pub struct ChannelStorage {
    channels: HashMap<Snowflake, Channel>
}

impl Storage for ChannelStorage {}

impl Stored for Channel {
    type Storage = ChannelStorage;
}

impl ChannelStorage {
    pub fn all(&self) -> Vec<&Channel> {
        self.channels.values().collect()
    }

    pub fn get(&self, id: Snowflake) -> &Channel {
        self.find(id).unwrap()
    }

    pub fn find(&self, id: Snowflake) -> Option<&Channel> {
        self.channels.get(&id)
    }

    pub fn find_by<P>(&self, mut filter: P) -> Vec<&Channel>
        where P: FnMut(&Channel) -> bool {
        self.channels.values()
            .filter(|u| filter(u))
            .collect()
    }

    pub fn find_one_by<P>(&self, mut filter: P) -> Option<&Channel>
        where P: FnMut(&Channel) -> bool {
        for channel in self.channels.values() {
            if filter(channel) {
                return Some(channel);
            }
        }

        None
    }

    pub fn insert(&mut self, channel: &Channel) {
        self.channels.insert(channel.id, Clone::clone(channel));
    }

    pub fn remove(&mut self, channel: &Channel) {
        self.channels.remove(&channel.id);
    }
}