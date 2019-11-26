use crate::gateway::*;
use std::collections::HashMap;
use crate::json::{FromJson, Nullable};
use crate::Snowflake;

#[object(server)]
pub struct Payload<D> where D: FromJson {
    pub op: u8,
    pub d: D,
    pub s: Nullable<i32>,
    pub t: Nullable<String>,
}

#[payload(op = 0, event = "READY", server)]
pub struct ReadyDispatch {
    pub v: u16,
    pub user: User,
    pub private_channels: Vec<Channel>,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub shard: Option<[u32; 2]>
}

#[payload(op = 0, event = "RESUMED", server)]
pub struct ResumedDispatch;

#[payload(op = 0, event = "CHANNEL_CREATE", server)]
pub struct ChannelCreateDispatch(pub Channel);

#[payload(op = 0, event = "CHANNEL_UPDATE", server)]
pub struct ChannelUpdateDispatch(pub Channel);

#[payload(op = 0, event = "CHANNEL_DELETE", server)]
pub struct ChannelDeleteDispatch(pub Channel);

#[payload(op = 0, event = "CHANNEL_PINS_UPDATE", server)]
pub struct ChannelPinsUpdateDispatch {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    pub last_pin_timestamp: Option<String>
}

#[payload(op = 0, event = "GUILD_CREATE", server)]
pub struct GuildCreateDispatch(pub Guild);

#[payload(op = 0, event = "GUILD_UPDATE", server)]
pub struct GuildUpdateDispatch(pub Guild);

#[payload(op = 0, event = "GUILD_DELETE", server)]
pub struct GuildDeleteDispatch {
    pub id: Snowflake,
    pub unavailable: Option<bool>
}

#[payload(op = 0, event = "GUILD_BAN_ADD", server)]
pub struct GuildBanAddDispatch {
    pub guild_id: Snowflake,
    pub user: User
}

#[payload(op = 0, event = "GUILD_BAN_REMOVE", server)]
pub struct GuildBanRemoveDispatch {
    pub guild_id: Snowflake,
    pub user: User
}

#[payload(op = 0, event = "GUILD_EMOJIS_UPDATE", server)]
pub struct GuildEmojisUpdateDispatch {
    pub guild_id: Snowflake,
    pub emojis: Vec<Emoji>
}

#[payload(op = 0, event = "GUILD_INTEGRATIONS_UPDATE", server)]
pub struct GuildIntegrationsUpdateDispatch {
    pub guild_id: Snowflake,
}

#[payload(op = 0, event = "GUILD_MEMBER_ADD", server)]
pub struct GuildMemberAddDispatch {
    pub guild_id: Snowflake,
    pub user: User,
    pub nick: Option<Nullable<String>>,
    pub roles: Vec<Snowflake>,
    pub joined_at: String,
    pub premium_since: Option<Nullable<String>>,
    pub hoisted_role: Nullable<Snowflake>,
    pub deaf: bool,
    pub mute: bool
}

#[payload(op = 0, event = "GUILD_MEMBER_REMOVE", server)]
pub struct GuildMemberRemoveDispatch {
    pub guild_id: Snowflake,
    pub user: User
}

#[payload(op = 0, event = "GUILD_MEMBER_UPDATE", server)]
pub struct GuildMemberUpdateDispatch {
    pub guild_id: Snowflake,
    pub roles: Vec<Snowflake>,
    pub user: User,
    pub nick: String
}

/// Sent in response to [RequestGuildMembers](RequestGuildMembers)
#[payload(op = 0, event = "GUILD_MEMBERS_CHUNK", server)]
pub struct GuildMembersChunkDispatch {
    pub guild_id: Snowflake,
    pub members: Vec<GuildMember>,
    pub not_found: Option<Vec<Snowflake>>,
    pub presences: Option<Vec<PresenceUpdate>>
}

#[payload(op = 0, event = "GUILD_ROLE_CREATE", server)]
pub struct GuildRoleCreateDispatch {
    pub guild_id: Snowflake,
    pub role: Role,
}

#[payload(op = 0, event = "GUILD_ROLE_UPDATE", server)]
pub struct GuildRoleUpdateDispatch {
    pub guild_id: Snowflake,
    pub role: Role,
}

