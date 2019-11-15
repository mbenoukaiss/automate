use crate::json::Nullable;
use crate::models::User;

#[object(both)]
pub struct Channel {
    pub id: u64,
    pub _type: ChannelType,
    pub guild_id: Option<u64>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,
    pub topic: Option<Nullable<String>>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<Nullable<u64>>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<Nullable<String>>,
    pub owner_id: Option<u64>,
    pub application_id: Option<u64>,
    pub parent_id: Option<Nullable<u64>>,
    pub last_pin_timestamp: Option<String>,
}

#[object(client)]
pub struct ModifyChannel {
    pub name: Option<String>,
    pub position: Option<i32>,
    pub topic: Option<Nullable<String>>,
    pub nsfw: Option<bool>,
    pub rate_limit_per_user: Option<i32>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub parent_id: Option<Nullable<u64>>,
}

#[object(client)]
pub struct GetChannelMessages {
    pub around: Option<u64>,
    pub before: Option<u64>,
    pub after: Option<u64>,
    pub limit: Option<i32>,
}

#[convert(u8)]
pub enum ChannelType {
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GroupDM = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6
}

#[object(both)]
pub struct Overwrite {
    pub id: u64,
    pub _type: OverwriteType,
    pub allow: u32,
    pub deny: u32
}

#[stringify(snake_case)]
pub enum OverwriteType {
    Role,
    Member
}

#[object(both)]
pub struct ChannelMention {
    pub id: u64,
    pub guild_id: u64,
    pub _type: ChannelType,
    pub name: String
}