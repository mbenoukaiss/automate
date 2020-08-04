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

    pub fn category(&self, id: Snowflake) -> &Category {
        match self.get_opt(id).unwrap() {
            Channel::Category(channel) => channel,
            _ => panic!("Given channel is not a category channel")
        }
    }

    pub fn text(&self, id: Snowflake) -> &TextChannel {
        match self.get_opt(id).unwrap() {
            Channel::Text(channel) => channel,
            _ => panic!("Given channel is not a text channel")
        }
    }

    pub fn voice(&self, id: Snowflake) -> &VoiceChannel {
        match self.get_opt(id).unwrap() {
            Channel::Voice(channel) => channel,
            _ => panic!("Given channel is not a voice channel")
        }
    }

    pub fn news(&self, id: Snowflake) -> &NewsChannel {
        match self.get_opt(id).unwrap() {
            Channel::News(channel) => channel,
            _ => panic!("Given channel is not a news channel")
        }
    }

    pub fn store(&self, id: Snowflake) -> &StoreChannel {
        match self.get_opt(id).unwrap() {
            Channel::Store(channel) => channel,
            _ => panic!("Given channel is not a store channel")
        }
    }

    pub fn direct(&self, id: Snowflake) -> &DirectChannel {
        match self.get_opt(id).unwrap() {
            Channel::Direct(channel) => channel,
            _ => panic!("Given channel is not a direct channel")
        }
    }

    pub fn group(&self, id: Snowflake) -> &GroupChannel {
        match self.get_opt(id).unwrap() {
            Channel::Group(channel) => channel,
            _ => panic!("Given channel is not a group channel")
        }
    }

    pub fn get_opt(&self, id: Snowflake) -> Option<&Channel> {
        self.channels.get(&id)
    }

    pub(crate) fn get_mut(&mut self, id: Snowflake) -> Option<&mut Channel> {
        self.channels.get_mut(&id)
    }

    pub(crate) fn insert(&mut self, channel: Channel) {
        self.channels.insert(channel.id(), channel);
    }

    pub(crate) fn remove(&mut self, channel: Snowflake) {
        self.channels.remove(&channel);
    }
}