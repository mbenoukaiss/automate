mod models;

pub use models::*;

use crate::gateway::*;
use crate::encode::AsJson;
use crate::{Error, Snowflake};
use crate::encode::{ExtractSnowflake, WriteUrl};
use crate::http::rate_limit::Bucket;
use bytes::buf::ext::BufExt;
use hyper::{Client, Request, Body, Response, Method, HeaderMap};
use hyper::client::HttpConnector;
use hyper::header::HeaderValue;
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use chrono::NaiveDateTime;

/// Creates the URL to an API endpoint
/// by concatenating the given expressions.
///
/// This macro accepts four kinds of arguments:
/// * String literals, which are simply concatenated to
/// the final string
/// * Types implementing the ExtractSnoflake type,
/// their snowflake will be appended to the URL using
/// [extract_snowflake](automate::encode:ExtractSnowflake::extract_snowflake)
/// (prefix: #).
/// * Types implementing the WriteUrl type, which will
/// be appended to the final string by calling their
/// [write_url](automate::encode::WriteUrl::write_url)
/// method. Useful for types that require a specific
/// formatting or for strings that need to be escaped
/// (prefix: ~).
/// * Expressions that return a type implementing
/// [write_fmt](std::fmt::Write) (no prefix).
macro_rules! api {
    //exit
    (impl $buf:ident,) => {};
    //string literals
    (impl $buf:ident, $lit:literal) => {
        ::std::fmt::Write::write_fmt(&mut $buf, format_args!("{}", $lit)).expect("Failed to write api string");
    };
    (impl $buf:ident, $lit:literal, $($tail:tt)*) => {
        api!(impl $buf, $lit);
        api!(impl $buf, $($tail)*);
    };
    //types to convert using ExtractSnowflake
    (impl $buf:ident, #$snow:expr) => {
        let ext: Snowflake = ::automate::encode::ExtractSnowflake::extract_snowflake(&$snow)?;
        ::std::fmt::Write::write_fmt(&mut $buf, format_args!("{}", ext)).expect("Failed to write api string");
    };
    (impl $buf:ident, #$snow:expr, $($tail:tt)*) => {
        api!(impl $buf, #$snow);
        api!(impl $buf, $($tail)*);
    };
    //types to convert using WriteUrl
    (impl $buf:ident, ~$wurl:expr) => {
        ::automate::encode::WriteUrl::write_url($wurl, &mut $buf)?;
    };
    (impl $buf:ident, ~$wurl:expr, $($tail:tt)*) => {
        api!(impl $buf, ~$wurl);
        api!(impl $buf, $($tail)*);
    };
    //any other expression
    (impl $buf:ident, $any:expr) => {
        ::std::fmt::Write::write_fmt(&mut $buf, format_args!("{}", $any)).expect("Failed to write api string");
    };
    (impl $buf:ident, $any:expr, $($tail:tt)*) => {
        api!(impl $buf, $any);
        api!(impl $buf, $($tail)*);
    };
    //entry point
    ($($tokens:tt)*) => {&{
        let mut s = String::from("https://discordapp.com/api/v6");
        api!(impl s, $($tokens)*);
        s
    }};
}

/// Default user agent for automate bots
const USER_AGENT: &str = concat!("DiscordBot (https://github.com/mbenoukaiss/automate, ", env!("CARGO_PKG_VERSION"), ")");

/// Struct to interact with the discord HTTP API.
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

    #[inline]
    async fn send(&self, uri: &str, method: Method, body: Body) -> Result<Response<Body>, Error> {
        let response = self.client.request(Request::builder()
            .uri(uri)
            .method(method)
            .header("Content-Type", "application/json")
            .header("Authorization", &self.token)
            .header("User-Agent", USER_AGENT)
            .body(body)
            .unwrap())
            .await?;

        Ok(response)
    }

    #[inline]
    async fn request<T, R>(&self, method: Method, uri: &str, content: T) -> Result<R, Error> where T: AsJson, R: DeserializeOwned {
        let response = self.send(uri, method, Body::from(content.as_json())).await?;
        let body = hyper::body::aggregate(response).await?;

        Ok(serde_json::from_reader(body.reader())?)
    }

    #[inline]
    async fn request_code<T>(&self, method: Method, uri: &str, content: T, exp_code: u16) -> Result<(), Error> where T: AsJson {
        let response = self.send(uri, method, Body::from(content.as_json())).await?;
        let code = response.status().as_u16();

        if exp_code == code {
            Ok(())
        } else {
            Error::err(format!("Expected status code {}, got {}", exp_code, code))
        }
    }

    pub async fn gateway(&self) -> Result<Gateway, Error> {
        self.request(Method::GET, api!("/gateway"), ()).await
    }

    pub async fn gateway_bot(&self) -> Result<GatewayBot, Error> {
        self.request(Method::GET, api!("/gateway/bot"), ()).await
    }

    pub async fn guild<S: ExtractSnowflake>(&self, guild: S) -> Result<Guild, Error> {
        self.request(Method::GET, api!("/guilds/", #guild), ()).await
    }

    /// Creates a guild
    /// The first role defined in the roles vector will
    /// be used to define the permissions for `@everyone`.
    //TODO: Check if the bot is in less than 10 guilds
    //TODO: Check that channels don't have a `parent_id`
    pub async fn create_guild(&self, guild: NewGuild) -> Result<Guild, Error> {
        self.request(Method::POST, api!("/guilds"), guild).await
    }

    pub async fn modify_guild<S: ExtractSnowflake>(&self, guild: S, modification: ModifyGuild) -> Result<Guild, Error> {
        self.request(Method::PATCH, api!("/guilds/", #guild), modification).await
    }

    pub async fn delete_guild<S: ExtractSnowflake>(&self, guild: S) -> Result<(), Error> {
        self.request_code(Method::DELETE, api!("/guilds/", #guild), (), 204).await
    }

    pub async fn audit_logs<S: ExtractSnowflake>(&self, guild: S) -> Result<AuditLog, Error> {
        self.request(Method::GET, api!("/guilds/", #guild, "/audit-logs"), ()).await
    }

    pub async fn channels<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<Channel>, Error> {
        self.request(Method::GET, api!("/guilds/", #guild, "/channels"), ()).await
    }

    pub async fn channel<S: ExtractSnowflake>(&self, channel: S) -> Result<Channel, Error> {
        self.request(Method::GET, api!("/channels/", #channel), ()).await
    }

    pub async fn create_channel<S: ExtractSnowflake>(&self, guild: S, channel: NewChannel) -> Result<Channel, Error> {
        self.request(Method::POST, api!("/guilds/", #guild, "/channels"), channel).await
    }

    pub async fn modify_channel<S: ExtractSnowflake>(&self, channel: S, modification: ModifyChannel) -> Result<Channel, Error> {
        self.request(Method::PATCH, api!("/channels/", #channel), modification).await
    }

    pub async fn move_channels<S: ExtractSnowflake>(&self, guild: S, moves: Vec<MoveChannel>) -> Result<(), Error> {
        self.request_code(Method::PATCH, api!("/guilds/", #guild, "/channels"), moves, 204).await
    }

    pub async fn delete_channel<S: ExtractSnowflake>(&self, channel: S) -> Result<Channel, Error> {
        self.request(Method::DELETE, api!("/channels/", #channel), ()).await
    }

    //TODO: delete channels recursively?

    pub async fn member<S: ExtractSnowflake>(&self, guild: S, user: S) -> Result<GuildMember, Error> {
        self.request(Method::GET, api!("/guilds/", #guild, "/members/", #user), ()).await
    }

    pub async fn members<S: ExtractSnowflake>(&self, guild: S, filter: MemberFilter) -> Result<Vec<GuildMember>, Error> {
        let query = match filter {
            MemberFilter::Default => String::new(),
            MemberFilter::Limit(limit) => format!("?limit={}", limit),
            MemberFilter::After(s, limit) => format!("?after={}&limit={}", s, limit),
        };

        self.request(Method::GET, api!("/guilds/", #guild, "/members", query), ()).await
    }

    pub async fn modify_member<S: ExtractSnowflake>(&self, guild: S, user: S, member: ModifyMember) -> Result<(), Error> {
        self.request_code(Method::PATCH, api!("/guilds/", #guild, "/members/", #user), member, 204).await
    }

    pub async fn remove_member<S: ExtractSnowflake>(&self, guild: S, user: S) -> Result<(), Error> {
        self.request_code(Method::DELETE, api!("/guilds/", #guild, "/members/", #user), (), 204).await
    }

    pub async fn modify_own_nick<S: ExtractSnowflake>(&self, guild: S, nick: &str) -> Result<(), Error> {
        self.request_code(Method::PATCH, api!("/guilds/", #guild, "/members/@me/nick"), nick, 200).await
    }

    pub async fn member_add_role<S: ExtractSnowflake>(&self, guild: S, user: S, role: S) -> Result<(), Error> {
        self.request_code(Method::PUT, api!("/guilds/", #guild, "/members/", #user, "/roles/", #role), (), 204).await
    }

    pub async fn member_remove_role<S: ExtractSnowflake>(&self, guild: S, user: S, role: S) -> Result<(), Error> {
        self.request_code(Method::DELETE, api!("/guilds/", #guild, "/members/", #user, "/roles/", #role), (), 204).await
    }

    pub async fn bans<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<Ban>, Error> {
        self.request(Method::GET, api!("/guilds/", #guild, "/bans"), ()).await
    }

    pub async fn ban<S: ExtractSnowflake>(&self, guild: S, user: S) -> Result<Ban, Error> {
        self.request(Method::GET, api!("/guilds/", #guild, "/bans/", #user), ()).await
    }

    pub async fn create_ban<S: ExtractSnowflake>(&self, guild: S, user: S, reason: Option<&str>, delete_days: Option<i8>) -> Result<(), Error> {
        let mut query = String::new();

        if let Some(reason) = reason {
            query.push_str("?reason=");
            query.push_str(reason);
        }

        if let Some(delete_days) = delete_days {
            if query.is_empty() {
                query.push('?');
            } else {
                query.push('&');
            }

            query.push_str("delete-message-days=");
            query.push_str(&delete_days.to_string());
        }

        self.request_code(Method::PUT, api!("/guilds/", #guild, "/bans/", #user, query), (), 204).await
    }

    pub async fn remove_ban<S: ExtractSnowflake>(&self, guild: S, user: S) -> Result<(), Error> {
        self.request_code(Method::DELETE, api!("/guilds/", #guild, "/bans/", #user), (), 204).await
    }

    pub async fn roles<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<Role>, Error> {
        self.request(Method::GET, api!("/guilds/", #guild, "/roles"), ()).await
    }

    pub async fn create_role<S: ExtractSnowflake>(&self, guild: S, role: NewRole) -> Result<Role, Error> {
        self.request(Method::POST, api!("/guilds/", #guild, "/roles"), role).await
    }

    pub async fn modify_roles<S: ExtractSnowflake>(&self, guild: S, role: S, modification: ModifyRole) -> Result<Role, Error> {
        self.request(Method::PATCH, api!("/guilds/", #guild, "/roles/", #role), modification).await
    }

    pub async fn move_roles<S: ExtractSnowflake>(&self, guild: S, roles: Vec<MoveRole>) -> Result<Vec<Role>, Error> {
        self.request(Method::PATCH, api!("/guilds/", #guild, "/roles"), roles).await
    }

    pub async fn remove_role<S: ExtractSnowflake>(&self, guild: S, role: S) -> Result<(), Error> {
        self.request_code(Method::DELETE, api!("/guilds/", #guild, "/roles/", #role), (), 204).await
    }

    pub async fn simulate_prune<S: ExtractSnowflake>(&self, guild: S, days: i32) -> Result<Prune, Error> {
        self.request(Method::GET, api!("/guilds/", #guild, "/roles?days=", days), ()).await
    }

    pub async fn prune<S: ExtractSnowflake>(&self, guild: S, days: i32) -> Result<(), Error> {
        self.request(Method::POST, api!("/guilds/", #guild, "/prune?days=", days, "&compute_prune_count=0"), ()).await
    }

    pub async fn prune_with_report<S: ExtractSnowflake>(&self, guild: S, days: i32) -> Result<Prune, Error> {
        self.request(Method::POST, api!("/guilds/", #guild, "/prune?days=", days, "&compute_prune_count=1"), ()).await
    }

    pub async fn voice_regions(&self) -> Result<Vec<VoiceRegion>, Error> {
        self.request(Method::GET, api!("/voice/regions"), ()).await
    }

    pub async fn guild_voice_regions<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<VoiceRegion>, Error> {
        self.request(Method::GET, api!("/voice/", #guild, "/regions"), ()).await
    }

    pub async fn integrations<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<Integration>, Error> {
        self.request(Method::GET, api!("/guilds/", #guild, "/integrations"), ()).await
    }

    pub async fn embed<S: ExtractSnowflake>(&self, guild: S) -> Result<GuildEmbed, Error> {
        self.request(Method::GET, api!("/guilds/", #guild, "/embed"), ()).await
    }

    pub async fn modify_embed<S: ExtractSnowflake>(&self, guild: S, embed: GuildEmbed) -> Result<GuildEmbed, Error> {
        self.request(Method::PATCH, api!("/guilds/", #guild, "/embed"), embed).await
    }

    pub async fn vanity_url<S: ExtractSnowflake>(&self, guild: S) -> Result<PartialInvite, Error> {
        self.request(Method::GET, api!("/guilds/", #guild, "/vanity-url"), ()).await
    }

    pub async fn message<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<Message, Error> {
        self.request(Method::GET, api!("/channels/", #channel, "/messages/", #message), ()).await
    }

    pub async fn messages<S: ExtractSnowflake>(&self, channel: S, messages: MessagesPosition) -> Result<Vec<Message>, Error> {
        let query = match messages {
            MessagesPosition::Default => String::new(),
            MessagesPosition::Limit(limit) => format!("?limit={}", limit),
            MessagesPosition::Before(s, limit) => format!("?before={}&limit={}", s, limit),
            MessagesPosition::Around(s, limit) => format!("?around={}&limit={}", s, limit),
            MessagesPosition::After(s, limit) => format!("?after={}&limit={}", s, limit),
        };

        self.request(Method::GET, api!("/channels/", #channel, "/messages", query), ()).await
    }

    pub async fn create_message<S: ExtractSnowflake>(&self, channel: S, message: CreateMessage) -> Result<Message, Error> {
        self.request(Method::POST, api!("/channels/", #channel, "/messages"), message).await
    }

    pub async fn modify_message<S: ExtractSnowflake>(&self, channel: S, message: S, modification: ModifyMessage) -> Result<Message, Error> {
        self.request(Method::PATCH, api!("/channels/", #channel, "/messages", #message), modification).await
    }

    pub async fn delete_message<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<(), Error> {
        self.request_code(Method::DELETE, api!("/channels/", #channel, "/messages/", #message), (), 204).await
    }

    pub async fn delete_message_bulk<S: ExtractSnowflake + AsJson>(&self, channel: S, messages: Vec<S>) -> Result<(), Error> {
        self.request_code(Method::POST, api!("/channels/", #channel, "/messages/bulk-delete"), messages, 204).await
    }

    pub async fn reactions<S: ExtractSnowflake>(&self, channel: S, message: S, emoji: &str, reactions: ReactionsPosition) -> Result<Vec<User>, Error> {
        let query = match reactions {
            ReactionsPosition::Default => String::new(),
            ReactionsPosition::Limit(limit) => format!("?limit={}", limit),
            ReactionsPosition::Before(s, limit) => format!("?before={}&limit={}", s, limit),
            ReactionsPosition::After(s, limit) => format!("?after={}&limit={}", s, limit),
        };

        self.request(Method::GET, api!("/channels/", #channel, "/messages/", #message, "/reactions/", emoji, query), ()).await
    }

    pub async fn create_reaction<S: ExtractSnowflake, U: WriteUrl>(&self, channel: S, message: S, emoji: &U) -> Result<(), Error> {
        self.request_code(Method::PUT, api!("/channels/", #channel, "/messages/", #message, "/reactions/", ~emoji, "/@me"), "", 204).await
    }

    pub async fn delete_reaction<S: ExtractSnowflake, U: WriteUrl>(&self, channel: S, message: S, emoji: &U, user: S) -> Result<(), Error> {
        self.request_code(Method::DELETE, api!("/channels/", #channel, "/messages/", #message, "/reactions/", ~emoji, "/", #user), (), 204).await
    }

    pub async fn delete_own_reaction<S: ExtractSnowflake, U: WriteUrl>(&self, channel: S, message: S, emoji: &U) -> Result<(), Error> {
        self.request_code(Method::DELETE, api!("/channels/", #channel, "/messages/", #message, "/reactions/", ~emoji, "/@me"), (), 204).await
    }

    pub async fn delete_all_reaction<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<(), Error> {
        self.request_code(Method::DELETE, api!("/channels/", #channel, "/messages/", #message, "/reactions"), (), 204).await
    }

    pub async fn emojis<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<Emoji>, Error> {
        self.request(Method::GET, api!("/guilds/", #guild, "/emojis"), ()).await
    }

    pub async fn emoji<S: ExtractSnowflake>(&self, guild: S, emoji: S) -> Result<Emoji, Error> {
        self.request(Method::GET, api!("/guilds/", #guild, "/emojis/", #emoji), ()).await
    }

    pub async fn create_emoji<S: ExtractSnowflake>(&self, guild: S, emoji: NewEmoji) -> Result<Emoji, Error> {
        self.request(Method::POST, api!("/guilds/", #guild, "/emojis"), emoji).await
    }

    pub async fn modify_emoji<S: ExtractSnowflake>(&self, guild: S, emoji: UpdateEmoji) -> Result<Emoji, Error> {
        self.request(Method::PATCH, api!("/guilds/", #guild, "/emojis/", #emoji), emoji).await
    }

    pub async fn delete_emoji<S: ExtractSnowflake>(&self, guild: S, emoji: S) -> Result<(), Error> {
        self.request_code(Method::DELETE, api!("/guilds/", #guild, "/emojis/", #emoji), (), 204).await
    }

    /// Retrieves an invite by its code.
    pub async fn invite(&self, code: &str) -> Result<Invite, Error> {
        self.request(Method::GET, api!("/invites/", code), ()).await
    }

    /// Retrieves an invite by its code with the
    /// approximate member counts of the server.
    pub async fn invite_with_counts(&self, code: &str) -> Result<Invite, Error> {
        self.request(Method::GET, api!("/invites/", code, "?with_counts=true"), ()).await
    }

    /// Retrieves all the invites in a guild.
    pub async fn guild_invites<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<Invite>, Error> {
        self.request(Method::GET, api!("/guilds/", #guild, "/invites"), ()).await
    }

    /// Retrieves all the invites in a channel.
    pub async fn channel_invites<S: ExtractSnowflake>(&self, channel: S) -> Result<Vec<Invite>, Error> {
        self.request(Method::GET, api!("/channels/", #channel, "/invites"), ()).await
    }

    /// Create an invite for the specified channel.
    pub async fn create_invite<S: ExtractSnowflake>(&self, channel: S, invite: NewInvite) -> Result<Invite, Error> {
        self.request(Method::POST, api!("/channels/", #channel, "/invites"), invite).await
    }

    /// Create an invite for the specified channel.
    pub async fn delete_invite(&self, code: &str) -> Result<Invite, Error> {
        self.request(Method::DELETE, api!("/invites/", code), ()).await
    }

    pub async fn modify_channel_permissions<S: ExtractSnowflake>(&self, channel: S, overwrite: S, permissions: NewOverwrite) -> Result<(), Error> {
        self.request_code(Method::POST, api!("/channels/", #channel, "/permissions/", #overwrite), permissions, 204).await
    }

    pub async fn delete_channel_permission<S: ExtractSnowflake>(&self, channel: S, overwrite: S) -> Result<(), Error> {
        self.request_code(Method::DELETE, api!("/channels/", #channel, "/permissions/", #overwrite), (), 204).await
    }

    pub async fn trigger_typing<S: ExtractSnowflake>(&self, channel: S) -> Result<(), Error> {
        self.request_code(Method::POST, api!("/channels/", #channel, "/typing"), (), 204).await
    }

    pub async fn pinned_messages<S: ExtractSnowflake>(&self, channel: S) -> Result<Vec<Message>, Error> {
        self.request(Method::GET, api!("/channels/", #channel, "/pins"), ()).await
    }

    pub async fn pin_message<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<(), Error> {
        self.request_code(Method::PUT, api!("/channels/", #channel, "/pins/", #message), (), 204).await
    }

    //TODO: deletes the message or the pin?
    pub async fn delete_pinned_message<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<(), Error> {
        self.request_code(Method::DELETE, api!("/channels/", #channel, "/pins/", #message), (), 204).await
    }

    /// Returns the current user.
    pub async fn curent_user(&self) -> Result<User, Error> {
        self.request(Method::GET, api!("/users/@me"), ()).await
    }

    /// Retrieves the ids of the guilds this
    /// bot is in.
    pub async fn bot_guilds(&self) -> Result<Vec<PartialGuild>, Error> {
        self.request(Method::GET, api!("/users/@me/guilds"), ()).await
    }

    pub async fn modify_bot(&self, bot: ModifyBot) -> Result<User, Error> {
        self.request(Method::PATCH, api!("/users/@me"), bot).await
    }

    pub async fn leave_guild<S: ExtractSnowflake>(&self, guild: S) -> Result<(), Error> {
        self.request_code(Method::DELETE, api!("/users/@me/guilds/", #guild), (), 204).await
    }

    pub async fn user<S: ExtractSnowflake>(&self, user: S) -> Result<User, Error> {
        self.request(Method::GET, api!("/users/", #user), ()).await
    }

    //TODO: create a dm channel type
    pub async fn create_dm<S: ExtractSnowflake>(&self, recipient: Recipient) -> Result<Channel, Error> {
        self.request(Method::POST, api!("/users/@me/channels"), recipient).await
    }
}