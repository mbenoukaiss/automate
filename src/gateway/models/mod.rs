mod payload;
mod audit_log;
mod channel;
mod guild;
mod message;
mod user;
mod voice;

pub use payload::*;
pub use audit_log::*;
pub use channel::*;
pub use guild::*;
pub use message::*;
pub use user::*;
pub use voice::*;

/// An object with a single valid WSS URL, which is used
/// for connecting. This value should be cached.
#[object(server)]
pub struct Gateway {
    pub url: String
}

/// An object with a valid WSS URL and additional
/// metadata that can help during the operation of
/// large or sharded bots. This value should not
/// be cached since they may change as the bot
/// joins and leaves guilds.
#[object(server)]
pub struct GatewayBot {
    pub url: String,
    pub shards: i32,
    pub session_start_limit: SessionStartLimit
}

/// Information about the sessions allowed for
/// the current user.
#[object(server)]
pub struct SessionStartLimit {
    pub total: i32,
    pub remaining: i32,
    pub reset_after: i32
}
