use chrono::{NaiveDateTime, Utc};
use hyper::HeaderMap;
use hyper::header::HeaderValue;
use std::collections::HashMap;
use futures::lock::Mutex;
use std::borrow::Cow;
use crate::{Error, Snowflake};
use std::time::Instant;

lazy_static::lazy_static! {
    /// Rate-limit buckets storage.
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
#[derive(Debug)] //TODO: remove debug derive
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
            let reset = {
                let reset = reset.to_str().unwrap();
                let mut split_reset = reset.split('.');
                let secs = split_reset.next().unwrap().parse::<i64>().unwrap();

                if let Some(m) = split_reset.next() {
                    NaiveDateTime::from_timestamp(secs, m.parse::<u32>().unwrap() * 1_000_000)
                } else {
                    NaiveDateTime::from_timestamp(secs, 0)
                }
            };

            let bucket = Bucket {
                id: bucket.to_str()?.to_owned(),
                limit: limit.to_str()?.parse::<u16>()?,
                remaining: remaining.to_str()?.parse::<u16>()?,
                reset,
            };

            Ok(Some(bucket))
        } else {
            Ok(None)
        }
    }
}

/// Cleans up the bucket hashmap by removing every bucket
/// that contains a date inferior to now.
pub async fn collect_outdated_buckets() {
    let mut removes = 0;

    let now: NaiveDateTime = Utc::now().naive_utc();

    let time = {
        let start = Instant::now();
        let buckets: &mut HashMap<Key<'static>, Bucket> = &mut *BUCKETS.lock().await;

        buckets.retain(|_, bucket| {
            if bucket.reset < now {
                removes += 1;

                false
            } else {
                true
            }
        });

        start.elapsed().as_micros()
    };

    trace!("Removed {} outdated rate-limit buckets in {}µs", removes, time);
}
