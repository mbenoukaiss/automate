#[macro_use]
extern crate automate;

use automate::{Context, Error, Snowflake, Configuration, Automate};
use automate::gateway::{MessageCreateDispatch, UpdateStatus, StatusType, ActivityType, ActivityUpdate, User};
use automate::http::CreateMessage;
use automate::events::{Initializable, StatefulListener};
use automate::log::LevelFilter;
use std::collections::HashMap;
use automate::storage::{UserStorage, Storage, Stored};

#[derive(Clone)]
struct Count(u32);

impl Stored for Count {
    type Storage = CountsStorage;
}

#[derive(Default)]
struct CountsStorage {
    counts: HashMap<(Snowflake, Snowflake), Count>,
}

impl Storage for CountsStorage {
    type Key = (Snowflake, Snowflake);
    type Stored = Count;

    fn get(&self, id: &Self::Key) -> &Self::Stored {
        self.find(id).unwrap()
    }

    fn find(&self, id: &Self::Key) -> Option<&Self::Stored> {
        self.counts.get(&id)
    }

    fn insert(&mut self, key: &Self::Key, val: &Self::Stored) {
        self.counts.insert((*key).clone(), (*val).clone());
    }
}

impl CountsStorage {
    /// Finds the 10 players with the most messages sent.
    fn leaderboard<'a>(&self, guild: Snowflake) -> Vec<(Snowflake, u32)> {
        let mut leaderboard = self.counts.iter()
            .filter(|((g, _), _)| *g == guild) //take only from given guild
            .map(|((_, u), count)| (*u, count.0)) //remove the guild
            .take(10)
            .collect::<Vec<(Snowflake, u32)>>();

        leaderboard.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));
        leaderboard
    }
}

#[listener]
async fn leaderboard_command(ctx: &Context, data: &MessageCreateDispatch) -> Result<(), Error> {
    let message = &data.0;

    if message.content.starts_with("!leaderboard") {
        if let Some(guild) = message.guild_id {
            let users = ctx.storage::<User>().await;
            let leaderboard = ctx.storage::<Count>().await.leaderboard(guild);
            let mut output = String::from("These are the top 10 players:\n");

            for (position, (user, count)) in leaderboard.iter().enumerate() {
                output.push_str(&format!("{}. {} is **level {}** and posted a total of **{} messages**\n",
                                         position,
                                         users.get(user).username,
                                         level(*count).0,
                                         count));
            }

            ctx.create_message(message.channel_id, CreateMessage {
                content: Some(output),
                ..Default::default()
            }).await?;
        }
    }

    Ok(())
}

#[listener]
async fn count(ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
    let message = &data.0;

    //ignore messages from the bot itself
    if message.author.id == ctx.bot.id {
        return Ok(());
    }

    //don't count messages outside of guilds
    if let Some(guild) = message.guild_id {
        let key = (guild, message.author.id);

        let count = {
            let mut storage = ctx.storage::<Count>().await;

            let count = storage.find(&key).unwrap_or(&Count(0)).0;
            storage.insert(&key, &Count(count + 1));

            count
        };

        let (level, levelled_up) = level(count);

        if levelled_up && level != 0 {
            let content = format!("<@{}> you just advanced to **level {}**!", message.author.id, level);

            ctx.create_message(message.channel_id, CreateMessage {
                content: Some(content),
                ..Default::default()
            }).await?;
        }
    }

    Ok(())
}

fn level(msg: u32) -> (u32, bool) {
    let level = 0.4 * f64::from(msg).sqrt();
    let level = level.round() as u32;

    let previous_level = if msg > 0 {
        (0.4 * f64::from(msg - 1).sqrt()).round() as u32
    } else {
        0
    };

    (level, previous_level < level)
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
                url: None,
            }),
            since: None,
        })
        .register(stateless!(leaderboard_command, count));

    Automate::launch(config);
}