#[payload(op = 0, event = "GUILD_ROLE_DELETE", server)]
pub struct GuildRoleDeleteDispatch {
    pub guild_id: Snowflake,
    pub role: Snowflake,
}

#[payload(op = 0, event = "MESSAGE_CREATE", server)]
pub struct MessageCreateDispatch(pub Message);

#[payload(op = 0, event = "MESSAGE_UPDATE", server)]
pub struct MessageUpdateDispatch(pub Message);

#[payload(op = 0, event = "MESSAGE_DELETE", server)]
pub struct MessageDeleteDispatch {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>
}

#[payload(op = 0, event = "MESSAGE_DELETE_BULK", server)]
pub struct MessageDeleteBulkDispatch {
    pub ids: Vec<Snowflake>,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>
}

#[payload(op = 0, event = "MESSAGE_REACTION_ADD", server)]
pub struct MessageReactionAddDispatch {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    pub user_id: Snowflake,
    pub message_id: Snowflake,
    pub emoji: PartialEmoji
}

#[payload(op = 0, event = "MESSAGE_REACTION_REMOVE", server)]
pub struct MessageReactionRemoveDispatch {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    pub user_id: Snowflake,
    pub message_id: Snowflake,
    pub emoji: PartialEmoji
}

#[payload(op = 0, event = "MESSAGE_REACTION_REMOVE_ALL", server)]
pub struct MessageReactionRemoveAllDispatch {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    pub message_id: Snowflake
}

/// This payload should not be handled by bots
/// as it will always be empty and does not mean
/// anything.
///
/// More information in [this issue](https://github.com/discordapp/discord-api-docs/issues/683)
#[payload(op = 0, event = "PRESENCES_REPLACE", server)]
pub struct PresencesReplaceDispatch(pub Vec<PresenceUpdate>);

#[payload(op = 0, event = "PRESENCE_UPDATE", server)]
pub struct PresenceUpdateDispatch(pub PresenceUpdate);

#[payload(op = 0, event = "TYPING_START", server)]
pub struct TypingStartDispatch {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    pub user_id: Snowflake,
    pub member: GuildMember
}

#[payload(op = 0, event = "USER_UPDATE", server)]
pub struct UserUpdateDispatch(pub User);

#[payload(op = 0, event = "VOICE_STATE_UPDATE", server)]
pub struct VoiceStateUpdateDispatch(pub VoiceState);

#[payload(op = 0, event = "VOICE_SERVER_UPDATE", server)]
pub struct VoiceServerUpdateDispatch {
    pub token: String,
    pub guild_id: Snowflake,
    pub endpoint: String,
}

#[payload(op = 0, event = "WEBHOOKS_UPDATE", server)]
pub struct WebhooksUpdateDispatch {
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
}

#[payload(op = 1, client)]
pub struct Heartbeat(pub Nullable<i32>);

#[payload(op = 2, client)]
pub struct Identify {
    pub token: String,
    pub properties: HashMap<String, String>,
    pub compress: Option<bool>,
    pub large_threshold: Option<u8>,
    pub shard: Option<[i32; 2]>,
    pub presence: Option<UpdateStatus>,
    pub guild_subscriptions: Option<bool>
}

#[payload(op = 3, client)]
pub struct UpdateStatus {
    pub since: Nullable<i32>,
    pub game: Nullable<Activity>,
    pub status: StatusType,
    pub afk: bool
}

#[payload(op = 4, client)]
pub struct UpdateVoiceState {
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub self_mute: bool,
    pub self_deaf: bool,
}

#[payload(op = 6, client)]
pub struct Resume {
    pub token: String,
    pub session_id: String,
    pub seq: i32,
}

#[payload(op = 7, server)]
pub struct Reconnect;

#[payload(op = 8, client)]
pub struct RequestGuildMembers {
    pub guild_id: Snowflake,
    pub query: Option<String>,
    pub limit: Option<i32>,
    pub presences: Option<bool>,
    pub user_ids: Vec<Snowflake>
}

#[payload(op = 9, server)]
pub struct InvalidSession(pub bool);

#[payload(op = 10, server)]
pub struct Hello {
    pub heartbeat_interval: u32
}

#[stringify(snake_case)]
pub enum StatusType {
    Online,
    Dnd,
    Idle,
    Invisible,
    Offline
}