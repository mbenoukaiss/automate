#![allow(where_clauses_object_safety)] //should be fixable when async traits are allowed

use automate::{tokio, async_trait};
use automate::{Error, Discord, Listener, Session};
use automate::models::{CreateMessage, MessageReactionAddDispatch, MessageCreateDispatch};
use std::env;

struct MessageListener;

#[async_trait]
impl Listener for MessageListener {
    async fn on_message_create(&mut self, session: &Session, message: &MessageCreateDispatch) -> Result<(), Error> {
        if !message.author.bot.unwrap_or(false) {
            let content = Some(format!("Hello {}!", message.author.username));

            session.create_message(message.channel_id, CreateMessage {
                content,
                ..Default::default()
            }).await?;
        }   

        Ok(())
    }

    async fn on_reaction_add(&mut self, session: &Session, message: &MessageReactionAddDispatch) -> Result<(), Error> {
        let content = Some(format!("{}?!", message.emoji.name));

        session.create_message(message.channel_id, CreateMessage {
            content,
            ..Default::default()
        }).await?;

        Ok(())
    }
}

#[automate::tokio::main]
async fn main() -> Result<(), Error> {
    automate::setup_logging();

    Discord::new(&env::var("DISCORD_API_TOKEN").expect("API token not found"))
        .with_listener(Box::new(MessageListener))
        .connect().await?
}
