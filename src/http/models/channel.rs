use crate::Nullable;
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
    pub parent_id: Option<Nullable<u64>>,
}

pub enum MessagesPosition {
    Default,
    Limit(i32),
    Before(u64, i32),
    Around(u64, i32),
    After(u64, i32)
}

pub enum ReactionsPosition {
    Default,
    Limit(i32),
    Before(u64, i32),
    After(u64, i32)
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