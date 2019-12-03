use crate::{Snowflake, Nullable};
use crate::gateway::{Emoji, PartialEmoji, VerificationLevel, MessageNotificationLevel, ExplicitContentFilterLevel};
use percent_encoding::NON_ALPHANUMERIC;
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

pub trait UrlEncode {
    fn encode(&self) -> String;
}

impl UrlEncode for &str {
    fn encode(&self) -> String {
        percent_encoding::percent_encode(self.as_bytes(), NON_ALPHANUMERIC).to_string()
    }
}

impl UrlEncode for String {
    fn encode(&self) -> String {
        percent_encoding::percent_encode(self.as_bytes(), NON_ALPHANUMERIC).to_string()
    }
}

impl UrlEncode for Emoji {
    fn encode(&self) -> String {
        let mut code = String::with_capacity(self.name.len());
        code.push_str(&percent_encoding::percent_encode(self.name.as_bytes(), NON_ALPHANUMERIC).to_string());

        if let Nullable::Value(id) = self.id {
            code.push(':');
            code.push_str(&id.to_string());
        }

        code
    }
}

impl UrlEncode for PartialEmoji {
    fn encode(&self) -> String {
        let mut code = String::with_capacity(self.name.len());
        code.push_str(&percent_encoding::percent_encode(self.name.as_bytes(), NON_ALPHANUMERIC).to_string());

        if let Some(nullable_id) = self.id {
            if let Nullable::Value(id) = nullable_id {
                code.push(':');
                code.push_str(&id.to_string());
            }
        }

        code
    }
}