use crate::gateway::{User, PartialUser, PartialGuild};
use crate::Snowflake;
use std::collections::HashMap;
use crate::snowflake::Identifiable;

#[object(server)]
#[serde(untagged)]
pub enum AnyChannel {
    Guild(Channel),
    Private(PrivateChannel)
}

#[object(server)]
pub struct Channel {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub _type: ChannelType,
    pub guild_id: Option<Snowflake>,
    pub position: i32,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: String,
    #[nullable]
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    #[option_nullable]
    pub last_message_id: Option<Option<Snowflake>>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    #[option_nullable]
    pub icon: Option<Option<String>>,
    #[nullable]
    pub parent_id: Option<Snowflake>,
    pub last_pin_timestamp: Option<String>,
}

impl Identifiable for Channel {
    fn id(&self) -> Snowflake {
        self.id
    }
}

#[object(server)]
pub struct PrivateChannel {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub _type: ChannelType,
    pub name: Option<String>,
    #[option_nullable]
    pub last_message_id: Option<Option<Snowflake>>,
    #[serde(deserialize_with = "automate::encode::json::as_hashmap")]
    pub recipients: HashMap<Snowflake, User>,
    #[nullable]
    pub icon: Option<String>,
    pub owner_id: Snowflake,
    pub application_id: Option<Snowflake>,
    pub last_pin_timestamp: Option<String>,
}

#[object(server)]
pub struct Invite {
    pub code: String,
    pub guild: Option<PartialGuild>,
    pub channel: Channel,
    pub inviter: Option<User>,
    pub target_user: Option<PartialUser>,
    pub target_user_type: Option<i32>,
    pub approximate_presence_count: Option<i32>,
    pub approximate_member_count: Option<i32>,

    //following variables are extra information
    pub uses: Option<i32>,
    pub max_uses: Option<i32>,
    pub max_age: Option<i32>,
    pub temporary: Option<bool>,
    pub created_at: Option<String>
}

#[object(server)]
pub struct PartialInvite {
    pub code: Option<String>,
    pub guild: Option<PartialGuild>,
    pub channel: Option<Channel>,
    pub inviter: Option<User>,
    pub target_user: Option<PartialUser>,
    pub target_user_type: Option<i32>,
    pub approximate_presence_count: Option<i32>,
    pub approximate_member_count: Option<i32>,

    //following variables are extra information
    pub uses: Option<i32>,
    pub max_uses: Option<i32>,
    pub max_age: Option<i32>,
    pub temporary: Option<bool>,
    pub created_at: Option<String>
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
    pub id: Snowflake,
    #[serde(rename = "type")]
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
    pub id: Snowflake,
    pub guild_id: Snowflake,
    #[serde(rename = "type")]
    pub _type: ChannelType,
    pub name: String
}

#[object(server)]
pub struct Webhook {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub _type: WebhookType,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    pub user: Option<User>,
    #[option_nullable]
    pub name: Option<Option<String>>,
    #[option_nullable]
    pub avatar: Option<Option<String>>,
    pub token: Option<String>,
}

#[convert(u8)]
pub enum WebhookType {
    Incoming = 1,
    ChannelFollower = 2
}