use crate::gateway::{User, PartialUser, PartialGuild};
use crate::Snowflake;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Channel {
    Category(Category),
    Text(TextChannel),
    Voice(VoiceChannel),
    News(NewsChannel),
    Store(StoreChannel),
    Direct(DirectChannel),
    Group(GroupChannel),
}

#[derive(Debug, Clone)]
pub enum GuildChannel {
    Category(Category),
    Text(TextChannel),
    Voice(VoiceChannel),
    News(NewsChannel),
    Store(StoreChannel),
}

#[derive(Debug, Clone)]
pub enum PrivateChannel {
    Direct(DirectChannel),
    Group(GroupChannel),
}

#[object(server)]
pub struct Category {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub position: i32,
    pub permission_overwrites: Vec<Overwrite>,
    pub name: String,
}

#[object(server)]
pub struct TextChannel {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    #[nullable]
    pub parent_id: Option<Snowflake>,
    pub position: i32,
    pub permission_overwrites: Vec<Overwrite>,
    pub name: String,
    #[nullable]
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    #[nullable]
    pub last_message_id: Option<Snowflake>,
    pub rate_limit_per_user: i32,
    pub last_pin_timestamp: Option<String>,
}

#[object(server)]
pub struct VoiceChannel {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    #[nullable]
    pub parent_id: Option<Snowflake>,
    pub position: i32,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: String,
    pub bitrate: i32,
    pub user_limit: i32,
}

#[object(server)]
pub struct NewsChannel {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    #[nullable]
    pub parent_id: Option<Snowflake>,
    pub position: i32,
    pub permission_overwrites: Vec<Overwrite>,
    pub name: String,
    #[nullable]
    pub topic: Option<String>,
    #[nullable]
    pub last_message_id: Option<Snowflake>,
    pub last_pin_timestamp: Option<String>,
}

#[object(server)]
pub struct StoreChannel {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    #[nullable]
    pub parent_id: Option<Snowflake>,
    pub position: i32,
    pub permission_overwrites: Vec<Overwrite>,
    pub name: String,
}

#[object(server)]
pub struct DirectChannel {
    pub id: Snowflake,
    pub name: Option<String>,
    #[option_nullable]
    pub last_message_id: Option<Option<Snowflake>>,
    pub last_pin_timestamp: Option<String>,
}

#[object(server)]
pub struct GroupChannel {
    pub id: Snowflake,
    pub name: String,
    #[option_nullable]
    pub icon: Option<Option<String>>,
    #[serde(deserialize_with = "automate::encode::json::as_hashmap")]
    pub recipients: HashMap<Snowflake, User>,
    pub owner_id: Snowflake,
    pub application_id: Option<Snowflake>,
    #[option_nullable]
    pub last_message_id: Option<Option<Snowflake>>,
    pub last_pin_timestamp: Option<String>,
}

#[object(server)]
pub struct Invite {
    pub code: String,
    pub guild: Option<PartialGuild>,
    pub channel: Channel,
    pub inviter: Option<User>,
    pub target_user: Option<PartialUser>,
    pub target_user_type: Option<i32>,
    pub approximate_presence_count: Option<i32>,
    pub approximate_member_count: Option<i32>,

    //following variables are extra information
    pub uses: Option<i32>,
    pub max_uses: Option<i32>,
    pub max_age: Option<i32>,
    pub temporary: Option<bool>,
    pub created_at: Option<String>,
}

#[object(server)]
pub struct PartialInvite {
    pub code: Option<String>,
    pub guild: Option<PartialGuild>,
    pub channel: Option<Channel>,
    pub inviter: Option<User>,
    pub target_user: Option<PartialUser>,
    pub target_user_type: Option<i32>,
    pub approximate_presence_count: Option<i32>,
    pub approximate_member_count: Option<i32>,

    //following variables are extra information
    pub uses: Option<i32>,
    pub max_uses: Option<i32>,
    pub max_age: Option<i32>,
    pub temporary: Option<bool>,
    pub created_at: Option<String>,
}

