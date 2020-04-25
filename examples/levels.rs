#[macro_use]
extern crate automate;

use automate::{Context, Error, Snowflake, Configuration, Automate};
use automate::gateway::{MessageCreateDispatch, UpdateStatus, StatusType, ActivityType, ActivityUpdate, User, GuildCreateDispatch, GuildMemberAddDispatch, Guild};
use automate::http::CreateMessage;
use automate::events::{Initializable, StatefulListener};
use automate::log::LevelFilter;
use std::collections::HashMap;

/// Stores the users and their level per guild.
/// Necessary since Automate currently does not
/// handle caching data sent by Discord.
#[derive(Default, Clone)]
struct Storage {
    users: HashMap<Snowflake, User>,
    counts: HashMap<(Snowflake, Snowflake), u32>,
}

impl Storage {
    /// Finds the 10 players with the most messages
    /// sent.
    fn leaderboard(&self, guild: Snowflake) -> Vec<(&User, u32)> {
        let mut leaderboard = self.counts
            .clone()
            .into_iter()
            .filter(|((g, _), _)| *g == guild)
            .map(|((_, u), count)| (self.users.get(&u).unwrap(), count))
            .take(10)
            .collect::<Vec<(&User, u32)>>();

        leaderboard.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));
        leaderboard
    }

    /// Caches all the users of a guild.
    fn cache_users(&mut self, guild: &Guild) {
        if let Some(members) = guild.members.as_ref() {
            for member in members {
                self.users.insert(member.user.id, member.user.clone());
            }
        }
    }

    /// Add a single user to the cache
    fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    /// Mutable reference to the message count of a user.
    fn user_count(&mut self, guild: Snowflake, user: Snowflake) -> &mut u32 {
        if !self.counts.contains_key(&(guild, user)) {
            self.counts.insert((guild, user), 0);
        }

        self.counts.get_mut(&(guild, user)).unwrap()
    }
}

#[derive(State, Default, Clone)]
struct MessageCounter {
    storage: Storage,
}

impl Initializable for MessageCounter {
    fn initialize() -> Vec<StatefulListener<Self>> {
        methods!(MessageCounter: register_guilds, register_member, leaderboard, count)
    }
}

impl MessageCounter {
    #[listener]
    async fn register_guilds(&mut self, _: &mut Context, data: &GuildCreateDispatch) -> Result<(), Error> {
        self.storage.cache_users(&data.0);
        Ok(())
    }

    #[listener]
    async fn register_member(&mut self, _: &mut Context, data: &GuildMemberAddDispatch) -> Result<(), Error> {
        self.storage.add_user(data.user.clone());
        Ok(())
    }

    #[listener]
    async fn leaderboard(&self, ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
        let message = &data.0;

        if message.content.starts_with("!leaderboard") {
            if let Some(guild) = message.guild_id {
                let leaderboard = self.storage.leaderboard(guild);
                let mut output = String::from("These are the top 10 players:\n");

                for (position, (user, count)) in leaderboard.iter().enumerate() {
                    output.push_str(&format!("{}. {} is **level {}** and posted a total of **{} messages**\n",
                                             position,
                                             user.username,
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
    async fn count(&mut self, ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
        let message = &data.0;

        //ignore messages from the bot itself
        if message.author.id == ctx.bot().id {
            return Ok(())
        }

        //don't count messages outside of guilds
        if let Some(guild) = message.guild_id {
            let count = self.storage.user_count(guild, message.author.id);
            *count += 1;

            let (level, levelled_up) = level(*count);

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
        .register(stateful!(MessageCounter::default()));

    Automate::launch(config);
}
