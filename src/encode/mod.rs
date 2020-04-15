//! Types used by the library to transform
//! objects into data that can be sent to
//! and understood by Discord's API.

pub mod json;
mod urls;

pub use urls::ExtractSnowflake;
pub use urls::WriteUrl;