#[convert(u8)]
pub enum ChannelType {
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GroupDM = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
}

#[object(both)]
pub struct Overwrite {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub _type: OverwriteType,
    pub allow: u32,
    pub deny: u32,
}

#[stringify(snake_case)]
pub enum OverwriteType {
    Role,
    Member,
}

#[object(both)]
pub struct ChannelMention {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    #[serde(rename = "type")]
    pub _type: ChannelType,
    pub name: String,
}

#[object(server)]
pub struct Webhook {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub _type: WebhookType,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    pub user: Option<User>,
    #[option_nullable]
    pub name: Option<Option<String>>,
    #[option_nullable]
    pub avatar: Option<Option<String>>,
    pub token: Option<String>,
}

#[convert(u8)]
pub enum WebhookType {
    Incoming = 1,
    ChannelFollower = 2,
}

mod channels {
    use crate::{Snowflake, Identifiable};
    use super::{Channel, GuildChannel, PrivateChannel};

    impl Identifiable for Channel {
        fn id(&self) -> Snowflake {
            match self {
                Channel::Category(c) => c.id,
                Channel::Text(c) => c.id,
                Channel::Voice(c) => c.id,
                Channel::News(c) => c.id,
                Channel::Store(c) => c.id,
                Channel::Direct(c) => c.id,
                Channel::Group(c) => c.id,
            }
        }
    }

    impl Identifiable for GuildChannel {
        fn id(&self) -> Snowflake {
            match self {
                GuildChannel::Category(c) => c.id,
                GuildChannel::Text(c) => c.id,
                GuildChannel::Voice(c) => c.id,
                GuildChannel::News(c) => c.id,
                GuildChannel::Store(c) => c.id,
            }
        }
    }

    impl Identifiable for PrivateChannel {
        fn id(&self) -> Snowflake {
            match self {
                PrivateChannel::Direct(c) => c.id,
                PrivateChannel::Group(c) => c.id,
            }
        }
    }

    impl Channel {
        pub fn from_guild(channel: &GuildChannel) -> Channel {
            match channel {
                GuildChannel::Category(c) => Channel::Category(Clone::clone(c)),
                GuildChannel::Text(c) => Channel::Text(Clone::clone(c)),
                GuildChannel::Voice(c) => Channel::Voice(Clone::clone(c)),
                GuildChannel::News(c) => Channel::News(Clone::clone(c)),
                GuildChannel::Store(c) => Channel::Store(Clone::clone(c)),
            }
        }

        pub fn from_private(channel: &PrivateChannel) -> Channel {
            match channel {
                PrivateChannel::Direct(c) => Channel::Direct(Clone::clone(c)),
                PrivateChannel::Group(c) => Channel::Group(Clone::clone(c)),
            }
        }
    }
}

mod deserialize {
    use serde::{Deserialize, Deserializer};
    use serde::private::de::{ContentDeserializer, TaggedContentVisitor};
    use serde::de::{Visitor, Error, Unexpected};
    use std::fmt::{self, Formatter};
    use super::{Category, TextChannel, VoiceChannel, NewsChannel, StoreChannel, DirectChannel, GroupChannel};
    use super::{Channel, GuildChannel, PrivateChannel};

    enum ChannelTag {
        Category,
        Text,
        Voice,
        News,
        Store,
        Direct,
        Group,
    }

    struct ChannelTypeVisitor;

    impl<'de> Visitor<'de> for ChannelTypeVisitor {
        type Value = ChannelTag;

        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
            Formatter::write_str(formatter, "variant number")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> where E: Error {
            match value {
                0 => Ok(ChannelTag::Text),
                1 => Ok(ChannelTag::Direct),
                2 => Ok(ChannelTag::Voice),
                3 => Ok(ChannelTag::Group),
                4 => Ok(ChannelTag::Category),
                5 => Ok(ChannelTag::News),
                6 => Ok(ChannelTag::Store),
                _ => Err(Error::invalid_value(Unexpected::Unsigned(value), &"variant index 0 <= i < 7")),
            }
        }
    }

