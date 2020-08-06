//! An example with a simple per guild and per user message
//! counter, an invite generator and simple use of reactions.
//!
//! A state is used but this could also work with a custom
//! storage and stateless listeners.

#[macro_use]
extern crate automate;

use automate::{Context, Error, Snowflake, Configuration, Automate};
use automate::gateway::{MessageCreateDispatch, MessageReactionAddDispatch, UpdateStatus, StatusType, ActivityUpdate, ActivityType};
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

#[listener]
async fn copy_reaction(ctx: &mut Context, reac: &MessageReactionAddDispatch) -> Result<(), Error> {
    if reac.user_id != ctx.bot.id {
        ctx.create_reaction(reac.channel_id, reac.message_id, &reac.emoji).await?;
    }

    Ok(())
}

fn main() {
    let config = Configuration::from_env("DISCORD_API_TOKEN")
        .enable_logging()
        .level_for("automate", LevelFilter::Info)
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
        .register(stateless!(copy_reaction))
        .register(stateful!(MessageCounter::default()));

    Automate::launch(config)
}
