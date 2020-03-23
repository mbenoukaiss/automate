use chrono::NaiveDateTime;
use hyper::HeaderMap;
use hyper::header::HeaderValue;
use std::collections::HashMap;
use futures::lock::Mutex;
use crate::{Error, Snowflake};
use std::borrow::Cow;

lazy_static::lazy_static! {
    /// Global rate-limit buckets storage.
    /// Associates the bot token, the bucket id and the
    /// major parameter to the bucket data provided
    /// by Discord.
    pub static ref BUCKETS: Mutex<HashMap<Key<'static>, Bucket>> = Mutex::default();
}

/// Groups the API token, bucket id and major
/// parameter for lookup and insertions in the
/// global buckets storage.
///
/// Helps to avoid copying token and bucket id
/// just for lookup.
#[derive(Hash, PartialEq, Eq)]
pub struct Key<'a> {
    token: Cow<'a, str>,
    bucket: Cow<'a, str>,
    major: Option<Snowflake>
}

impl<'a> Key<'a> {
    /// Creates a key meant for lookup in
    /// the global bucket storage.
    pub fn lookup(token: &'a str, bucket: &'a str, major: Option<Snowflake>) -> Key<'a> {
        Key {
            token: Cow::Borrowed(token),
            bucket: Cow::Borrowed(bucket),
            major
        }
    }

    /// Creates a key meant for insertion
    /// in the global bucket storage.
    /// Both token and bucket will be cloned,
    /// do not use this for lookup.
    pub fn insert(token: String, bucket: String, major: Option<Snowflake>) -> Key<'a> {
        Key {
            token: Cow::Owned(token),
            bucket: Cow::Owned(bucket),
            major
        }
    }
}

/// Bucket data provided by discord
/// in HTTP endpoint headers.
/// More information on [Discord's rate-limit documentation](https://discordapp.com/developers/docs/topics/rate-limits).
pub struct Bucket {
    pub id: String,
    pub limit: u16,
    pub remaining: u16,
    pub reset: NaiveDateTime,
}

impl Bucket {
    /// Creates a bucket from the headers
    /// returned by an HTTP API call.
    pub fn new(headers: &HeaderMap<HeaderValue>) -> Result<Option<Bucket>, Error> {
        let bucket: Option<&HeaderValue> = headers.get("x-ratelimit-bucket");
        let limit: Option<&HeaderValue> = headers.get("x-ratelimit-limit");
        let remaining: Option<&HeaderValue> = headers.get("x-ratelimit-remaining");
        let reset: Option<&HeaderValue> = headers.get("x-ratelimit-reset");

        if let (Some(bucket), Some(limit), Some(remaining), Some(reset)) = (bucket, limit, remaining, reset) {
            let bucket = Bucket {
                id: bucket.to_str()?.to_owned(),
                limit: limit.to_str()?.parse::<u16>()?,
                remaining: remaining.to_str()?.parse::<u16>()?,
                reset: NaiveDateTime::parse_from_str(reset.to_str()?, "%s").unwrap(),
            };

            Ok(Some(bucket))
        } else {
            Ok(None)
        }
    }
}