use crate::gateway::GuildMember;
use crate::{Snowflake, Nullable};

#[object(server)]
pub struct VoiceState {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Nullable<Snowflake>,
    pub user_id: Snowflake,
    pub member: GuildMember,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub suppress: bool
}

#[object(server)]
pub struct PartialVoiceState {
    pub channel_id: Nullable<Snowflake>,
    pub user_id: Snowflake,
    pub member: GuildMember,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub suppress: bool
}


#[object(server)]
pub struct VoiceRegion {
    pub id: String,
    pub name: String,
    pub vip: bool,
    pub optimal: bool,
    pub deprecated: bool,
    pub custom: bool,
}