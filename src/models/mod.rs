mod payload;
mod guild;
mod channel;
mod message;
mod user;
mod voice;

pub use payload::*;
pub use guild::*;
pub use channel::*;
pub use message::*;
pub use user::*;
pub use voice::*;
use url::Url;

#[object(server)]
pub struct Gateway {
    pub url: Url
}