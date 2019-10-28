use async_trait::async_trait;
use crate::models::Message;
use crate::{Session, Error};

#[async_trait]
pub trait Listener {
    async fn on_message_create(&mut self, session: &Session, message: &Message) -> Result<(), Error>;
}