use std::collections::HashMap;
use crate::{Snowflake, Identifiable};
use crate::gateway::*;
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
        self.get_opt(id).unwrap()
    }

    pub fn get_opt(&self, id: Snowflake) -> Option<&Channel> {
        self.channels.get(&id)
    }

    pub(crate) fn insert(&mut self, channel: Channel) {
        self.channels.insert(channel.id(), channel);
    }

    pub(crate) fn remove(&mut self, channel: Snowflake) {
        self.channels.remove(&channel);
    }
}