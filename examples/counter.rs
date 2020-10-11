//! An example with a simple per guild and per user message
//! counter, an invite generator and simple use of reactions.
//!
//! A state is used but this could also work with a custom
//! storage and stateless listeners.

#[macro_use]
extern crate automate;

use automate::{Context, Error, Snowflake, Configuration, Automate};
use automate::gateway::{MessageCreateDispatch, UpdateStatus, StatusType, ActivityUpdate, ActivityType};
use automate::http::CreateMessage;
use automate::events::{Initializable, StatefulListener};
use automate::log::LevelFilter;
use std::collections::HashMap;

#[derive(State, Default, Clone)]
struct MessageCounter {
    counts: HashMap<Snowflake, i32>,
}

impl Initializable for MessageCounter {
    fn initialize() -> Vec<StatefulListener<Self>> {
        methods!(MessageCounter: tell_count, count)
    }
}

impl MessageCounter {
    #[listener]
    async fn tell_count(&mut self, ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
        let message = &data.0;

        if message.content.starts_with("!count") {
            let count = self.counts.get(&message.author.id)
                .map(|i| i.to_owned())
                .unwrap_or(0);

            ctx.create_message(message.channel_id, CreateMessage {
                content: Some(format!("You posted a total of {} messages!", count)),
                ..Default::default()
            }).await?;
        }

        Ok(())
    }

    #[listener]
    async fn count(&mut self, _: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
        let message = &data.0;

        let count = self.counts.remove(&message.author.id).unwrap_or(0);
        self.counts.insert(message.author.id, count + 1);

        Ok(())
    }
}

fn main() {
    let config = Configuration::from_env("DISCORD_API_TOKEN")
        .enable_logging()
        .level_for("automate", LevelFilter::Trace)
        .presence(UpdateStatus {
            status: StatusType::Dnd,
            afk: false,
            game: Some(ActivityUpdate {
                name: String::from("counting messages..."),
                _type: ActivityType::Game,
                url: None
            }),
            since: None
        })
        .register(stateful!(MessageCounter::default()));

    Automate::launch(config)
}
