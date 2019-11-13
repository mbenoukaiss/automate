#![feature(never_type)]

use async_trait::async_trait;
use automate::{Error, Discord, Listener, Session};
use automate::models::{CreateMessage, MessageReactionAddDispatch, MessageCreateDispatch};

struct MessageListener;

#[async_trait]
impl Listener for MessageListener {
    async fn on_message_create(&mut self, session: &Session, message: &MessageCreateDispatch) -> Result<(), Error> {
        if !message.author.bot.unwrap_or(false) {
            session.create_message(message.channel_id, CreateMessage {
                content: Some(String::from("Hello sir!")),
                ..Default::default()
            }).await?;
        }   

        Ok(())
    }

    async fn on_reaction_add(&mut self, session: &Session, message: &MessageReactionAddDispatch) -> Result<(), Error> {
        session.create_message(message.channel_id, CreateMessage {
            content: Some(String::from("Nice reaction")),
            ..Default::default()
        }).await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    automate::setup_logging();

    Discord::new("NjEzMDUzOTEwMjc3NTU0MTg0.XVrU-Q.-Liuq8tU9HQtNN6pWD-Tjxu7IRY")
        .register_listener(Box::new(MessageListener))
        .connect().await?
}
