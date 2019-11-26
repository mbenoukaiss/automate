use crate::gateway::Embed;
use crate::Snowflake;

#[object(client, default)]
pub struct CreateMessage {
    pub content: Option<String>,
    pub nonce: Option<Snowflake>,
    pub tts: Option<bool>,
    pub file: Option<String>,
    pub embed: Option<Embed>,
    pub payload_json: Option<String>
}

#[object(client, default)]
pub struct ModifyMessage {
    pub content: Option<String>,
    pub embed: Option<Embed>,
}
