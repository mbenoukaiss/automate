use crate::Snowflake;
use crate::gateway::{VerificationLevel, MessageNotificationLevel, ExplicitContentFilterLevel};
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
    pub channels: Vec<NewChannel>

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
    pub mentionable: bool
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