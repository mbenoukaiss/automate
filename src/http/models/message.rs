use crate::gateway::Embed;
use crate::Snowflake;

/// See [HttpApi::create_message](automate::HttpAPI::create_message)
/// for documentation.
///
/// You may create a message as a reply to another message. To do so,
/// include a [`message_reference`](#DOCS_RESOURCES_CHANNEL/message-object-message-reference-structure)
/// with a `message_id`. This requires the `VIEW MESSAGE HISTORY`
/// permission, and the referenced message must exist and cannot be
/// a system message. The `channel_id` and `guild_id` in the
/// `message_reference` are optional, but will be validated if provided.
#[object(client, default)]
pub struct CreateMessage {
    pub content: Option<String>,
    pub nonce: Option<Snowflake>,
    pub tts: bool,
    pub embed: Option<Embed>,
    pub allowed_mentions: Option<AllowedMentions>,
    pub message_reference: Option<Snowflake>,
    pub attachment: Option<CreateAttachment>,
}

#[stringify(snake_case)]
pub enum AllowedMentionType {
    /// Controls role mentions
    Roles,
    ///	Controls user mentions
    Users,
    /// Controls `@everyone` and `@here` mentions
    Everyone,
}

#[object(client, default)]
pub struct AllowedMentions {
    /// An array of mention types to parse from the content.
    pub parse: Vec<AllowedMentionType>,
    /// An array of roles that can be mentioned, maximum 100.
    pub roles: Vec<Snowflake>,
    /// An array of users that can be mentioned, maximum 100.
    pub users: Vec<Snowflake>,
    /// For replies, whether to mention the author of the
    /// message being replied to (default false)
    pub replied_user: bool,
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
