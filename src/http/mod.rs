//! Tools to interact with Discord's HTTP API

mod models;
mod rate_limit;

pub use models::*;
pub use rate_limit::collect_outdated_buckets;

use crate::gateway::*;
use crate::{Error, Snowflake};
use crate::encode::{ExtractSnowflake, WriteUrl};
use hyper::Client;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;

/// Struct used to interact with the discord HTTP API.
#[derive(Clone)]
pub struct HttpAPI {
    client: Client<HttpsConnector<HttpConnector>>,
    token: String,
}

impl HttpAPI {
    pub fn new(token: &str) -> HttpAPI {
        let https = HttpsConnector::new();

        let mut bot_token = String::with_capacity(token.len() + 4);
        bot_token.push_str("Bot ");
        bot_token.push_str(token);

        HttpAPI {
            client: Client::builder().build(https),
            token: bot_token,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    #[endpoint(get, route = "/gateway", status = 200)]
    pub async fn gateway(&self) -> Result<Gateway, Error> {}

    #[endpoint(get, route = "/gateway/bot", status = 200)]
    pub async fn gateway_bot(&self) -> Result<GatewayBot, Error> {}

    #[endpoint(get, route = "/guilds/{#guild}/audit-logs", status = 200)]
    pub async fn audit_logs<S: ExtractSnowflake>(&self, guild: S) -> Result<AuditLog, Error> {}

    #[endpoint(get, route = "/guilds/{#guild}", status = 200)]
    pub async fn guild<S: ExtractSnowflake>(&self, guild: S) -> Result<Guild, Error> {}

    /// Creates a guild
    /// The first role defined in the roles vector will
    /// be used to define the permissions for `@everyone`.
    //TODO: Check if the bot is in less than 10 guilds
    //TODO: Check that channels don't have a `parent_id`
    #[endpoint(post, route = "/guilds", body = "new_guild", status = 201)]
    pub async fn create_guild(&self, new_guild: NewGuild) -> Result<Guild, Error> {}

    #[endpoint(patch, route = "/guilds/{#guild}", body = "modification", status = 200)]
    pub async fn modify_guild<S: ExtractSnowflake>(&self, guild: S, modification: ModifyGuild) -> Result<Guild, Error> {}

    #[endpoint(delete, route = "/guilds/{#guild}", status = 204, empty)]
    pub async fn delete_guild<S: ExtractSnowflake>(&self, guild: S) -> Result<(), Error> {}

    #[endpoint(get, route = "/guilds/{#guild}/channels", status = 200)]
    pub async fn channels<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<GuildChannel>, Error> {}

    #[endpoint(get, route = "/channels/{#channel}", status = 200)]
    pub async fn channel<S: ExtractSnowflake>(&self, channel: S) -> Result<Channel, Error> {}

    #[endpoint(post, route = "/guilds/{#guild}/channels", body = "new_channel", status = 200)]
    pub async fn create_channel<S: ExtractSnowflake>(&self, guild: S, new_channel: NewChannel) -> Result<GuildChannel, Error> {}

    //TODO: is it possible to modify DM channels?
    #[endpoint(patch, route = "/channels/{#channel}", body = "modification", status = 200)]
    pub async fn modify_channel<S: ExtractSnowflake>(&self, channel: S, modification: ModifyChannel) -> Result<Channel, Error> {}

    //TODO: Check if there are at least 2 channels
    #[endpoint(patch, route = "/guilds/{#guild}/channels", body = "moves", status = 204, empty)]
    pub async fn move_channels<S: ExtractSnowflake>(&self, guild: S, moves: Vec<MoveChannel>) -> Result<(), Error> {}

    //TODO: delete channels recursively?
    #[endpoint(delete, route = "/channels/{#channel}", status = 200)]
    pub async fn delete_channel<S: ExtractSnowflake>(&self, channel: S) -> Result<Channel, Error> {}

    #[endpoint(get, route = "/guilds/{#guild}/members/{#user}", status = 200)]
    pub async fn member<S: ExtractSnowflake>(&self, guild: S, user: S) -> Result<GuildMember, Error> {}

    #[endpoint(get, route = "/guilds/{#guild}/members/{query}", status = 200)]
    pub async fn members<S: ExtractSnowflake>(&self, guild: S, filter: MemberFilter) -> Result<Vec<GuildMember>, Error> {
        let query = match filter {
            MemberFilter::Default => String::new(),
            MemberFilter::Limit(limit) => format!("?limit={}", limit),
            MemberFilter::After(s, limit) => format!("?after={}&limit={}", s, limit),
        };
    }

    #[endpoint(patch, route = "/guilds/{#guild}/members/{#user}", body = "member", status = 204, empty)]
    pub async fn modify_member<S: ExtractSnowflake>(&self, guild: S, user: S, member: ModifyMember) -> Result<(), Error> {}

    #[endpoint(delete, route = "/guilds/{#guild}/members/{#user}", status = 204, empty)]
    pub async fn remove_member<S: ExtractSnowflake>(&self, guild: S, user: S) -> Result<(), Error> {}

    #[endpoint(patch, route = "/guilds/{#guild}/members/@me/nick", body = "nick", status = 200, empty)]
    pub async fn modify_own_nick<S: ExtractSnowflake>(&self, guild: S, nick: &str) -> Result<(), Error> {
        let nick = serde_json::json!({
            "nick": nick
        });
    }

    #[endpoint(put, route = "/guilds/{#guild}/members/{#user}/roles/{#role}", status = 204, empty)]
    pub async fn member_add_role<S: ExtractSnowflake>(&self, guild: S, user: S, role: S) -> Result<(), Error> {}

    #[endpoint(delete, route = "/guilds/{#guild}/members/{#user}/roles/{#role}", status = 204, empty)]
    pub async fn member_remove_role<S: ExtractSnowflake>(&self, guild: S, user: S, role: S) -> Result<(), Error> {}

    #[endpoint(get, route = "/guilds/{#guild}/bans", status = 200)]
    pub async fn bans<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<Ban>, Error> {}

    #[endpoint(get, route = "/guilds/{#guild}/bans/{#user}", status = 200)]
    pub async fn ban<S: ExtractSnowflake>(&self, guild: S, user: S) -> Result<Ban, Error> {}

    #[endpoint(put, route = "/guilds/{#guild}/bans/{#user}/{query}", status = 204, empty)]
    pub async fn create_ban<S: ExtractSnowflake>(&self, guild: S, user: S, reason: Option<&str>, delete_days: Option<i8>) -> Result<(), Error> {
        let mut query = String::from("?");

        if let Some(reason) = reason {
            query.push_str("reason=");
            query.push_str(reason);
        }

        if let Some(delete_days) = delete_days {
            if query.len() != 1 {
                query.push('&');
            }

            query.push_str("delete-message-days=");
            query.push_str(&delete_days.to_string());
        }
    }

    #[endpoint(delete, route = "/guilds/{#guild}/bans/{#user}", status = 204, empty)]
    pub async fn remove_ban<S: ExtractSnowflake>(&self, guild: S, user: S) -> Result<(), Error> {}

    #[endpoint(get, route = "/guilds/{#guild}/roles", status = 200)]
    pub async fn roles<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<Role>, Error> {}

    #[endpoint(post, route = "/guilds/{#guild}/roles", body = "role", status = 200)]
    pub async fn create_role<S: ExtractSnowflake>(&self, guild: S, role: NewRole) -> Result<Role, Error> {}

    #[endpoint(patch, route = "/guilds/{#guild}/roles/{#role}", body = "modification", status = 200)]
    pub async fn modify_roles<S: ExtractSnowflake>(&self, guild: S, role: S, modification: ModifyRole) -> Result<Role, Error> {}

    #[endpoint(patch, route = "/guilds/{#guild}/roles", body = "roles", status = 200)]
    pub async fn move_roles<S: ExtractSnowflake>(&self, guild: S, roles: Vec<MoveRole>) -> Result<Vec<Role>, Error> {}

    #[endpoint(delete, route = "/guilds/{#guild}/roles/{#role}", status = 204, empty)]
    pub async fn remove_role<S: ExtractSnowflake>(&self, guild: S, role: S) -> Result<(), Error> {}

    #[endpoint(get, route = "/guilds/{#guild}/prune?days={days}", status = 200)]
    pub async fn simulate_prune<S: ExtractSnowflake>(&self, guild: S, days: i32) -> Result<Prune, Error> {}

    #[endpoint(post, route = "/guilds/{#guild}/prune?days={days}&compute_prune_count=false", status = 200, empty)]
    pub async fn prune<S: ExtractSnowflake>(&self, guild: S, days: i32) -> Result<(), Error> {}

    #[endpoint(post, route = "/guilds/{#guild}/prune?days={days}&compute_prune_count=true", status = 200)]
    pub async fn prune_with_report<S: ExtractSnowflake>(&self, guild: S, days: i32) -> Result<Prune, Error> {}

    #[endpoint(get, route = "/voice/regions", status = 200)]
    pub async fn voice_regions(&self) -> Result<Vec<VoiceRegion>, Error> {}

    #[endpoint(get, route = "/guilds/{#guild}/regions", status = 200)]
    pub async fn guild_voice_regions<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<VoiceRegion>, Error> {}

    #[endpoint(get, route = "/guilds/{#guild}/integrations", status = 200)]
    pub async fn integrations<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<Integration>, Error> {}

    #[endpoint(get, route = "/guilds/{#guild}/embed", status = 200)]
    pub async fn embed<S: ExtractSnowflake>(&self, guild: S) -> Result<GuildEmbed, Error> {}

    #[endpoint(patch, route = "/guilds/{#guild}/embed", body = "embed", status = 200)]
    pub async fn modify_embed<S: ExtractSnowflake>(&self, guild: S, embed: GuildEmbed) -> Result<GuildEmbed, Error> {}

    #[endpoint(get, route = "/guilds/{#guild}/vanity-url", status = 200)]
    pub async fn vanity_url<S: ExtractSnowflake>(&self, guild: S) -> Result<PartialInvite, Error> {}

    //TODO: guild widget image

    #[endpoint(get, route = "/channels/{#channel}/messages/{#message}", status = 200)]
    pub async fn message<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<Message, Error> {}

    #[endpoint(get, route = "/channels/{#channel}/messages/{query}", status = 200)]
    pub async fn messages<S: ExtractSnowflake>(&self, channel: S, messages: MessagesPosition) -> Result<Vec<Message>, Error> {
        let query = match messages {
            MessagesPosition::Default => String::new(),
            MessagesPosition::Limit(limit) => format!("?limit={}", limit),
            MessagesPosition::Before(s, limit) => format!("?before={}&limit={}", s, limit),
            MessagesPosition::Around(s, limit) => format!("?around={}&limit={}", s, limit),
            MessagesPosition::After(s, limit) => format!("?after={}&limit={}", s, limit),
        };
    }

    //TODO: handle sending files
    #[endpoint(post, route = "/channels/{#channel}/messages", body = "message", status = 200)]
    pub async fn create_message<S: ExtractSnowflake>(&self, channel: S, message: CreateMessage) -> Result<Message, Error> {}

    #[endpoint(post, route = "/channels/{#channel}/messages/{#message}", body = "modification", status = 200)]
    pub async fn modify_message<S: ExtractSnowflake>(&self, channel: S, message: S, modification: ModifyMessage) -> Result<Message, Error> {}

    #[endpoint(delete, route = "/channels/{#channel}/messages/{#message}", status = 204, empty)]
    pub async fn delete_message<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<(), Error> {}

    #[endpoint(delete, route = "/channels/{#channel}/messages/bulk-delete", body = "snowflakes", status = 204, empty)]
    pub async fn delete_message_bulk<S: ExtractSnowflake>(&self, channel: S, messages: Vec<S>) -> Result<(), Error> {
        let snowflakes = messages.iter()
            .map(|m| m.extract_snowflake())
            .collect::<Result<Vec<Snowflake>, Error>>()?;
    }

    #[endpoint(get, route = "/channels/{#channel}/messages/{#message}/reactions/{+emoji}/{query}", status = 200)]
    pub async fn reactions<S: ExtractSnowflake, U: WriteUrl>(&self, channel: S, message: S, emoji: &U, reactions: ReactionsPosition) -> Result<Vec<User>, Error> {
        let query = match reactions {
            ReactionsPosition::Default => String::new(),
            ReactionsPosition::Limit(limit) => format!("?limit={}", limit),
            ReactionsPosition::Before(s, limit) => format!("?before={}&limit={}", s, limit),
            ReactionsPosition::After(s, limit) => format!("?after={}&limit={}", s, limit),
        };
    }

    #[endpoint(put, route = "/channels/{#channel}/messages/{#message}/reactions/{+emoji}/@me", status = 204, empty)]
    pub async fn create_reaction<S: ExtractSnowflake, U: WriteUrl>(&self, channel: S, message: S, emoji: &U) -> Result<(), Error> {}

    #[endpoint(delete, route = "/channels/{#channel}/messages/{#message}/reactions/{+emoji}/{#user}", status = 204, empty)]
    pub async fn delete_reaction<S: ExtractSnowflake, U: WriteUrl>(&self, channel: S, message: S, emoji: &U, user: S) -> Result<(), Error> {}

    #[endpoint(delete, route = "/channels/{#channel}/messages/{#message}/reactions/{+emoji}/@me", status = 204, empty)]
    pub async fn delete_own_reaction<S: ExtractSnowflake, U: WriteUrl>(&self, channel: S, message: S, emoji: &U) -> Result<(), Error> {}

    #[endpoint(delete, route = "/channels/{#channel}/messages/{#message}/reactions", status = 204, empty)]
    pub async fn delete_all_reaction<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<(), Error> {}

    #[endpoint(get, route = "/guilds/{#guild}/emojis", status = 200)]
    pub async fn emojis<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<Emoji>, Error> {}

    #[endpoint(get, route = "/guilds/{#guild}/emojis/{#emoji}", status = 200)]
    pub async fn emoji<S: ExtractSnowflake>(&self, guild: S, emoji: S) -> Result<Emoji, Error> {}

    #[endpoint(post, route = "/guilds/{#guild}/emojis", body = "emoji", status = 200)]
    pub async fn create_emoji<S: ExtractSnowflake>(&self, guild: S, emoji: NewEmoji) -> Result<Emoji, Error> {}

    #[endpoint(patch, route = "/guilds/{#guild}/emojis/{#emoji}", body = "modification", status = 200)]
    pub async fn modify_emoji<S: ExtractSnowflake>(&self, guild: S, emoji: S, modification: ModifyEmoji) -> Result<Emoji, Error> {}

    #[endpoint(delete, route = "/guilds/{#guild}/emojis/{#emoji}", status = 204, empty)]
    pub async fn delete_emoji<S: ExtractSnowflake>(&self, guild: S, emoji: S) -> Result<(), Error> {}

    /// Retrieves an invite by its code.
    #[endpoint(get, route = "/invites/{code}", status = 200)]
    pub async fn invite(&self, code: &str) -> Result<Invite, Error> {}

    /// Retrieves an invite by its code with the
    /// approximate member counts of the server.
    #[endpoint(get, route = "/invites/{code}?with_counts=true", status = 200)]
    pub async fn invite_with_counts(&self, code: &str) -> Result<Invite, Error> {}

    /// Retrieves all the invites in a guild.
    #[endpoint(get, route = "/guilds/{#guild}/invites", status = 200)]
    pub async fn guild_invites<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<Invite>, Error> {}

    /// Retrieves all the invites in a channel.
    #[endpoint(get, route = "/channels/{#channel}/invites", status = 200)]
    pub async fn channel_invites<S: ExtractSnowflake>(&self, channel: S) -> Result<Vec<Invite>, Error> {}

    /// Create an invite for the specified channel.
    #[endpoint(post, route = "/channels/{#channel}/invites", body = "invite", status = 200)]
    pub async fn create_invite<S: ExtractSnowflake>(&self, channel: S, invite: NewInvite) -> Result<Invite, Error> {}

    /// Create an invite for the specified channel.
    #[endpoint(delete, route = "/invites/{code}", status = 200)]
    pub async fn delete_invite(&self, code: &str) -> Result<Invite, Error> {}

    #[endpoint(post, route = "/channels/{#channel}/permissions/{#overwrite}", body = "permissions", status = 204, empty)]
    pub async fn modify_channel_permissions<S: ExtractSnowflake>(&self, channel: S, overwrite: S, permissions: NewOverwrite) -> Result<(), Error> {}

    #[endpoint(delete, route = "/channels/{#channel}/permissions/{#overwrite}", status = 204, empty)]
    pub async fn delete_channel_permission<S: ExtractSnowflake>(&self, channel: S, overwrite: S) -> Result<(), Error> {}

    #[endpoint(post, route = "/channels/{#channel}/typing", status = 204, empty)]
    pub async fn trigger_typing<S: ExtractSnowflake>(&self, channel: S) -> Result<(), Error> {}

    #[endpoint(get, route = "/channels/{#channel}/pins", status = 200)]
    pub async fn pinned_messages<S: ExtractSnowflake>(&self, channel: S) -> Result<Vec<Message>, Error> {}

    #[endpoint(put, route = "/channels/{#channel}/pins/{#message}", status = 204, empty)]
    pub async fn pin_message<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<(), Error> {}

    //TODO: deletes the message or the pin?
    #[endpoint(delete, route = "/channels/{#channel}/pins/{#message}", status = 204, empty)]
    pub async fn delete_pinned_message<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<(), Error> {}

    /// Returns the current user.
    #[endpoint(get, route = "/users/@me", status = 200)]
    pub async fn curent_user(&self) -> Result<User, Error> {}

    #[endpoint(patch, route = "/users/@me", body = "bot", status = 200)]
    pub async fn modify_current_user(&self, bot: ModifyBot) -> Result<User, Error> {}

    /// Retrieves the ids of the guilds this
    /// bot is in.
    #[endpoint(get, route = "/users/@me/guilds", status = 200)]
    pub async fn bot_guilds(&self) -> Result<Vec<PartialGuild>, Error> {}

    #[endpoint(delete, route = "/users/@me/guilds/{#guild}", status = 204, empty)]
    pub async fn leave_guild<S: ExtractSnowflake>(&self, guild: S) -> Result<(), Error> {}

    #[endpoint(get, route = "/users/{#user}", status = 200)]
    pub async fn user<S: ExtractSnowflake>(&self, user: S) -> Result<User, Error> {}

    #[endpoint(post, route = "/users/@me/channels", body = "recipient", status = 200)]
    pub async fn create_dm(&self, recipient: Recipient) -> Result<DirectChannel, Error> {}

    #[endpoint(put, route = "/channels/{#channel}/recipients/{#user}", body = "recipient", status = 204)]
    pub async fn add_dm_recipient<S: ExtractSnowflake>(&self, channel: S, user: S, recipient: Recipient) -> Result<(), Error> {}

    #[endpoint(delete, route = "/channels/{#channel}/recipients/{#user}", status = 204)]
    pub async fn remove_dm_recipient<S: ExtractSnowflake>(&self, channel: S, user: S) -> Result<(), Error> {}

    #[endpoint(delete, route = "/channels/{#channel}", status = 200)]
    pub async fn close_dm<S: ExtractSnowflake>(&self, channel: S) -> Result<PrivateChannel, Error> {}

}