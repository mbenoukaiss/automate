use std::fmt::{Display, Debug, Formatter, Error as FmtError};
use std::ops::{Deref, DerefMut};
use serde::{Deserializer, Deserialize};
use serde::de::Visitor;
use serde::de::Error as DeError;
use core::fmt;

/// Twitter's [snowflake](https://github.com/twitter/snowflake/tree/snowflake-2010)
/// used by discord API to provide unique IDs for guilds, channels, users, etc.
/// Since a snowflake is 64bits in size, it is stored as a u64 in the
/// `automate` library.
///
/// Snowflakes are serialized as normal integers but can be deserialized
/// from string or integer since Discord may send normal integers or wrap
/// them in a string.
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Copy, Clone)]
#[derive(Serialize)]
pub struct Snowflake(pub u64);

struct SnowflakeVisitor;

impl<'de> Visitor<'de> for SnowflakeVisitor {
    type Value = Snowflake;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A large integer or a large integer in a string")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> where E: DeError {
        if value < std::u64::MIN as i64 {
            Ok(Snowflake(value as u64))
        } else {
            Err(E::custom(format!("Snowflake out of range: {}", value)))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> where E: DeError {
        Ok(Snowflake(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: DeError, {
        match value.parse::<u64>() {
            Ok(val) => self.visit_u64(val),
            Err(_) => Err(E::custom("Failed to parse snowflake")),
        }
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Snowflake, D::Error> where D: Deserializer<'de>, {
        deserializer.deserialize_str(SnowflakeVisitor)
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

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Snowflake {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
