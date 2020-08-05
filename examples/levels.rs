//! Example demonstrating the custom storage
//! feature by creating a stored `Count` object
//! and using the provided ones.

#[macro_use]
extern crate automate;

use automate::{Context, Error, Snowflake, Configuration, Automate};
use automate::gateway::{MessageCreateDispatch, UpdateStatus, StatusType, User};
use automate::http::CreateMessage;
use automate::log::LevelFilter;
use std::collections::HashMap;

/// This is a special case: because the type we are storing is
/// not defined in our example because it is a u32, we can't
/// implement `Stored` on it.
/// A way to go around that problem would be to create a
/// [newtype struct](https://doc.rust-lang.org/stable/rust-by-example/generics/new_types.html)
/// and make it the `Stored` value. However, since we are only
/// storing a `u32` and not a whole struct, it would be quite
/// annoying to use a newtype struct.
///
/// Therefore, we make this empty `Count` struct that is only used
/// to fetch the `CountStorage` through `ctx.storage::<Count>().await`
/// and we directly deal with `u32`s when calling the storage's methods.
#[derive(Stored, Copy, Clone)]
struct Count;

/// The storage struct is responsible for keeping the
/// stored objects in memory and providing methods
/// for retrieving and inserting objects.
#[derive(Storage, Default, Clone)]
struct CountStorage {
    counts: HashMap<(Snowflake, Snowflake), u32>,
}

impl CountStorage {
    /// Increments the message count of the given user
    fn increment(&mut self, guild: Snowflake, user: Snowflake) -> u32 {
        let value = self.counts.get(&(guild, user)).map_or(1, |v| v + 1);
        self.counts.insert((guild, user), value);

        value
    }

    /// Finds the 10 users with the most messages sent.
    fn leaderboard<'a>(&self, guild: Snowflake) -> Vec<(Snowflake, u32)> {
        let mut leaderboard = self.counts.iter()
            .filter(|((g, _), _)| *g == guild) //take only from given guild
            .map(|((_, u), count)| (*u, *count)) //remove the guild
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

            let mut output = String::from("These are the top 10 users:\n");

            for (position, (user, count)) in leaderboard.iter().enumerate() {
                output.push_str(&format!("{}. {} is **level {}** and posted a total of **{} messages**\n",
                                         position,
                                         users.get(*user).username,
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
        //get the storage and increment for the message author
        let count = ctx.storage_mut::<Count>().await.increment(guild, message.author.id);
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

/// Calculates the level of a user based on the
/// amount of messages he has sent.
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

fn main() -> Result<(), Error> {
    let config = Configuration::from_env("DISCORD_API_TOKEN")
        .enable_logging()
        .level_for("automate", LevelFilter::Trace)
        .presence(UpdateStatus {
            status: StatusType::Idle,
            game: None,
            afk: false,
            since: None,
        })
        .add_initializer(|ctn| ctn.initialize::<Count>())
        .register(stateless!(leaderboard_command, count));

    Automate::launch(config)
}
