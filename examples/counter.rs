#[macro_use]
extern crate automate;

use automate::{async_trait, Session, Listener, Error, Discord, Snowflake};
use automate::gateway::MessageCreateDispatch;
use automate::http::CreateMessage;
use std::env;
use std::collections::HashMap;

#[derive(Default)]
struct MessageCounter {
    counts: HashMap<Snowflake, i32>
}

#[async_trait]
impl Listener for MessageCounter {
    async fn on_message_create(&mut self, session: &Session, data: &MessageCreateDispatch) -> Result<(), Error> {
        let message = &data.0;

        if message.content.starts_with("!count") {
            let count =  self.counts.get(&message.author.id)
                .map(|i| i.to_owned())
                .unwrap_or(0);

            session.create_message(message.channel_id, CreateMessage {
                content: Some(format!("You posted a total of {} messages!", count)),
                nonce: None,
                tts: None,
                file: None,
                embed: None,
                payload_json: None
            }).await?;
        } else {
            let count = self.counts.remove(&message.author.id).unwrap_or(0);
            self.counts.insert(message.author.id, count + 1);
        }

        Ok(())
    }
}

fn main() {
    automate::setup_logging();

    Discord::new(&env::var("DISCORD_API_TOKEN").expect("API token not found"))
        .with(structs!(MessageCounter::default()))
        .connect_blocking()
}
