use crate::gateway::{User, Webhook, Overwrite, ChannelType, PartialRole};
use crate::Nullable;
use crate::json::{self, FromJson, JsonError};

#[object(server)]
pub struct AuditLog {
    pub webhooks: Vec<Webhook>,
    pub users: Vec<User>,
    pub audit_log_entries: Vec<AuditLogEntry>,
}

#[object(server)]
pub struct AuditLogEntry {
    pub id: u64,
    pub target_id: Nullable<String>,
    pub changes: Option<Vec<AuditLogChange>>,
    pub user_id: u64,
    pub action_type: AuditLogEvent,
    pub options: Option<AuditEntryInfo>,
    pub reason: Option<String>,
}

#[convert(u8)]
pub enum AuditLogEvent {
    GuildUpdate = 1,
    ChannelCreate = 10,
    ChannelUpdate = 11,
    ChannelDelete = 12,
    ChannelOverwriteCreate = 13,
    ChannelOverwriteUpdate = 14,
    ChannelOverwriteDelete = 15,
    MemberKick = 20,
    MemberPrune = 21,
    MemberBanAdd = 22,
    MemberBanRemove = 23,
    MemberUpdate = 24,
    MemberRoleUpdate = 25,
    MemberMove = 26,
    MemberDisconnect = 27,
    BotAdd = 28,
    RoleCreate = 30,
    RoleUpdate = 31,
    RoleDelete = 32,
    InviteCreate = 40,
    InviteUpdate = 41,
    InviteDelete = 42,
    WebhookCreate = 50,
    WebhookUpdate = 51,
    WebhookDelete = 52,
    EmojiCreate = 60,
    EmojiUpdate = 61,
    EmojiDelete = 62,
    MessageDelete = 72,
    MessageBulkDelete = 73,
    MessagePin = 74,
    MessageUnpin = 75,
    IntegrationCreate = 80,
    IntegrationUpdate = 81,
    IntegrationDelete = 82
}

#[object(server)]
pub struct AuditEntryInfo {
    pub delete_member_days: Option<String>,
    pub members_removed: Option<String>,
    pub channel_id: Option<u64>,
    pub count: Option<i32>,
    pub id: Option<u64>,
    pub _type: Option<String>,
    pub role_name: Option<String>
}

#[derive(Debug)]
pub struct AuditLogChange {
    pub key: String,
    pub new_value: Option<AuditLogChangeValue>,
    pub old_value: Option<AuditLogChangeValue>,
}

#[derive(Debug)]
pub enum AuditLogChangeValue {
    String(String),
    Snowflake(u64),
    Integer(i32),
    Boolean(bool),
    Roles(Vec<PartialRole>),
    Overwrites(Vec<Overwrite>),
    Channel(ChannelType)
}

impl AuditLogChange {
    fn deserialize_value(key: &str, value: &str) -> Result<AuditLogChangeValue, JsonError> {
        match key {
            "id" => Ok(AuditLogChangeValue::Snowflake(u64::from_json(value)?)),
            "type" => if value.starts_with('"') && value.ends_with('"') {
                Ok(AuditLogChangeValue::String(String::from_json(value)?))
            } else {
                Ok(AuditLogChangeValue::Channel(ChannelType::from_json(value)?))
            }

            "name" => Ok(AuditLogChangeValue::String(String::from_json(value)?)),
            "icon_hash" => Ok(AuditLogChangeValue::String(String::from_json(value)?)),
            "splash_hash" => Ok(AuditLogChangeValue::String(String::from_json(value)?)),
            "owner_id" => Ok(AuditLogChangeValue::Snowflake(u64::from_json(value)?)),
            "region" => Ok(AuditLogChangeValue::String(String::from_json(value)?)),
            "afk_channel_id" => Ok(AuditLogChangeValue::Snowflake(u64::from_json(value)?)),
            "afk_timeout" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "mfa_level" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "verification_level" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "explicit_content_filter" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "default_message_notifications" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "vanity_code_url" => Ok(AuditLogChangeValue::String(String::from_json(value)?)),
            "$add" => Ok(AuditLogChangeValue::Roles(FromJson::from_json(value)?)),
            "$remove" => Ok(AuditLogChangeValue::Roles(FromJson::from_json(value)?)),
            "prune_delete_days" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "widget_enabled" => Ok(AuditLogChangeValue::Boolean(bool::from_json(value)?)),
            "widget_channel_id" => Ok(AuditLogChangeValue::Snowflake(u64::from_json(value)?)),
            "system_channel_id" => Ok(AuditLogChangeValue::Snowflake(u64::from_json(value)?)),

            "position" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "topic" => Ok(AuditLogChangeValue::String(String::from_json(value)?)),
            "bitrate" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "permission_overwrites" => Ok(AuditLogChangeValue::Overwrites(Vec::<Overwrite>::from_json(value)?)),
            "rate_limit_per_user" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "nsfw" => Ok(AuditLogChangeValue::Boolean(bool::from_json(value)?)),
            "application_id" => Ok(AuditLogChangeValue::Snowflake(u64::from_json(value)?)),

            "permissions" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "color" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "hoist" => Ok(AuditLogChangeValue::Boolean(bool::from_json(value)?)),
            "mentionable" => Ok(AuditLogChangeValue::Boolean(bool::from_json(value)?)),
            "allow" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "deny" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),

            "code" => Ok(AuditLogChangeValue::String(String::from_json(value)?)),
            "channel_id" => Ok(AuditLogChangeValue::Snowflake(u64::from_json(value)?)),
            "inviter_id" => Ok(AuditLogChangeValue::Snowflake(u64::from_json(value)?)),
            "max_uses" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "uses" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "max_age" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "age" => Ok(AuditLogChangeValue::Integer(i32::from_json(value)?)),
            "temporary" => Ok(AuditLogChangeValue::Boolean(bool::from_json(value)?)),

            "deaf" => Ok(AuditLogChangeValue::Boolean(bool::from_json(value)?)),
            "mute" => Ok(AuditLogChangeValue::Boolean(bool::from_json(value)?)),
            "nick" => Ok(AuditLogChangeValue::String(String::from_json(value)?)),
            "avatar_hash" => Ok(AuditLogChangeValue::String(String::from_json(value)?)),

            _ => JsonError::err(format!("Unknown audit log change key ({})", key))
        }
    }
}

impl FromJson for AuditLogChange {
    fn from_json(json: &str) -> Result<Self, JsonError> where Self: Sized {
        let map = json::json_object_to_map(json)?;

        if let Some(&key) = map.get("key") {
            let key = String::from_json(key)?;

            Ok(AuditLogChange {
                new_value: match map.get("new_value") {
                    Some(v) => Some(AuditLogChange::deserialize_value(&key, v)?),
                    None => None
                },
                old_value: match map.get("old_value") {
                    Some(v) => Some(AuditLogChange::deserialize_value(&key, v)?),
                    None => None
                },
                key,
            })
        } else {
            JsonError::err("Could not find key in audit log change")
        }
    }
}