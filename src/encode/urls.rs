use crate::{Error, Snowflake, Nullable};
use crate::gateway::*;
use crate::http::*;
use percent_encoding::NON_ALPHANUMERIC;
use std::fmt::Write;

/// Any type which is identified by
/// a snowflake.
pub trait ExtractSnowflake {
    fn extract_snowflake(&self) -> Result<Snowflake, Error>;
}

/// Any type that can be put in a URL
/// but requires some kind of encoding
pub trait WriteUrl {
    fn write_url(&self, buf: &mut String) -> Result<(), Error>;
}

macro_rules! automate_types {
    ($($struct:ty),*) => {
        $(
            impl ExtractSnowflake for $struct {
                fn extract_snowflake(&self) -> Result<Snowflake, Error> {
                    Ok(self.id)
                }
            }

            impl ExtractSnowflake for &$struct {
                fn extract_snowflake(&self) -> Result<Snowflake, Error> {
                    Ok(self.id)
                }
            }
        )*
    }
}

automate_types! {
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

impl ExtractSnowflake for Snowflake {
    fn extract_snowflake(&self) -> Result<Snowflake, Error> {
        Ok(*self)
    }
}

impl ExtractSnowflake for Emoji {
    fn extract_snowflake(&self) -> Result<Snowflake, Error> {
        if let Nullable::Value(id) = self.id {
            Ok(id)
        } else {
            Error::err("Emoji's id field is empty")
        }
    }
}

impl ExtractSnowflake for PartialEmoji {
    fn extract_snowflake(&self) -> Result<Snowflake, Error> {
        if let Nullable::Value(id) = self.id {
            Ok(id)
        } else {
            Error::err("Emoji's id field is empty")
        }
    }
}

impl WriteUrl for &str {
    fn write_url(&self, buf: &mut String) -> Result<(), Error> {
        let penc = percent_encoding::percent_encode(self.as_bytes(), NON_ALPHANUMERIC);
        buf.write_fmt(format_args!("{}", penc))?;

        Ok(())
    }
}

impl WriteUrl for String {
    fn write_url(&self, buf: &mut String) -> Result<(), Error> {
        let penc = percent_encoding::percent_encode(self.as_bytes(), NON_ALPHANUMERIC);
        buf.write_fmt(format_args!("{}", penc))?;

        Ok(())
    }
}

impl WriteUrl for Emoji {
    fn write_url(&self, buf: &mut String) -> Result<(), Error> {
        let penc = percent_encoding::percent_encode(self.name.as_bytes(), NON_ALPHANUMERIC);

        buf.reserve(self.name.len());
        buf.write_fmt(format_args!("{}", penc))?;

        if let Nullable::Value(id) = self.id {
            buf.push(':');
            buf.push_str(&id.to_string());
        }

        Ok(())
    }
}

impl WriteUrl for PartialEmoji {
    fn write_url(&self, buf: &mut String) -> Result<(), Error> {
        let penc = percent_encoding::percent_encode(self.name.as_bytes(), NON_ALPHANUMERIC);

        buf.reserve(self.name.len());
        buf.write_fmt(format_args!("{}", penc))?;

        if let Nullable::Value(id) = self.id {
            buf.push(':');
            buf.push_str(&id.to_string());
        }

        Ok(())
    }
}