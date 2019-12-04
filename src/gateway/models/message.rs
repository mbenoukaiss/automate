use crate::gateway::{User, MentionnedUser, PartialGuildMember, ChannelMention, PartialEmoji};
use crate::encode::Nullable;
use crate::Snowflake;

#[object(server)]
pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub author: User,
    pub member: Option<PartialGuildMember>,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Nullable<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<MentionnedUser>,
    pub mention_roles: Vec<Snowflake>,
    pub mention_channels: Option<Vec<ChannelMention>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Nullable<Snowflake>,
    pub pinned: bool,
    pub webhook_id: Option<Snowflake>,
    pub _type: MessageType,
    pub activity: Option<MessageActivity>,
    pub application: Option<MessageApplication>,
    pub message_reference: Option<MessageReference>,
    pub flags: Option<u32>
}

#[object(server)]
pub struct PartialMessage {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub author: Option<User>,
    pub member: Option<PartialGuildMember>,
    pub content: Option<String>,
    pub timestamp: String,
    pub edited_timestamp: Nullable<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<MentionnedUser>,
    pub mention_roles: Vec<Snowflake>,
    pub mention_channels: Option<Vec<ChannelMention>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    pub pinned: bool,
    pub webhook_id: Option<Snowflake>,
    pub _type: MessageType,
    pub activity: Option<MessageActivity>,
    pub application: Option<MessageApplication>,
    pub message_reference: Option<MessageReference>,
    pub flags: Option<u32>
}

#[convert(u8)]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    GuildMemberJoin = 7,
    UserPremiumGuildSubscription = 8,
    UserPremiumGuildSubscriptionTier1 = 9,
    UserPremiumGuildSubscriptionTier2 = 10,
    UserPremiumGuildSubscriptionTier3 = 11,
    ChannelFollowAdd = 12
}

#[convert(u32)]
pub enum MessageFlags {
    Crossposted = 1 << 0,
    IsCrosspost = 1 << 1,
    SuppressEmbeds = 1 << 2
}

#[object(server)]
pub struct Attachment {
    pub id: Snowflake,
    pub filename: String,
    pub size: usize,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u32>,
    pub width: Option<u32>
}

#[object(both)]
pub struct Embed {
    pub title: Option<String>,
    pub _type: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<i32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>
}

#[object(both)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>
}

#[object(both)]
pub struct EmbedImage {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>
}

#[object(both)]
pub struct EmbedThumbnail {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>
}

#[object(both)]
pub struct EmbedVideo {
    pub url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>
}

#[object(both)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>
}

#[object(both)]
pub struct EmbedAuthor {
    pub name: Option<String>,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>
}

#[object(both)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>
}

#[object(server)]
pub struct Reaction {
    pub count: u32,
    pub me: bool,
    pub emoji: PartialEmoji
}

#[object(server)]
pub struct MessageActivity {
    pub _type: MessageActivityType,
    pub party_id: Option<String>
}

#[convert(u8)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

#[object(server)]
pub struct MessageApplication {
    pub id: Snowflake,
    pub cover_image: Option<String>,
    pub description: String,
    pub icon: Nullable<String>,
    pub name: String
}

#[object(server)]
pub struct MessageReference {
    pub message_id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Snowflake
}