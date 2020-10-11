use crate::gateway::GuildMember;
use crate::Snowflake;

#[object(server)]
pub struct VoiceState {
    pub guild_id: Option<Snowflake>,
    #[nullable]
    pub channel_id: Option<Snowflake>,
    pub user_id: Snowflake,
    pub member: Option<GuildMember>,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_video: bool,
    pub self_stream: Option<bool>,
    pub suppress: bool
}

#[object(server)]
pub struct PartialVoiceState {
    #[nullable]
    pub channel_id: Option<Snowflake>,
    pub user_id: Snowflake,
    pub member: Option<GuildMember>,
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