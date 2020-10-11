use crate::gateway::Embed;
use crate::Snowflake;

#[object(client, default)]
pub struct CreateMessage {
    pub content: Option<String>,
    pub nonce: Option<Snowflake>,
    pub tts: Option<bool>,
    pub embed: Option<Embed>,
    pub attachment: Option<CreateAttachment>
}

#[object(client, default)]
pub struct CreateAttachment {
    pub name: String,
    pub mime: String,
    pub content: Vec<u8>,
}

#[object(client, default)]
pub struct ModifyMessage {
    pub content: Option<String>,
    pub embed: Option<Embed>,
    pub flags: u32
}
