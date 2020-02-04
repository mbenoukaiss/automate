use crate::Snowflake;
use crate::gateway::{VerificationLevel, MessageNotificationLevel, ExplicitContentFilterLevel, User};
use crate::http::NewChannel;

#[object(client)]
pub struct NewGuild {
    pub name: String,
    pub region: String,
    pub icon: String,
    pub verification_level: VerificationLevel,
    pub default_message_notifications: MessageNotificationLevel,
    pub explicit_content_filter: ExplicitContentFilterLevel,
    pub roles: Vec<NewRole>,
    pub channels: Vec<NewChannel>,
}

#[object(client, default)]
pub struct ModifyGuild {
    pub name: Option<String>,
    pub region: Option<String>,
    pub icon: Option<String>,
    pub verification_level: Option<VerificationLevel>,
    pub default_message_notifications: Option<MessageNotificationLevel>,
    pub explicit_content_filter: Option<ExplicitContentFilterLevel>,
    pub afk_channel_id: Option<Snowflake>,
    pub afk_timeout: Option<i32>,
    pub owner_id: Option<Snowflake>,
    pub splash: Option<String>,
    pub banner: Option<String>,
    pub system_channel_id: Option<Snowflake>,
}

#[object(client)]
pub struct NewRole {
    pub name: String,
    pub color: i32,
    pub hoist: bool,
    pub position: i32,
    pub permissions: u32,
    pub managed: bool,
    pub mentionable: bool,
}

#[object(client)]
pub struct ModifyRole {
    pub name: Option<String>,
    pub color: Option<i32>,
    pub hoist: Option<bool>,
    pub permissions: Option<u32>,
    pub mentionable: Option<bool>,
}

#[object(client)]
pub struct MoveRole {
    pub id: u64,
    pub position: i32,
}

#[object(client)]
pub struct NewEmoji {
    pub name: String,
    pub image: String,
    pub roles: Vec<Snowflake>,
}

#[object(client)]
pub struct UpdateEmoji {
    pub id: Snowflake,
    pub image: String,
    pub roles: Vec<Snowflake>,
}

pub enum MemberFilter {
    Default,
    Limit(i32),
    After(Snowflake, i32),
}

#[object(client, default)]
pub struct ModifyMember {
    pub nick: Option<String>,
    pub roles: Option<Vec<Snowflake>>,
    pub mute: Option<bool>,
    pub deaf: Option<bool>,
    pub channel_id: Option<u64>,
}

#[object(client, default)]
pub struct NewBan {
    pub reason: Option<String>,
    pub delete_message_days: Option<i8>,
}

#[object(server)]
pub struct Prune {
    pub pruned: i32,
}

#[object(server)]
pub struct Integration {
    pub id: Snowflake,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub enabled: bool,
    pub syncing: bool,
    pub role_id: Snowflake,
    pub expire_behavior: i32,
    pub expire_grace_period: i32,
    pub user: User,
    pub account: IntegrationAccount,
    pub synced_at: String,
}

#[object(server)]
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}

#[object(both)]
pub struct GuildEmbed {
    pub enabled: bool,
    pub channel_id: Option<Snowflake>,
}
