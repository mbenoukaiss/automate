use std::collections::HashMap;
use crate::{Snowflake, Identifiable};
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

    #[inline]
    pub fn get(&self, id: Snowflake) -> &PrivateChannel {
        self.get_opt(id).unwrap()
    }

    pub fn get_opt(&self, id: Snowflake) -> Option<&PrivateChannel> {
        self.channels.get(&id)
    }

    pub fn insert(&mut self, channel: PrivateChannel) {
        self.channels.insert(channel.id(), channel);
    }

    pub fn remove(&mut self, channel: Snowflake) {
        self.channels.remove(&channel);
    }
}