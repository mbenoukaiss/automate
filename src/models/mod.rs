mod payload;
mod channel;
mod guild;
mod user;
mod voice;

pub use payload::*;
pub use channel::*;
pub use guild::*;
pub use user::*;
pub use voice::*;
use url::Url;

#[object(server)]
pub struct Gateway {
    pub url: Url
}