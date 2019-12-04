use crate::gateway::{Overwrite, OverwriteType, ChannelType};
use crate::encode::Nullable;
use crate::Snowflake;

#[object(client)]
pub struct NewChannel {
    pub parent_id: Option<u64>,
    pub name: String,
    pub _type: Option<ChannelType>,
    pub topic: Option<Nullable<String>>,
    pub bitrate: Option<i32>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub nsfw: Option<bool>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
}

#[object(client)]
pub struct ModifyChannel {
    pub name: Option<String>,
    pub position: Option<i32>,
    pub topic: Option<Nullable<String>>,
    pub nsfw: Option<bool>,
    pub rate_limit_per_user: Option<i32>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub parent_id: Option<Nullable<Snowflake>>,
}

#[object(client)]
pub struct MoveChannel {
    pub id: u64,
    pub position: i32
}

pub enum MessagesPosition {
    Default,
    Limit(i32),
    Before(Snowflake, i32),
    Around(Snowflake, i32),
    After(Snowflake, i32)
}

pub enum ReactionsPosition {
    Default,
    Limit(i32),
    Before(Snowflake, i32),
    After(Snowflake, i32)
}

#[object(client)]
pub struct NewOverwrite {
    pub _type: OverwriteType,
    pub allow: u32,
    pub deny: u32
}

#[object(client)]
pub struct NewInvite {
    pub max_age: i32,
    pub max_uses: i32,
    pub temporary: bool,
    pub unique: bool
}