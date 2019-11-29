use crate::json::{AsJson, FromJson, JsonError};
use crate::gateway::*;
use crate::http::*;
use std::fmt::{Display, Debug, Formatter, Error as FmtError};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

macro_rules! from {
    ($($struct:ty),*) => {
        $(
            impl From<$struct> for Snowflake {
                fn from(val: $struct) -> Self {
                    val.id
                }
            }
        )*
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Snowflake(pub u64);

impl AsJson for Snowflake {
    #[inline]
    fn as_json(&self) -> String {
        self.0.as_json()
    }

    #[inline]
    fn concat_json(&self, dest: &mut String) {
        self.0.concat_json(dest)
    }
}

impl FromJson for Snowflake {
    #[inline]
    fn from_json(json: &str) -> Result<Snowflake, JsonError> {
        if json.len() >= 2 && json.starts_with('"') && json.ends_with('"') {
            Ok(Snowflake(u64::from_str(&json[1..json.len() - 1])
                .map_err(|_| JsonError::new(format!("Failed to parse {} into Snowflake", json)))?))
        } else {
            JsonError::err("Incorrect JSON snowflake received")
        }
    }
}

impl From<u64> for Snowflake {
    fn from(s: u64) -> Self {
        Snowflake(s)
    }
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        Display::fmt(&self.0, f)
    }
}

impl Debug for Snowflake {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        Debug::fmt(&self.0, f)
    }
}

impl Deref for Snowflake {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Snowflake {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

from! {
    AuditLogEntry,
    Channel, ChannelMention,
    Overwrite,
    Webhook,
    Guild, PartialGuild, UnavailableGuild,
    Role, PartialRole,
    UpdateEmoji,
    Message, Attachment, MessageApplication,
    User, PartialUser, MentionnedUser
}
