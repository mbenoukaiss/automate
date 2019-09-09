use crate::json::Nullable;
use crate::models::{User, PartialUser};

#[object(server)]
pub struct Guild {
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
    pub max_presences: Option<Nullable<i32>>,
    pub max_members: Option<i32>,
    pub vanity_url_code: Nullable<String>,
    pub description: Nullable<String>,
    pub banner: Nullable<String>,
    pub premium_tier: PremiumTier,
    pub premium_subscription_count: Option<i32>,
    pub preferred_locale: String
}

#[object(server)]
pub struct UnavailableGuild {
    pub id: u64,
    pub unavailable: bool
}

#[convert(u32)]
pub enum Permission {
    CreateInstantInvite = 1 << 0,
    KickMembers = 1 << 1,
    BanMembers = 1 << 2
}

#[convert(u8)]
pub enum VerificationLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4
}

#[convert(u8)]
pub enum DefaultMessageNotificationLevel {
    AllMessages = 0,
    OnlyMentions = 1,
}

#[convert(u8)]
pub enum ExplicitContentFilterLevel {
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2
}

#[stringify(upper_snake_case)]
pub enum GuildFeature {
    InviteSplash,
    VipRegions,
    VanityUrl,
    Verified,
    Partnered,
    Lurkable,
    Commerce,
    News,
    Discoverable,
    Featurable,
    AnimatedIcon,
    Banner
}

#[convert(u8)]
pub enum MFALevel {
    None = 0,
    Elevated = 1,
}

#[convert(u8)]
pub enum PremiumTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}

#[object(server)]
pub struct Role {
    pub id: u64,
    pub name: String,
    pub color: i32,
    pub hoist: bool,
    pub position: i32,
    pub permissions: u32,
    pub managed: bool,
    pub mentionable: bool
}

#[object(server)]
pub struct Emoji {
    pub id: Nullable<u64>,
    pub name: String,
    pub roles: Option<Vec<Role>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>
}

#[object(server)]
pub struct GuildMember {
    pub user: User,
    pub nick: Option<String>,
    pub roles: Vec<u64>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool
}

/// A user's presence is their current state on a guild.
/// This event is sent when a user's presence or info,
/// such as name or avatar, is updated.
///
/// The user object within this event can be partial,
/// the only field which must be sent is the `id`
/// field, everything else is optional. Along with this
/// limitation, no fields are required, and the types of
/// the fields are not validated. Your client should
/// expect any combination of fields and types within
/// this event.
///
/// More information on [Discord's documentation](https://discordapp.com/developers/docs/topics/gateway#presence-update)
#[object(server)]
pub struct PresenceUpdate {
    pub user: PartialUser,
    pub roles: Vec<u64>,
    pub game: Nullable<Activity>,
    pub guild_id: u64,
    pub status: String,
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus
}

/// A user's presence is their current state on a guild.
/// This event is sent when a user's presence or info,
/// such as name or avatar, is updated.
///
/// The user object within this event can be partial,
/// the only field which must be sent is the `id`
/// field, everything else is optional. Along with this
/// limitation, no fields are required, and the types of
/// the fields are not validated. Your client should
/// expect any combination of fields and types within
/// this event.
///
/// More information on [Discord's documentation](https://discordapp.com/developers/docs/topics/gateway#presence-update)
#[object(server)]
pub struct PartialPresenceUpdate {
    pub user: Option<PartialUser>,
    pub roles: Option<Vec<u64>>,
    pub game: Option<Nullable<Activity>>,
    pub guild_id: Option<u64>,
    pub status: Option<String>,
    pub activities: Option<Vec<Activity>>,
    pub client_status: Option<ClientStatus>
}

/// Active sessions are indicated with an "online",
/// "idle", or "dnd" string per platform. If a user
/// is offline or invisible, the corresponding
/// field is not present.
///
/// More information on [Discord's documentation](https://discordapp.com/developers/docs/topics/gateway#client-status-object)
#[object(server)]
pub struct ClientStatus {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>
}

/// A user's displayed activity.
///
/// More information on [Discord's documentation](https://discordapp.com/developers/docs/topics/gateway#activity-object)
#[object(server)]
pub struct Activity {
    pub name: String,
    pub _type: ActivityType,
    pub url: Option<Nullable<String>>,
    pub timestamps: ActivityTimestamps,
    pub application_id: Option<u64>,
    pub details: Option<Nullable<String>>,
    pub state: Option<Nullable<String>>,
    pub party: Option<ActivityParty>,
    pub assets: Option<ActivityAssets>,
    pub secrets: Option<ActivitySecrets>,
    pub instance: Option<bool>,
    pub flags: Option<u32>
}

#[convert(u8)]
pub enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2
}

#[convert(u32)]
pub enum ActivityFlags {
    Instance = 1 << 0,
    Join = 1 << 1,
    Spectate = 1 << 2,
    JoinRequest = 1 << 3,
    Sync = 1 << 4,
    Play = 1 << 5,
}

/// The unix timestamps of the start and the
/// end of the activity in milliseconds.
///
/// More information on [Discord's documentation](https://discordapp.com/developers/docs/topics/gateway#activity-object-activity-timestamps)
#[object(server)]
pub struct ActivityTimestamps {
    pub start: Option<i32>,
    pub end: Option<i32>
}

/// More information on [Discord's documentation](https://discordapp.com/developers/docs/topics/gateway#activity-object-activity-party)
#[object(server)]
pub struct ActivityParty {
    pub id: Option<String>,
    pub size: Option<[i32; 2]>
}

/// More information on [Discord's documentation](https://discordapp.com/developers/docs/topics/gateway#activity-object-activity-assets)
#[object(server)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

/// More information on [Discord's documentation](https://discordapp.com/developers/docs/topics/gateway#activity-object-activity-secrets)
#[object(server)]
pub struct ActivitySecrets {
    pub join: Option<String>,
    pub spectate: Option<String>,
    pub _match: Option<String>,
}