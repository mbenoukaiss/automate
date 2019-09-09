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
pub struct Ready {
    pub v: u16,
    pub user: User,
    pub private_channels: Vec<Channel>,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub shard: Option<[u32; 2]>
}

#[payload(op = 0, event = "GUILD_CREATE", server)]
pub struct GuildCreate {
    pub id: u64,
    pub name: String,
    pub icon: Nullable<String>,
    pub splash: Nullable<String>,
    pub owner: Option<bool>,
    pub owner_id: u64,
    pub permissions: Option<u32>,
    pub region: String,
    pub afk_channel_id: Nullable<u64>,
    pub afk_timeout: i32,
    pub embed_enabled: Option<bool>,
    pub embed_channel_id: Option<u64>,
    pub verification_level: VerificationLevel,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub explicit_content_filter: ExplicitContentFilterLevel,
    pub roles: Vec<Role>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<GuildFeature>,
    pub mfa_level: MFALevel,
    pub application_id: Nullable<u64>,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<u64>,
    pub system_channel_id: Nullable<u64>,
    pub joined_at: Option<String>,
    pub large: Option<bool>,
    pub unavailable: Option<bool>,
    pub member_count: Option<i32>,
    pub voice_states: Option<Vec<PartialVoiceState>>,
    pub members: Option<Vec<GuildMember>>,
    pub channels: Option<Vec<Channel>>,
    pub presences: Option<Vec<PartialPresenceUpdate>>,
    pub max_presences: Option<Nullable<i32>>,
    pub max_members: Option<i32>,
    pub vanity_url_code: Nullable<String>,
    pub description: Nullable<String>,
    pub banner: Nullable<String>,
    pub premium_tier: PremiumTier,
    pub premium_subscription_count: Option<i32>,
    pub preferred_locale: String
}

#[payload(op = 2, client)]
pub struct Identify {
    pub token: String,
    pub properties: HashMap<String, String>,
    pub compress: Option<bool>
}


#[payload(op = 10, server)]
pub struct Hello {
    pub heartbeat_interval: u32
}