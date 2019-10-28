#![feature(never_type)]

use async_trait::async_trait;
use automatea::{Error, Discord, Listener, Session};
use automatea::models::{Message, CreateMessage};

struct MessageListener;

#[async_trait]
impl Listener for MessageListener {
    async fn on_message_create(&mut self, session: &Session, message: &Message) -> Result<(), Error> {
        if !message.author.bot.unwrap_or(false) {
            session.create_message(message.channel_id, CreateMessage {
                content: Some(String::from("Hello sir!")),
                ..Default::default()
            }).await?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    automatea::setup_logging();

    Discord::new()
        .register_listener(Box::new(MessageListener))
        .connect().await?
}
