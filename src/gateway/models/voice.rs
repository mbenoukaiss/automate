use crate::gateway::GuildMember;

#[object(server)]
pub struct VoiceState {
    pub guild_id: Option<u64>,
    pub channel_id: Option<u64>,
    pub user_id: u64,
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
    pub channel_id: Option<u64>,
    pub user_id: u64,
    pub member: GuildMember,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub suppress: bool
}