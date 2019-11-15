use crate::{FromJson, Error, AsJson};
use hyper::{Client, Request, Body, Chunk, Response, Uri, Method};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use futures::TryStreamExt;
use crate::models::{Gateway, GatewayBot, CreateMessage, Message, Channel, ModifyChannel};

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

    //TODO: handle query string params
    //pub async fn get_channel_messages(&self, channel_id: u64, messages: GetChannelMessages) -> Result<Vec<Message>, Error> {
    //    self.get(api!("/channels/", channel_id, "/messages")).await
    //}

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

    //TODO: handle query string params
    //pub async fn get_reactions(&self, channel_id: u64, message_id: u64, emoji: &str) -> Result<Vec<User>, Error> {
    //    self.get(api!("/channels/", channel_id, "/messages/", message_id, "/reactions/", emoji)).await
    //}

    pub async fn delete_all_reaction(&self, channel_id: u64, message_id: u64) -> Result<(), Error> {
        self.delete(api!("/channels/", channel_id, "/messages/", message_id, "/reactions")).await
    }

    pub async fn edit_message(&self, channel_id: u64, message_id: u64, message: CreateMessage) -> Result<Message, Error> {
        self.patch(api!("/channels/", channel_id, "/messages", message_id), message).await
    }

    pub async fn delete_message(&self, channel_id: u64, message_id: u64) -> Result<(), Error> {
        self.delete(api!("/channels/", channel_id, "/messages/", message_id)).await
    }

    pub async fn delete_message_bulk(&self, channel_id: u64, messages: Vec<u64>) -> Result<(), Error> {
        self.post(api!("/channels/", channel_id, "/messages/bulk-delete"), messages).await
    }

}