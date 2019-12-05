use crate::encode::Nullable;
use crate::gateway::{User, PartialUser, PartialVoiceState, Channel};
use crate::Snowflake;

#[object(server)]
pub struct Guild {
    pub id: Snowflake,
    pub name: String,
    pub icon: Nullable<String>,
    pub splash: Nullable<String>,
    pub owner: Option<bool>,
    pub owner_id: Snowflake,
    pub permissions: Option<u32>,
    pub region: String,
    pub afk_channel_id: Nullable<Snowflake>,
    pub afk_timeout: i32,
    pub embed_enabled: Option<bool>,
    pub embed_channel_id: Option<Snowflake>,
    pub verification_level: VerificationLevel,
    pub default_message_notifications: MessageNotificationLevel,
    pub explicit_content_filter: ExplicitContentFilterLevel,
    pub roles: Vec<Role>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<GuildFeature>,
    pub mfa_level: MFALevel,
    pub application_id: Nullable<Snowflake>,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<Snowflake>,
    pub system_channel_id: Nullable<Snowflake>,
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
    pub premium_subscription_count: Nullable<i32>,
    pub preferred_locale: String,
    pub lazy: bool,
    pub rules_channel_id: Nullable<Snowflake>
}

#[object(server)]
pub struct PartialGuild {
    pub id: Snowflake,
    pub name: String,
    pub icon: Nullable<String>,
    pub splash: Nullable<String>
}

#[object(server)]
pub struct UnavailableGuild {
    pub id: Snowflake,
    pub unavailable: bool
}

#[convert(u32)]
pub enum Permission {
    CreateInstantInvite = 1 << 0,
    KickMembers = 1 << 1,
    BanMembers = 1 << 2,
    Administrator = 1 << 3,
    ManageChannels = 1 << 4,
    ManageGuild = 1 << 5,
    AddReactions = 1 << 6,
    ViewAuditLog = 1 << 7,
    PrioritySpeaker = 1 << 8,
    Stream = 1 << 9,
    ViewChannel = 1 << 10,
    SendMessages = 1 << 11,
    SendTTSMessages = 1 << 12,
    ManageMessages = 1 << 13,
    EmbedLinks = 1 << 14,
    AttachFiles = 1 << 15,
    ReadMessageHistory = 1 << 16,
    MentionEveryone = 1 << 17,
    UseExternalEmojis = 1 << 18,
    Connect = 1 << 20,
    Speak = 1 << 21,
    MuteMembers = 1 << 22,
    DeafenMembers = 1 << 23,
    MoveMembers = 1 << 24,
    UseVAD = 1 << 25,
    ChangeNickname = 1 << 26,
    ManageNicknames = 1 << 27,
    ManageRoles = 1 << 28,
    ManageWebhooks = 1 << 29,
    ManageEmojis = 1 << 30,
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
pub enum MessageNotificationLevel {
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

#[object(both)]
pub struct Role {
    pub id: Snowflake,
    pub name: String,
    pub color: i32,
    pub hoist: bool,
    pub position: i32,
    pub permissions: u32,
    pub managed: bool,
    pub mentionable: bool
}

#[object(server)]
pub struct PartialRole {
    pub id: Snowflake,
    pub name: String,
}

#[object(both)]
pub struct Emoji {
    pub id: Nullable<Snowflake>,
    pub name: String,
    pub roles: Option<Vec<Role>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: bool
}

#[object(both)]
pub struct PartialEmoji {
    pub id: Option<Nullable<Snowflake>>,
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
    pub nick: Option<Nullable<String>>,
    pub roles: Vec<Snowflake>,
    pub joined_at: String,
    pub premium_since: Option<Nullable<String>>,
    pub hoisted_role: Nullable<Snowflake>,
    pub deaf: bool,
    pub mute: bool
}

#[object(server)]
pub struct PartialGuildMember {
    pub user: Option<User>,
    pub nick: Option<Nullable<String>>,
    pub roles: Vec<Snowflake>,
    pub joined_at: String,
    pub premium_since: Option<Nullable<String>>,
    pub hoisted_role: Nullable<Snowflake>,
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
    pub nick: Option<Nullable<String>>,
    pub roles: Vec<Snowflake>,
    pub game: Nullable<Activity>,
    pub guild_id: Snowflake,
    pub status: String,
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
    pub premium_since: Option<Nullable<String>>
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
    pub roles: Option<Vec<Snowflake>>,
    pub game: Option<Nullable<Activity>>,
    pub guild_id: Option<Snowflake>,
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
#[object(both)]
pub struct Activity {
    pub id: String,
    pub name: String,
    pub _type: ActivityType,
    pub url: Option<Nullable<String>>,
    pub timestamps: Option<ActivityTimestamps>,
    pub application_id: Option<Snowflake>,
    pub details: Option<Nullable<String>>,
    pub state: Option<Nullable<String>>,
    pub emoji: Option<PartialEmoji>,
    pub party: Option<ActivityParty>,
    pub assets: Option<ActivityAssets>,
    pub secrets: Option<ActivitySecrets>,
    pub instance: Option<bool>,
    pub flags: Option<u32>,
    pub sync_id: Option<String>,
    pub session_id: Option<String>,
    pub created_at: u64
}

#[convert(u8)]
pub enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Custom = 4
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
#[object(both)]
pub struct ActivityTimestamps {
    pub start: Option<u64>,
    pub end: Option<u64>
}

/// More information on [Discord's documentation](https://discordapp.com/developers/docs/topics/gateway#activity-object-activity-party)
#[object(both)]
pub struct ActivityParty {
    pub id: Option<String>,
    pub size: Option<[i32; 2]>
}

/// More information on [Discord's documentation](https://discordapp.com/developers/docs/topics/gateway#activity-object-activity-assets)
#[object(both)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

/// More information on [Discord's documentation](https://discordapp.com/developers/docs/topics/gateway#activity-object-activity-secrets)
#[object(both)]
pub struct ActivitySecrets {
    pub join: Option<String>,
    pub spectate: Option<String>,
    pub _match: Option<String>,
}

#[object(both)]
pub struct Ban {
    pub user: User,
    pub reason: Nullable<String>
}