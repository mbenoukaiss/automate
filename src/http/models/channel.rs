use crate::{Nullable, Snowflake};
use crate::gateway::{Overwrite, OverwriteType};

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