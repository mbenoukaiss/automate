use std::collections::HashMap;
use crate::Snowflake;
use crate::gateway::*;
use crate::storage::{Stored, Storage};

#[derive(Default, Debug, Clone)]
pub struct GuildMemberStorage {
    guilds: HashMap<(Snowflake, Snowflake), GuildMember>
}

impl Storage for GuildMemberStorage {}

impl Stored for GuildMember {
    type Storage = GuildMemberStorage;
}

impl GuildMemberStorage {
    pub fn all(&self) -> Vec<&GuildMember> {
        self.guilds.values().collect()
    }

    pub fn get(&self, guild: &Snowflake, member: &Snowflake) -> &GuildMember {
        self.find(guild, member).unwrap()
    }

    pub fn find(&self, guild: &Snowflake, member: &Snowflake) -> Option<&GuildMember> {
        self.guilds.get(&(*guild, *member))
    }

    pub fn find_by<P>(&self, mut filter: P) -> Vec<&GuildMember>
        where P: FnMut(&GuildMember) -> bool {
        self.guilds.values()
            .filter(|u| filter(u))
            .collect()
    }

    pub fn find_one_by<P>(&self, mut filter: P) -> Option<&GuildMember>
        where P: FnMut(&GuildMember) -> bool {
        for guild in self.guilds.values() {
            if filter(guild) {
                return Some(guild);
            }
        }

        None
    }

    pub(crate) fn insert(&mut self, guild: Snowflake, member: GuildMember) {
        self.guilds.insert((guild, member.user.id), member);
    }

    pub(crate) fn remove(&mut self, guild: &Snowflake, member: &Snowflake) {
        self.guilds.remove(&(*guild, *member));
    }
}