mod models;

pub use models::*;

use hyper::{Client, Request, Body, Chunk, Response, Method};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use futures::TryStreamExt;
use crate::gateway::*;
use crate::encode::{FromJson, AsJson};
use crate::{json, Error, Snowflake};
use crate::encode::{ExtractSnowflake, WriteUrl};

/// Creates the URL to an API endpoint
/// by concatenating the given expressions.
/// This macro accepts three kinds of arguments:
/// * String literals, which are simply concatenated to
/// the final string
/// * Types implementing the WriteUrl type, which will
/// be appended to the final string by calling their
/// [write_url](automate::encode::WriteUrl::write_url)
/// method. Useful for types that require a specific
/// formatting or for strings that need to be escaped
/// * Expressions that return a type implementing
/// [write_fmt](std::fmt::Write).
macro_rules! api {
    ($($tokens:tt)*) => {&{
        let mut s = String::from("https://discordapp.com/api/v6");
        api_ttmuncher!(s, $($tokens)*);
        s
    }}
}

macro_rules! api_ttmuncher {
    ($buf:ident,) => {};
    //string literals
    ($buf:ident, $lit:literal) => {
        ::std::fmt::Write::write_fmt(&mut $buf, format_args!("{}", $lit)).expect("Failed to write api string");
    };
    ($buf:ident, $lit:literal, $($tail:tt)*) => {
        api_ttmuncher!($buf, $lit);
        api_ttmuncher!($buf, $($tail)*);
    };
    //types to convert using ExtractSnowflake
    ($buf:ident, #$snow:expr) => {
        let ext: Snowflake = ::automate::encode::ExtractSnowflake::extract_snowflake(&$snow)?;
        ::std::fmt::Write::write_fmt(&mut $buf, format_args!("{}", ext)).expect("Failed to write api string");
    };
    ($buf:ident, #$snow:expr, $($tail:tt)*) => {
        api_ttmuncher!($buf, #$snow);
        api_ttmuncher!($buf, $($tail)*);
    };
    //types to convert using WriteUrl
    ($buf:ident, ~$wurl:expr) => {
        ::automate::encode::WriteUrl::write_url($wurl, &mut $buf)?;
    };
    ($buf:ident, ~$wurl:expr, $($tail:tt)*) => {
        api_ttmuncher!($buf, ~$wurl);
        api_ttmuncher!($buf, $($tail)*);
    };
    //any other expression
    ($buf:ident, $any:expr) => {
        ::std::fmt::Write::write_fmt(&mut $buf, format_args!("{}", $any)).expect("Failed to write api string");
    };
    ($buf:ident, $any:expr, $($tail:tt)*) => {
        api_ttmuncher!($buf, $any);
        api_ttmuncher!($buf, $($tail)*);
    };
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
        let https = HttpsConnector::new().unwrap();

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
    async fn request(&self, uri: &str, method: Method, body: Body) -> Result<Response<Body>, Error> {
        let response = self.client.request(Request::builder()
            .uri(uri)
            .method(method)
            .header("Content-Type", "application/json")
            .header("Authorization", &self.token)
            .header("User-Agent", USER_AGENT)
            .body(body)
            .unwrap()).await?;

        Ok(response)
    }

    #[inline]
    async fn get<T>(&self, uri: &str) -> Result<T, Error> where T: FromJson {
        let body: Chunk = self.request(uri, Method::GET, Body::empty()).await?.into_body().try_concat().await?;

        if let Ok(json) = std::str::from_utf8(body.as_ref()) {
            Ok(T::from_json(json)?)
        } else {
            Error::err("Failed to convert response body to a string")
        }
    }

    #[inline]
    async fn post<T, R>(&self, uri: &str, content: T) -> Result<R, Error> where T: AsJson, R: FromJson {
        let body: Chunk = self.request(uri, Method::POST, Body::from(content.as_json())).await?.into_body().try_concat().await?;

        if let Ok(json) = std::str::from_utf8(body.as_ref()) {
            Ok(R::from_json(json)?)
        } else {
            Error::err("Failed to convert response body to a string")
        }
    }

    #[inline]
    async fn put<T, R>(&self, uri: &str, content: T) -> Result<R, Error> where T: AsJson, R: FromJson {
        let body: Chunk = self.request(uri, Method::PUT, Body::from(content.as_json())).await?.into_body().try_concat().await?;

        if let Ok(json) = std::str::from_utf8(body.as_ref()) {
            Ok(R::from_json(json)?)
        } else {
            Error::err("Failed to convert response body to a string")
        }
    }

    #[inline]
    async fn patch<T, R>(&self, uri: &str, content: T) -> Result<R, Error> where T: AsJson, R: FromJson {
        let body: Chunk = self.request(uri, Method::PATCH, Body::from(content.as_json())).await?.into_body().try_concat().await?;

        if let Ok(json) = std::str::from_utf8(body.as_ref()) {
            Ok(R::from_json(json)?)
        } else {
            Error::err("Failed to convert response body to a string")
        }
    }

    #[inline]
    async fn delete<T>(&self, uri: &str) -> Result<T, Error> where T: FromJson {
        let body: Chunk = self.request(uri, Method::DELETE, Body::empty()).await?.into_body().try_concat().await?;

        if let Ok(json) = std::str::from_utf8(body.as_ref()) {
            Ok(T::from_json(json)?)
        } else {
            Error::err("Failed to convert response body to a string")
        }
    }

    pub async fn gateway(&self) -> Result<Gateway, Error> {
        self.get(api!("/gateway")).await
    }

    pub async fn gateway_bot(&self) -> Result<GatewayBot, Error> {
        self.get(api!("/gateway/bot")).await
    }

    pub async fn guild<S: ExtractSnowflake>(&self, guild: S) -> Result<Guild, Error> {
        self.get(api!("/guilds/", #guild)).await
    }

    pub async fn create_guild(&self, guild: NewGuild) -> Result<Guild, Error> {
        self.post(api!("/guilds"), guild).await
    }

    pub async fn modify_guild<S: ExtractSnowflake>(&self, guild: S, modification: ModifyGuild) -> Result<Guild, Error> {
        self.patch(api!("/guilds/", #guild), modification).await
    }

    pub async fn delete_guild<S: ExtractSnowflake>(&self, guild: S) -> Result<(), Error> {
        self.delete(api!("/guilds/", #guild)).await
    }

    pub async fn audit_logs<S: ExtractSnowflake>(&self, guild: S) -> Result<AuditLog, Error> {
        self.get(api!("/guilds/", #guild, "/audit-logs")).await
    }

    pub async fn channels<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<Channel>, Error> {
        self.get(api!("/guilds/", #guild, "/channels")).await
    }

    pub async fn channel<S: ExtractSnowflake>(&self, channel: S) -> Result<Channel, Error> {
        self.get(api!("/channels/", #channel)).await
    }

    pub async fn create_channel<S: ExtractSnowflake>(&self, guild: S, channel: NewChannel) -> Result<Channel, Error> {
        self.post(api!("/guilds/", #guild, "/channels"), channel).await
    }

    pub async fn modify_channel<S: ExtractSnowflake>(&self, channel: S, modification: ModifyChannel) -> Result<Channel, Error> {
        self.patch(api!("/channels/", #channel), modification).await
    }

    pub async fn move_channel<S: ExtractSnowflake>(&self, guild: S, moves: Vec<MoveChannel>) -> Result<(), Error> {
        self.patch(api!("/guilds/", #guild, "/channels"), moves).await
    }

    pub async fn delete_channel<S: ExtractSnowflake>(&self, channel: S) -> Result<Channel, Error> {
        self.delete(api!("/channels/", #channel)).await
    }

    //TODO: delete channels recursively?

    pub async fn message<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<Message, Error> {
        self.get(api!("/channels/", #channel, "/messages/", #message)).await
    }

    pub async fn messages<S: ExtractSnowflake>(&self, channel: S, messages: MessagesPosition) -> Result<Vec<Message>, Error> {
        let query = match messages {
            MessagesPosition::Default => String::new(),
            MessagesPosition::Limit(limit) => format!("?limit={}", limit),
            MessagesPosition::Before(s, limit) => format!("?before={}&limit={}", s, limit),
            MessagesPosition::Around(s, limit) => format!("?around={}&limit={}", s, limit),
            MessagesPosition::After(s, limit) => format!("?after={}&limit={}", s, limit),
        };

        self.get(api!("/channels/", #channel, "/messages?", query)).await
    }

    pub async fn create_message<S: ExtractSnowflake>(&self, channel: S, message: CreateMessage) -> Result<Message, Error> {
        self.post(api!("/channels/", #channel, "/messages"), message).await
    }

    pub async fn modify_message<S: ExtractSnowflake>(&self, channel: S, message: S, modification: ModifyMessage) -> Result<Message, Error> {
        self.patch(api!("/channels/", #channel, "/messages", #message), modification).await
    }

    pub async fn delete_message<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<(), Error> {
        self.delete(api!("/channels/", #channel, "/messages/", #message)).await
    }

    pub async fn delete_message_bulk<S: ExtractSnowflake + AsJson>(&self, channel: S, messages: Vec<S>) -> Result<(), Error> {
        self.post(api!("/channels/", #channel, "/messages/bulk-delete"), messages).await
    }

    pub async fn reactions<S: ExtractSnowflake>(&self, channel: S, message: S, emoji: &str, reactions: ReactionsPosition) -> Result<Vec<User>, Error> {
        let query = match reactions {
            ReactionsPosition::Default => String::new(),
            ReactionsPosition::Limit(limit) => format!("?limit={}", limit),
            ReactionsPosition::Before(s, limit) => format!("?before={}&limit={}", s, limit),
            ReactionsPosition::After(s, limit) => format!("?after={}&limit={}", s, limit),
        };

        self.get(api!("/channels/", #channel, "/messages/", #message, "/reactions/", emoji, query)).await
    }

    pub async fn create_reaction<S: ExtractSnowflake, U: WriteUrl>(&self, channel: S, message: S, emoji: &U) -> Result<(), Error> {
        self.put(api!("/channels/", #channel, "/messages/", #message, "/reactions/", ~emoji, "/@me"), ()).await
    }

    pub async fn delete_reaction<S: ExtractSnowflake, U: WriteUrl>(&self, channel: S, message: S, emoji: &U, user: S) -> Result<(), Error> {
        self.delete(api!("/channels/", #channel, "/messages/", #message, "/reactions/", ~emoji, "/", #user)).await
    }

    pub async fn delete_own_reaction<S: ExtractSnowflake, U: WriteUrl>(&self, channel: S, message: S, emoji: &U) -> Result<(), Error> {
        self.delete(api!("/channels/", #channel, "/messages/", #message, "/reactions/", ~emoji, "/@me")).await
    }

    pub async fn delete_all_reaction<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<(), Error> {
        self.delete(api!("/channels/", #channel, "/messages/", #message, "/reactions")).await
    }

    pub async fn emojis<S: ExtractSnowflake>(&self, guild: S) -> Result<Vec<Emoji>, Error> {
        self.get(api!("/guilds/", #guild, "/emojis")).await
    }

    pub async fn emoji<S: ExtractSnowflake>(&self, guild: S, emoji: S) -> Result<Emoji, Error> {
        self.get(api!("/guilds/", #guild, "/emojis/", #emoji)).await
    }

    pub async fn create_emoji<S: ExtractSnowflake>(&self, guild: S, emoji: NewEmoji) -> Result<Emoji, Error> {
        self.post(api!("/guilds/", #guild, "/emojis"), emoji).await
    }

    pub async fn modify_emoji<S: ExtractSnowflake>(&self, guild: S, emoji: UpdateEmoji) -> Result<Emoji, Error> {
        self.patch(api!("/guilds/", #guild, "/emojis/", #emoji), emoji).await
    }

    pub async fn delete_emoji<S: ExtractSnowflake>(&self, guild: S, emoji: S) -> Result<(), Error> {
        self.delete(api!("/guilds/", #guild, "/emojis/", #emoji)).await
    }

    /// Retrieves an invite by its code.
    pub async fn invite(&self, code: &str) -> Result<Invite, Error> {
        self.get(api!("/invites/", code)).await
    }

    /// Retrieves an invite by its code with the
    /// approximate member counts of the server.
    pub async fn invite_with_counts(&self, code: &str) -> Result<Invite, Error> {
        self.get(api!("/invites/", code, "?with_counts=true")).await
    }

    /// Retrieves all the invites in a channel.
    pub async fn invites<S: ExtractSnowflake>(&self, channel: S) -> Result<Vec<Invite>, Error> {
        self.get(api!("/channels/", #channel, "/invites")).await
    }

    /// Create an invite for the specified channel.
    pub async fn create_invite<S: ExtractSnowflake>(&self, channel: S, invite: NewInvite) -> Result<Invite, Error> {
        self.post(api!("/channels/", #channel, "/invites"), invite).await
    }

    /// Create an invite for the specified channel.
    pub async fn delete_invite(&self, code: &str) -> Result<Invite, Error> {
        self.delete(api!("/invites/", code)).await
    }

    pub async fn modify_channel_permissions<S: ExtractSnowflake>(&self, channel: S, overwrite: S, permissions: NewOverwrite) -> Result<(), Error> {
        self.post(api!("/channels/", #channel, "/permissions/", #overwrite), permissions).await
    }

    pub async fn delete_channel_permission<S: ExtractSnowflake>(&self, channel: S, overwrite: S) -> Result<(), Error> {
        self.delete(api!("/channels/", #channel, "/permissions/", #overwrite)).await
    }

    pub async fn trigger_typing<S: ExtractSnowflake>(&self, channel: S) -> Result<(), Error> {
        self.post(api!("/channels/", #channel, "/typing"), ()).await
    }

    pub async fn pinned_messages<S: ExtractSnowflake>(&self, channel: S) -> Result<Vec<Message>, Error> {
        self.get(api!("/channels/", #channel, "/pins")).await
    }

    pub async fn pin_message<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<(), Error> {
        self.put(api!("/channels/", #channel, "/pins/", #message), ()).await
    }

    //TODO: deletes the message or the pin?
    pub async fn delete_pinned_message<S: ExtractSnowflake>(&self, channel: S, message: S) -> Result<(), Error> {
        self.delete(api!("/channels/", #channel, "/pins/", #message)).await
    }

    pub async fn group_dm_add_recipient<S: ExtractSnowflake>(&self, channel: S, user: S, access_token: String, nick: String) -> Result<(), Error> {
        self.put(api!("/channels/", #channel, "/recipients/", #user), json! {
            "access_token" => access_token,
            "nick" => nick
        }).await
    }

    pub async fn group_dm_remove_recipient<S: ExtractSnowflake>(&self, channel: S, user: S) -> Result<(), Error> {
        self.delete(api!("/channels/", #channel, "/recipients/", #user)).await
    }
}