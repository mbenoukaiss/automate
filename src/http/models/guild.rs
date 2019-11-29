use crate::{Snowflake, Nullable};
use crate::gateway::{Emoji, PartialEmoji};
use percent_encoding::NON_ALPHANUMERIC;

#[object(client)]
pub struct NewEmoji {
    pub name: String,
    pub image: String,
    pub roles: Vec<Snowflake>,
}

#[object(client)]
pub struct UpdateEmoji {
    pub id: Snowflake,
    pub image: String,
    pub roles: Vec<Snowflake>,
}

pub trait UrlEncode {
    fn encode(&self) -> String;
}

impl UrlEncode for &str {
    fn encode(&self) -> String {
        percent_encoding::percent_encode(self.as_bytes(), NON_ALPHANUMERIC).to_string()
    }
}

impl UrlEncode for String {
    fn encode(&self) -> String {
        percent_encoding::percent_encode(self.as_bytes(), NON_ALPHANUMERIC).to_string()
    }
}

impl UrlEncode for Emoji {
    fn encode(&self) -> String {
        let mut code = String::with_capacity(self.name.len());
        code.push_str(&percent_encoding::percent_encode(self.name.as_bytes(), NON_ALPHANUMERIC).to_string());

        if let Nullable::Value(id) = self.id {
            code.push(':');
            code.push_str(&id.to_string());
        }

        code
    }
}

impl UrlEncode for PartialEmoji {
    fn encode(&self) -> String {
        let mut code = String::with_capacity(self.name.len());
        code.push_str(&percent_encoding::percent_encode(self.name.as_bytes(), NON_ALPHANUMERIC).to_string());

        if let Some(nullable_id) = self.id {
            if let Nullable::Value(id) = nullable_id {
                code.push(':');
                code.push_str(&id.to_string());
            }
        }

        code
    }
}