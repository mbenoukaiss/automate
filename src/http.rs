use crate::{FromJson, Error, AsJson};
use hyper::{Client, Request, Body, Chunk, Response, Uri, Method};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use futures::TryStreamExt;
use crate::models::{Gateway, GatewayBot, CreateMessage, Message, Channel, ModifyChannel, MessagesPosition, ReactionsPosition, User, EditMessage, NewOverwrite, NewInvite, Invite, Emoji, NewEmoji, UpdateEmoji};
use crate::json;

macro_rules! api {
    ($($dest:expr),*) => {{
        let mut s = String::from("https://discordapp.com/api/v6");
        $(::std::fmt::Write::write_fmt(&mut s, format_args!("{}", $dest)).expect("Failed to write api string");)*
        s.parse::<::hyper::Uri>().expect("Invalid API route")
    }}
}

const USER_AGENT: &str = concat!("DiscordBot (https://github.com/mbenoukaiss/automate, ", env!("CARGO_PKG_VERSION"), ")");

#[derive(Clone)]
pub struct HttpAPI {
    client: Client<HttpsConnector<HttpConnector>>,
    token: String,
}

impl HttpAPI {
    pub fn new(token: &str) -> HttpAPI {
        let https = HttpsConnector::new().unwrap();

        let mut bot_token = String::from("Bot ");
        bot_token.push_str(token);

        HttpAPI {
            client: Client::builder().build(https),
            token: bot_token,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    async fn request(&self, uri: Uri, method: Method, body: Body) -> Result<Response<Body>, hyper::Error> {
        self.client.request(Request::builder()
            .uri(uri)
            .method(method)
            .header("Content-Type", "application/json")
            .header("Authorization", &self.token)
            .header("User-Agent", USER_AGENT)
            .body(body)
            .unwrap()).await
    }

    pub async fn get<T>(&self, uri: Uri) -> Result<T, Error> where T: FromJson {
        let body: Chunk = self.request(uri, Method::GET, Body::empty()).await?.into_body().try_concat().await?;

        if let Ok(json) = std::str::from_utf8(body.as_ref()) {
            Ok(T::from_json(json)?)
        } else {
            Error::err("Failed to convert response body to a string")
        }
    }

    pub async fn post<T, U>(&self, uri: Uri, content: T) -> Result<U, Error> where T: AsJson, U: FromJson {
        let body: Chunk = self.request(uri, Method::POST, Body::from(content.as_json())).await?.into_body().try_concat().await?;

        if let Ok(json) = std::str::from_utf8(body.as_ref()) {
            Ok(U::from_json(json)?)
        } else {
            Error::err("Failed to convert response body to a string")
        }
    }

    pub async fn put<T, U>(&self, uri: Uri, content: T) -> Result<U, Error> where T: AsJson, U: FromJson {
        let body: Chunk = self.request(uri, Method::PUT, Body::from(content.as_json())).await?.into_body().try_concat().await?;

        if let Ok(json) = std::str::from_utf8(body.as_ref()) {
            Ok(U::from_json(json)?)
        } else {
            Error::err("Failed to convert response body to a string")
        }
    }

    pub async fn delete<T>(&self, uri: Uri) -> Result<T, Error> where T: FromJson {
        let body: Chunk = self.request(uri, Method::DELETE, Body::empty()).await?.into_body().try_concat().await?;

        if let Ok(json) = std::str::from_utf8(body.as_ref()) {
            Ok(T::from_json(json)?)
        } else {
            Error::err("Failed to convert response body to a string")
        }
    }

    pub async fn patch<T, U>(&self, uri: Uri, content: T) -> Result<U, Error> where T: AsJson, U: FromJson {
        let body: Chunk = self.request(uri, Method::PATCH, Body::from(content.as_json())).await?.into_body().try_concat().await?;

        if let Ok(json) = std::str::from_utf8(body.as_ref()) {
            Ok(U::from_json(json)?)
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

    pub async fn get_channel(&self, channel_id: u64) -> Result<Channel, Error> {
        self.get(api!("/channels/", channel_id)).await
    }

    pub async fn modify_channel(&self, channel_id: u64, channel: ModifyChannel) -> Result<Channel, Error> {
        self.patch(api!("/channels/", channel_id), channel).await
    }

    pub async fn delete_channel(&self, channel_id: u64) -> Result<Channel, Error> {
        self.delete(api!("/channels/", channel_id)).await
    }

    //TODO: delete channels recursively?

    pub async fn get_channel_messages(&self, channel_id: u64, messages: MessagesPosition) -> Result<Vec<Message>, Error> {
        let query = match messages {
            MessagesPosition::Default => String::new(),
            MessagesPosition::Limit(limit) => format!("?limit={}", limit),
            MessagesPosition::Before(s, limit) => format!("?before={}&limit={}", s, limit),
            MessagesPosition::Around(s, limit) => format!("?around={}&limit={}", s, limit),
            MessagesPosition::After(s, limit) => format!("?after={}&limit={}", s, limit),
        };

        self.get(api!("/channels/", channel_id, "/messages?", query)).await
    }

    pub async fn get_channel_message(&self, channel_id: u64, message_id: u64) -> Result<Message, Error> {
        self.get(api!("/channels/", channel_id, "/messages/", message_id)).await
    }

    pub async fn create_message(&self, channel_id: u64, message: CreateMessage) -> Result<Message, Error> {
        self.post(api!("/channels/", channel_id, "/messages"), message).await
    }

    pub async fn create_reaction(&self, channel_id: u64, message_id: u64, emoji: &str) -> Result<(), Error> {
        self.put(api!("/channels/", channel_id, "/messages/", message_id, "/reactions/", emoji, "/@me"), ()).await
    }

    pub async fn delete_own_reaction(&self, channel_id: u64, message_id: u64, emoji: &str) -> Result<(), Error> {
        self.delete(api!("/channels/", channel_id, "/messages/", message_id, "/reactions/", emoji, "/@me")).await
    }

    pub async fn delete_reaction(&self, channel_id: u64, message_id: u64, emoji: &str, user_id: u64) -> Result<(), Error> {
        self.delete(api!("/channels/", channel_id, "/messages/", message_id, "/reactions/", emoji, "/", user_id)).await
    }

    pub async fn get_reactions(&self, channel_id: u64, message_id: u64, emoji: &str, reactions: ReactionsPosition) -> Result<Vec<User>, Error> {
        let query = match reactions {
            ReactionsPosition::Default => String::new(),
            ReactionsPosition::Limit(limit) => format!("?limit={}", limit),
            ReactionsPosition::Before(s, limit) => format!("?before={}&limit={}", s, limit),
            ReactionsPosition::After(s, limit) => format!("?after={}&limit={}", s, limit),
        };

        self.get(api!("/channels/", channel_id, "/messages/", message_id, "/reactions/", emoji, query)).await
    }

    pub async fn delete_all_reaction(&self, channel_id: u64, message_id: u64) -> Result<(), Error> {
        self.delete(api!("/channels/", channel_id, "/messages/", message_id, "/reactions")).await
    }

    pub async fn edit_message(&self, channel_id: u64, message_id: u64, message: EditMessage) -> Result<Message, Error> {
        self.patch(api!("/channels/", channel_id, "/messages", message_id), message).await
    }

    pub async fn delete_message(&self, channel_id: u64, message_id: u64) -> Result<(), Error> {
        self.delete(api!("/channels/", channel_id, "/messages/", message_id)).await
    }

    pub async fn delete_message_bulk(&self, channel_id: u64, messages: Vec<u64>) -> Result<(), Error> {
        self.post(api!("/channels/", channel_id, "/messages/bulk-delete"), messages).await
    }

    pub async fn edit_channel_permissions(&self, channel_id: u64, overwrite_id: u64, permissions: NewOverwrite) -> Result<(), Error> {
        self.post(api!("/channels/", channel_id, "/permissions/", overwrite_id), permissions).await
    }

    /// Retrieves an invite by its code.
    pub async fn get_invite(&self, code: &str) -> Result<Invite, Error> {
        self.get(api!("/invites/", code)).await
    }

    /// Retrieves an invite by its code with the
    /// approximate member counts of the server.
    pub async fn get_invite_with_counts(&self, code: &str) -> Result<Invite, Error> {
        self.get(api!("/invites/", code, "?with_counts=true")).await
    }

    /// Retrieves all the invites in a channel.
    pub async fn get_invites(&self, channel_id: u64) -> Result<Vec<Invite>, Error> {
        self.get(api!("/channels/", channel_id, "/invites")).await
    }

    /// Create an invite for the specified channel.
    pub async fn create_invite(&self, channel_id: u64, invite: NewInvite) -> Result<Invite, Error> {
        self.post(api!("/channels/", channel_id, "/invites"), invite).await
    }

    /// Create an invite for the specified channel.
    pub async fn delete_invite(&self, code: &str) -> Result<Invite, Error> {
        self.delete(api!("/invites/", code)).await
    }

    pub async fn delete_channel_permission(&self, channel_id: u64, overwrite_id: u64) -> Result<(), Error> {
        self.delete(api!("/channels/", channel_id, "/permissions/", overwrite_id)).await
    }

    pub async fn trigger_typing_indicator(&self, channel_id: u64) -> Result<(), Error> {
        self.post(api!("/channels/", channel_id, "/typing"), ()).await
    }

    pub async fn get_pinned_messages(&self, channel_id: u64) -> Result<Vec<Message>, Error> {
        self.get(api!("/channels/", channel_id, "/pins")).await
    }

    pub async fn add_pinned_message(&self, channel_id: u64, message_id: u64) -> Result<(), Error> {
        self.put(api!("/channels/", channel_id, "/pins/", message_id), ()).await
    }

    pub async fn delete_pinned_message(&self, channel_id: u64, message_id: u64) -> Result<(), Error> {
        self.delete(api!("/channels/", channel_id, "/pins/", message_id)).await
    }

    pub async fn group_dm_add_recipient(&self, channel_id: u64, user_id: u64, access_token: String, nick: String) -> Result<(), Error> {
        self.put(api!("/channels/", channel_id, "/recipients/", user_id), json! {
            "access_token" => access_token,
            "nick" => nick
        }).await
    }

    pub async fn group_dm_remove_recipient(&self, channel_id: u64, user_id: u64) -> Result<(), Error> {
        self.delete(api!("/channels/", channel_id, "/recipients/", user_id)).await
    }

    pub async fn list_guild_emojis(&self, guild_id: u64) -> Result<Vec<Emoji>, Error> {
        self.get(api!("/guilds/", guild_id, "/emojis")).await
    }

    pub async fn get_guild_emoji(&self, guild_id: u64, emoji_id: u64) -> Result<Emoji, Error> {
        self.get(api!("/guilds/", guild_id, "/emojis/", emoji_id)).await
    }

    pub async fn create_guild_emoji(&self, guild_id: u64, emoji: NewEmoji) -> Result<Emoji, Error> {
        self.post(api!("/guilds/", guild_id, "/emojis"), emoji).await
    }

    pub async fn modify_guild_emoji(&self, guild_id: u64, emoji: UpdateEmoji) -> Result<Emoji, Error> {
        self.patch(api!("/guilds/", guild_id, "/emojis/", emoji.id), emoji).await
    }

    pub async fn delete_guild_emoji(&self, guild_id: u64, emoji_id: u64) -> Result<(), Error> {
        self.delete(api!("/guilds/", guild_id, "/emojis/", emoji_id)).await
    }

}