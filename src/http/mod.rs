mod models;

pub use models::*;

use hyper::{Client, Request, Body, Chunk, Response, Method};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use futures::TryStreamExt;
use crate::gateway::*;
use crate::json::{FromJson, AsJson};
use crate::{json, Error, Snowflake};

/// Creates the URL to an API endpoint
/// by concatenating the given expressions.
macro_rules! api {
    ($($dest:expr),*) => {&{
        let mut s = String::from("https://discordapp.com/api/v6");
        $(::std::fmt::Write::write_fmt(&mut s, format_args!("{}", $dest)).expect("Failed to write api string");)*
        s
    }}
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

    pub async fn guild<S: Into<Snowflake>>(&self, guild: S) -> Result<Guild, Error> {
        self.get(api!("/guilds/", guild.into())).await
    }

    pub async fn create_guild(&self, guild: NewGuild) -> Result<Guild, Error> {
        self.post(api!("/guilds"), guild).await
    }

    pub async fn modify_guild<S: Into<Snowflake>>(&self, guild: S, modification: ModifyGuild) -> Result<Guild, Error> {
        self.patch(api!("/guilds/", guild), modification).await
    }

    pub async fn delete_guild<S: Into<Snowflake>>(&self, guild: S) -> Result<(), Error> {
        self.delete(api!("/guilds/", guild)).await
    }

    pub async fn audit_logs<S: Into<Snowflake>>(&self, guild: S) -> Result<AuditLog, Error> {
        self.get(api!("/guilds/", guild.into(), "/audit-logs")).await
    }

    pub async fn channels<S: Into<Snowflake>>(&self, guild: S) -> Result<Vec<Channel>, Error> {
        self.get(api!("/guilds/", guild.into(), "/channels")).await
    }

    pub async fn channel<S: Into<Snowflake>>(&self, channel: S) -> Result<Channel, Error> {
        self.get(api!("/channels/", channel.into())).await
    }

    pub async fn create_channel<S: Into<Snowflake>>(&self, guild: S, channel: NewChannel) -> Result<Channel, Error> {
        self.post(api!("/guilds/", guild.into(), "/channels"), channel).await
    }

    pub async fn modify_channel<S: Into<Snowflake>>(&self, channel: S, modification: ModifyChannel) -> Result<Channel, Error> {
        self.patch(api!("/channels/", channel.into()), modification).await
    }

    pub async fn move_channel<S: Into<Snowflake>>(&self, guild: S, moves: Vec<MoveChannel>) -> Result<(), Error> {
        self.patch(api!("/guilds/", guild.into(), "/channels"), moves).await
    }

    pub async fn delete_channel<S: Into<Snowflake>>(&self, channel: S) -> Result<Channel, Error> {
        self.delete(api!("/channels/", channel.into())).await
    }

    //TODO: delete channels recursively?

    pub async fn message<S: Into<Snowflake>>(&self, channel: S, message: S) -> Result<Message, Error> {
        self.get(api!("/channels/", channel.into(), "/messages/", message.into())).await
    }

    pub async fn messages<S: Into<Snowflake>>(&self, channel: S, messages: MessagesPosition) -> Result<Vec<Message>, Error> {
        let query = match messages {
            MessagesPosition::Default => String::new(),
            MessagesPosition::Limit(limit) => format!("?limit={}", limit),
            MessagesPosition::Before(s, limit) => format!("?before={}&limit={}", s, limit),
            MessagesPosition::Around(s, limit) => format!("?around={}&limit={}", s, limit),
            MessagesPosition::After(s, limit) => format!("?after={}&limit={}", s, limit),
        };

        self.get(api!("/channels/", channel.into(), "/messages?", query)).await
    }

    pub async fn create_message<S: Into<Snowflake>>(&self, channel: S, message: CreateMessage) -> Result<Message, Error> {
        self.post(api!("/channels/", channel.into(), "/messages"), message).await
    }

    pub async fn modify_message<S: Into<Snowflake>>(&self, channel: S, message: S, modification: ModifyMessage) -> Result<Message, Error> {
        self.patch(api!("/channels/", channel.into(), "/messages", message.into()), modification).await
    }

    pub async fn delete_message<S: Into<Snowflake>>(&self, channel: S, message: S) -> Result<(), Error> {
        self.delete(api!("/channels/", channel.into(), "/messages/", message.into())).await
    }

    pub async fn delete_message_bulk<S: Into<Snowflake> + AsJson>(&self, channel: S, messages: Vec<S>) -> Result<(), Error> {
        self.post(api!("/channels/", channel.into(), "/messages/bulk-delete"), messages).await
    }

    pub async fn reactions<S: Into<Snowflake>>(&self, channel: S, message: S, emoji: &str, reactions: ReactionsPosition) -> Result<Vec<User>, Error> {
        let query = match reactions {
            ReactionsPosition::Default => String::new(),
            ReactionsPosition::Limit(limit) => format!("?limit={}", limit),
            ReactionsPosition::Before(s, limit) => format!("?before={}&limit={}", s, limit),
            ReactionsPosition::After(s, limit) => format!("?after={}&limit={}", s, limit),
        };

        self.get(api!("/channels/", channel.into(), "/messages/", message.into(), "/reactions/", emoji, query)).await
    }

    pub async fn create_reaction<S: Into<Snowflake>, U: UrlEncode>(&self, channel: S, message: S, emoji: &U) -> Result<(), Error> {
        self.put(api!("/channels/", channel.into(), "/messages/", message.into(), "/reactions/", emoji.encode(), "/@me"), ()).await
    }

    pub async fn delete_reaction<S: Into<Snowflake>, U: UrlEncode>(&self, channel: S, message: S, emoji: &U, user: S) -> Result<(), Error> {
        self.delete(api!("/channels/", channel.into(), "/messages/", message.into(), "/reactions/", emoji.encode(), "/", user.into())).await
    }

    pub async fn delete_own_reaction<S: Into<Snowflake>, U: UrlEncode>(&self, channel: S, message: S, emoji: &U) -> Result<(), Error> {
        self.delete(api!("/channels/", channel.into(), "/messages/", message.into(), "/reactions/", emoji.encode(), "/@me")).await
    }

    pub async fn delete_all_reaction<S: Into<Snowflake>>(&self, channel: S, message: S) -> Result<(), Error> {
        self.delete(api!("/channels/", channel.into(), "/messages/", message.into(), "/reactions")).await
    }

    pub async fn emojis<S: Into<Snowflake>>(&self, guild: S) -> Result<Vec<Emoji>, Error> {
        self.get(api!("/guilds/", guild.into(), "/emojis")).await
    }

    pub async fn emoji<S: Into<Snowflake>>(&self, guild: S, emoji: S) -> Result<Emoji, Error> {
        self.get(api!("/guilds/", guild.into(), "/emojis/", emoji.into())).await
    }

    pub async fn create_emoji<S: Into<Snowflake>>(&self, guild: S, emoji: NewEmoji) -> Result<Emoji, Error> {
        self.post(api!("/guilds/", guild.into(), "/emojis"), emoji).await
    }

    pub async fn modify_emoji<S: Into<Snowflake>>(&self, guild: S, emoji: UpdateEmoji) -> Result<Emoji, Error> {
        self.patch(api!("/guilds/", guild.into(), "/emojis/", emoji.id), emoji).await
    }

    pub async fn delete_emoji<S: Into<Snowflake>>(&self, guild: S, emoji: S) -> Result<(), Error> {
        self.delete(api!("/guilds/", guild.into(), "/emojis/", emoji.into())).await
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
    pub async fn invites<S: Into<Snowflake>>(&self, channel: S) -> Result<Vec<Invite>, Error> {
        self.get(api!("/channels/", channel.into(), "/invites")).await
    }

    /// Create an invite for the specified channel.
    pub async fn create_invite<S: Into<Snowflake>>(&self, channel: S, invite: NewInvite) -> Result<Invite, Error> {
        self.post(api!("/channels/", channel.into(), "/invites"), invite).await
    }

    /// Create an invite for the specified channel.
    pub async fn delete_invite(&self, code: &str) -> Result<Invite, Error> {
        self.delete(api!("/invites/", code)).await
    }

    pub async fn modify_channel_permissions<S: Into<Snowflake>>(&self, channel: S, overwrite: S, permissions: NewOverwrite) -> Result<(), Error> {
        self.post(api!("/channels/", channel.into(), "/permissions/", overwrite.into()), permissions).await
    }

    pub async fn delete_channel_permission<S: Into<Snowflake>>(&self, channel: S, overwrite: S) -> Result<(), Error> {
        self.delete(api!("/channels/", channel.into(), "/permissions/", overwrite.into())).await
    }

    pub async fn trigger_typing<S: Into<Snowflake>>(&self, channel: S) -> Result<(), Error> {
        self.post(api!("/channels/", channel.into(), "/typing"), ()).await
    }

    pub async fn pinned_messages<S: Into<Snowflake>>(&self, channel: S) -> Result<Vec<Message>, Error> {
        self.get(api!("/channels/", channel.into(), "/pins")).await
    }

    pub async fn pin_message<S: Into<Snowflake>>(&self, channel: S, message: S) -> Result<(), Error> {
        self.put(api!("/channels/", channel.into(), "/pins/", message.into()), ()).await
    }

    //TODO: deletes the message or the pin?
    pub async fn delete_pinned_message<S: Into<Snowflake>>(&self, channel: S, message: S) -> Result<(), Error> {
        self.delete(api!("/channels/", channel.into(), "/pins/", message.into())).await
    }

    pub async fn group_dm_add_recipient<S: Into<Snowflake>>(&self, channel: S, user: S, access_token: String, nick: String) -> Result<(), Error> {
        self.put(api!("/channels/", channel.into(), "/recipients/", user.into()), json! {
            "access_token" => access_token,
            "nick" => nick
        }).await
    }

    pub async fn group_dm_remove_recipient<S: Into<Snowflake>>(&self, channel: S, user: S) -> Result<(), Error> {
        self.delete(api!("/channels/", channel.into(), "/recipients/", user.into())).await
    }
}