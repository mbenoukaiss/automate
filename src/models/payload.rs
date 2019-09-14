use crate::models::*;
use std::collections::HashMap;
use crate::json::Nullable;
use crate::FromJson;

#[object(server)]
pub struct Payload<D> where D: FromJson {
    pub op: u8,
    pub d: D,
    pub s: Nullable<u32>,
    pub t: Nullable<String>,
}

#[payload(op = 0, event = "READY", server)]
pub struct DispatchReady {
    pub v: u16,
    pub user: User,
    pub private_channels: Vec<Channel>,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub shard: Option<[u32; 2]>
}

#[payload(op = 0, event = "GUILD_CREATE", server)]
pub struct DispatchGuildCreate(pub Guild);

/// This payload should not be handled by bots
/// as it will always be empty and does not mean
/// anything.
///
/// More information in [this issue](https://github.com/discordapp/discord-api-docs/issues/683)
#[payload(op = 0, event = "PRESENCES_REPLACE", server)]
pub struct DispatchPresencesReplace(pub Vec<PresenceUpdate>);

#[payload(op = 0, event = "PRESENCE_UPDATE", server)]
pub struct DispatchPresenceUpdate(pub PresenceUpdate);

#[payload(op = 0, event = "MESSAGE_CREATE", server)]
pub struct DispatchMessageCreate(pub Message);

#[payload(op = 0, event = "MESSAGE_UPDATE", server)]
pub struct DispatchMessageUpdate(pub Message);

#[payload(op = 0, event = "MESSAGE_DELETE", server)]
pub struct DispatchMessageDelete {
    pub id: u64,
    pub channel_id: u64,
    pub guild_id: Option<u64>
}

#[payload(op = 0, event = "MESSAGE_DELETE_BULK", server)]
pub struct DispatchMessageDeleteBulk {
    pub ids: Vec<u64>,
    pub channel_id: u64,
    pub guild_id: Option<u64>
}

#[payload(op = 1, client)]
pub struct Heartbeat(pub Nullable<u32>);

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

#[payload(op = 10, server)]
pub struct Hello {
    pub heartbeat_interval: u32
}

#[object(client)]
pub struct UpdateStatus {
    pub since: Nullable<i32>,
    pub game: Nullable<Activity>,
    pub status: StatusType,
    pub afk: bool
}

#[stringify(snake_case)]
pub enum StatusType {
    Online,
    Dnd,
    Idle,
    Invisible,
    Offline
}