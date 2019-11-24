use crate::json::Nullable;
use crate::gateway::{User, PartialUser, PartialGuild};

#[object(server)]
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

#[object(server)]
pub struct Invite {
    pub code: String,
    pub guild: Option<PartialGuild>,
    pub channel: Channel,
    pub target_user: Option<PartialUser>,
    pub target_user_type: Option<i32>,
    pub approximate_presence_count: Option<i32>,
    pub approximate_member_count: Option<i32>,
}

pub struct InviteMetadata {
    pub max_age: i32,
    pub max_uses: i32,
    pub temporary: bool,
    pub unique: bool
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

#[object(server)]
pub struct Webhook {
    pub id: u64,
    pub _type: WebhookType,
    pub guild_id: Option<u64>,
    pub channel_id: u64,
    pub user: Option<User>,
    pub name: Nullable<String>,
    pub avatar: Nullable<String>,
    pub token: Option<String>,
}

#[convert(u8)]
pub enum WebhookType {
    Incoming = 1,
    ChannelFollower = 2
}