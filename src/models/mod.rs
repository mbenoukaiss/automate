mod payload;
mod channel;
mod guild;
mod user;

pub use payload::*;
pub use channel::*;
pub use guild::*;
pub use user::*;

#[object(server)]
pub struct Gateway {
    pub url: String
}