    impl<'de> Deserialize<'de> for ChannelTag {
        #[inline]
        fn deserialize<D>(d: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
            Deserializer::deserialize_u64(d, ChannelTypeVisitor)
        }
    }

    impl<'de> Deserialize<'de> for Channel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
            let tagged = Deserializer::deserialize_any(
                deserializer,
                TaggedContentVisitor::<ChannelTag>::new("type"),
            )?;

            match tagged.tag {
                ChannelTag::Category => Result::map(
                    Category::deserialize(ContentDeserializer::<D::Error>::new(tagged.content)),
                    Channel::Category,
                ),
                ChannelTag::Text => Result::map(
                    TextChannel::deserialize(ContentDeserializer::<D::Error>::new(tagged.content)),
                    Channel::Text,
                ),
                ChannelTag::Voice => Result::map(
                    VoiceChannel::deserialize(ContentDeserializer::<D::Error>::new(tagged.content)),
                    Channel::Voice,
                ),
                ChannelTag::News => Result::map(
                    NewsChannel::deserialize(ContentDeserializer::<D::Error>::new(tagged.content)),
                    Channel::News,
                ),
                ChannelTag::Store => Result::map(
                    StoreChannel::deserialize(ContentDeserializer::<D::Error>::new(tagged.content)),
                    Channel::Store,
                ),
                ChannelTag::Direct => Result::map(
                    DirectChannel::deserialize(ContentDeserializer::<D::Error>::new(tagged.content)),
                    Channel::Direct,
                ),
                ChannelTag::Group => Result::map(
                    GroupChannel::deserialize(ContentDeserializer::<D::Error>::new(tagged.content)),
                    Channel::Group,
                ),
            }
        }
    }

    impl<'de> Deserialize<'de> for GuildChannel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
            let tagged = Deserializer::deserialize_any(
                deserializer,
                TaggedContentVisitor::<ChannelTag>::new("type"),
            )?;

            match tagged.tag {
                ChannelTag::Category => Result::map(
                    Category::deserialize(ContentDeserializer::<D::Error>::new(tagged.content)),
                    GuildChannel::Category,
                ),
                ChannelTag::Text => Result::map(
                    TextChannel::deserialize(ContentDeserializer::<D::Error>::new(tagged.content)),
                    GuildChannel::Text,
                ),
                ChannelTag::Voice => Result::map(
                    VoiceChannel::deserialize(ContentDeserializer::<D::Error>::new(tagged.content)),
                    GuildChannel::Voice,
                ),
                ChannelTag::News => Result::map(
                    NewsChannel::deserialize(ContentDeserializer::<D::Error>::new(tagged.content)),
                    GuildChannel::News,
                ),
                ChannelTag::Store => Result::map(
                    StoreChannel::deserialize(ContentDeserializer::<D::Error>::new(tagged.content)),
                    GuildChannel::Store,
                ),
                _ => Err(Error::custom("disallowed type for guild channel"))
            }
        }
    }

    impl<'de> Deserialize<'de> for PrivateChannel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
            let tagged = Deserializer::deserialize_any(
                deserializer,
                TaggedContentVisitor::<ChannelTag>::new("type"),
            )?;

            match tagged.tag {
                ChannelTag::Direct => Result::map(
                    DirectChannel::deserialize(ContentDeserializer::<D::Error>::new(tagged.content)),
                    PrivateChannel::Direct,
                ),
                ChannelTag::Group => Result::map(
                    GroupChannel::deserialize(ContentDeserializer::<D::Error>::new(tagged.content)),
                    PrivateChannel::Group,
                ),
                _ => Err(Error::custom("disallowed type for private channel"))
            }
        }
    }
}