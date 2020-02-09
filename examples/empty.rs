#![allow(where_clauses_object_safety)] //should be fixable when async traits are allowed

use automate::{async_trait, Listener, Session};
use automate::{Error, Discord};
use automate::gateway::MessageReactionAddDispatch;
use std::env;

struct MessageListener;

#[async_trait]
impl Listener for MessageListener {
    async fn on_reaction_add(&mut self, session: &Session, data: &MessageReactionAddDispatch) -> Result<(), Error> {
        if data.user_id != session.bot().id {
            session.create_reaction(data.channel_id, data.message_id, &data.emoji).await?;
        }

        Ok(())
    }
}

fn main() {
    automate::setup_logging();

    Discord::new(&env::var("DISCORD_API_TOKEN").expect("API token not found"))
        .with_listener(MessageListener)
        .connect_blocking()
}
