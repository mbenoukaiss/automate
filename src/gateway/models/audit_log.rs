use crate::gateway::{User, Webhook, Overwrite, ChannelType, PartialRole};
use crate::Snowflake;

#[object(server)]
pub struct AuditLog {
    pub webhooks: Vec<Webhook>,
    pub users: Vec<User>,
    pub audit_log_entries: Vec<AuditLogEntry>,
}

#[object(server)]
pub struct AuditLogEntry {
    pub id: Snowflake,
    #[nullable]
    pub target_id: Option<String>,
    pub changes: Option<Vec<AuditLogChange>>,
    pub user_id: Snowflake,
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
    IntegrationDelete = 82,
}

#[object(server)]
pub struct AuditEntryInfo {
    pub delete_member_days: Option<String>,
    pub members_removed: Option<String>,
    pub channel_id: Option<Snowflake>,
    pub count: Option<String>,
    pub id: Option<Snowflake>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub role_name: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct AuditLogChange {
    pub key: String,
    pub new_value: Option<AuditLogChangeValue>,
    pub old_value: Option<AuditLogChangeValue>,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(untagged)]
pub enum AuditLogChangeValue {
    Snowflake(Snowflake),
    String(String),
    Integer(i32),
    Boolean(bool),
    Roles(Vec<PartialRole>),
    Overwrites(Vec<Overwrite>),
    Channel(ChannelType),
}
