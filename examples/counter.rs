//this is the stateful version which has not been released yet and doesn't
//work with versions less than or equal to 0.3.0
//see old_counter to see current version

#[macro_use]
extern crate automate;

use automate::{Context, Error, Snowflake, Configuration, Automate};
use automate::gateway::MessageCreateDispatch;
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
                nonce: None,
                tts: None,
                file: None,
                embed: None,
                payload_json: None,
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
        .log_level(LevelFilter::Trace)
        .register(instances!(MessageCounter::default()));

    Automate::launch(config);
}
