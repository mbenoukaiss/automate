use crate::models::{Channel, User, UnavailableGuild};
use std::collections::HashMap;

#[object(server)]
pub struct Payload<D> {
    pub op: u8,
    pub d: D,
    pub s: Option<u32>,
    pub t: Option<String>,
}

#[payload(op = 0, server)]
pub struct Ready {
    pub v: u16,
    pub user: User,
    pub private_channels: Vec<Channel>,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub shard: Option<[u32; 2]>
}

#[payload(op = 2, client)]
#[derive(::serde::Serialize)]
pub struct Identify {
    pub token: String,
    pub properties: HashMap<String, String>,
    pub compress: Option<bool>
}

#[payload(op = 10, server)]
pub struct Hello {
    pub heartbeat_interval: u32
}