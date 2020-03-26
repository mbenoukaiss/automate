#![allow(unused_variables)]

use async_trait::async_trait;
use crate::gateway::*;
use crate::{Session, Error};
use std::pin::Pin;
use std::future::Future;

/// Parses a list of struct listeners before sending them
/// to the [Discord::with](automate::Discord::with) method.
#[macro_export]
macro_rules! structs {
    ($($listener:expr),*) => {
        vec![$(::automate::events::ListenerType::Impl(Box::new($listener))),*]
    }
}

#[async_trait]
pub trait Listener: Send + Sync + 'static {
    async fn on_ready(&mut self, session: &Session, data: &ReadyDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_channel_create(&mut self, session: &Session, data: &ChannelCreateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_channel_update(&mut self, session: &Session, data: &ChannelUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_channel_delete(&mut self, session: &Session, data: &ChannelDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_channel_pins_update(&mut self, session: &Session, data: &ChannelPinsUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_guild_create(&mut self, session: &Session, data: &GuildCreateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_guild_update(&mut self, session: &Session, data: &GuildUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_guild_delete(&mut self, session: &Session, data: &GuildDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_guild_ban_add(&mut self, session: &Session, data: &GuildBanAddDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_guild_ban_remove(&mut self, session: &Session, data: &GuildBanRemoveDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_guild_emojis_update(&mut self, session: &Session, data: &GuildEmojisUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_guild_integrations_update(&mut self, session: &Session, data: &GuildIntegrationsUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_guild_member_add(&mut self, session: &Session, data: &GuildMemberAddDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_guild_member_remove(&mut self, session: &Session, data: &GuildMemberRemoveDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_guild_member_update(&mut self, session: &Session, data: &GuildMemberUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_guild_members_chunk(&mut self, session: &Session, data: &GuildMembersChunkDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_guild_role_create(&mut self, session: &Session, data: &GuildRoleCreateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_guild_role_update(&mut self, session: &Session, data: &GuildRoleUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_guild_role_delete(&mut self, session: &Session, data: &GuildRoleDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_invite_create(&mut self, session: &Session, data: &InviteCreateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_invite_delete(&mut self, session: &Session, data: &InviteDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_message_create(&mut self, session: &Session, data: &MessageCreateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_message_update(&mut self, session: &Session, data: &MessageUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_message_delete(&mut self, session: &Session, data: &MessageDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_message_delete_bulk(&mut self, session: &Session, data: &MessageDeleteBulkDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_reaction_add(&mut self, session: &Session, data: &MessageReactionAddDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_reaction_remove(&mut self, session: &Session, data: &MessageReactionRemoveDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_reaction_remove_all(&mut self, session: &Session, data: &MessageReactionRemoveAllDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_reaction_remove_emoji(&mut self, session: &Session, data: &MessageReactionRemoveEmojiDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_presence_update(&mut self, session: &Session, data: &PresenceUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_typing_start(&mut self, session: &Session, data: &TypingStartDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_user_update(&mut self, session: &Session, data: &UserUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_voice_state_update(&mut self, session: &Session, data: &VoiceStateUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_voice_server_update(&mut self, session: &Session, data: &VoiceServerUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_webhooks_update(&mut self, session: &Session, data: &WebhooksUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

pub type Ready = for<'a> fn(&'a Session, &'a ReadyDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type ChannelCreate = for<'a> fn(&'a Session, &'a ChannelCreateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type ChannelUpdate = for<'a> fn(&'a Session, &'a ChannelUpdateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type ChannelDelete = for<'a> fn(&'a Session, &'a ChannelDeleteDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type ChannelPinsUpdate = for<'a> fn(&'a Session, &'a ChannelPinsUpdateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type GuildCreate = for<'a> fn(&'a Session, &'a GuildCreateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type GuildUpdate = for<'a> fn(&'a Session, &'a GuildUpdateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type GuildDelete = for<'a> fn(&'a Session, &'a GuildDeleteDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type GuildBanAdd = for<'a> fn(&'a Session, &'a GuildBanAddDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type GuildBanRemove = for<'a> fn(&'a Session, &'a GuildBanRemoveDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type GuildEmojisUpdate = for<'a> fn(&'a Session, &'a GuildEmojisUpdateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type GuildIntegrationsUpdate = for<'a> fn(&'a Session, &'a GuildIntegrationsUpdateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type GuildMemberAdd = for<'a> fn(&'a Session, &'a GuildMemberAddDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type GuildMemberRemove = for<'a> fn(&'a Session, &'a GuildMemberRemoveDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type GuildMemberUpdate = for<'a> fn(&'a Session, &'a GuildMemberUpdateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type GuildMembersChunk = for<'a> fn(&'a Session, &'a GuildMembersChunkDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type GuildRoleCreate = for<'a> fn(&'a Session, &'a GuildRoleCreateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type GuildRoleUpdate = for<'a> fn(&'a Session, &'a GuildRoleUpdateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type GuildRoleDelete = for<'a> fn(&'a Session, &'a GuildRoleDeleteDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type InviteCreate = for<'a> fn(&'a Session, &'a InviteCreateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type InviteDelete = for<'a> fn(&'a Session, &'a InviteDeleteDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type MessageCreate = for<'a> fn(&'a Session, &'a MessageCreateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type MessageUpdate = for<'a> fn(&'a Session, &'a MessageUpdateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type MessageDelete = for<'a> fn(&'a Session, &'a MessageDeleteDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type MessageDeleteBulk = for<'a> fn(&'a Session, &'a MessageDeleteBulkDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type MessageReactionAdd = for<'a> fn(&'a Session, &'a MessageReactionAddDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type MessageReactionRemove = for<'a> fn(&'a Session, &'a MessageReactionRemoveDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type MessageReactionRemoveAll = for<'a> fn(&'a Session, &'a MessageReactionRemoveAllDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type MessageReactionRemoveEmoji = for<'a> fn(&'a Session, &'a MessageReactionRemoveEmojiDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type PresenceUpdate = for<'a> fn(&'a Session, &'a PresenceUpdateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type TypingStart = for<'a> fn(&'a Session, &'a TypingStartDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type UserUpdate = for<'a> fn(&'a Session, &'a UserUpdateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type VoiceStateUpdate = for<'a> fn(&'a Session, &'a VoiceStateUpdateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type VoiceServerUpdate = for<'a> fn(&'a Session, &'a VoiceServerUpdateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
pub type WebhooksUpdate = for<'a> fn(&'a Session, &'a WebhooksUpdateDispatch) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;

pub enum ListenerType {
    Impl(Box<dyn Listener>),
    Ready(Ready),
    ChannelCreate(ChannelCreate),
    ChannelUpdate(ChannelUpdate),
    ChannelDelete(ChannelDelete),
    ChannelPinsUpdate(ChannelPinsUpdate),
    GuildCreate(GuildCreate),
    GuildUpdate(GuildUpdate),
    GuildDelete(GuildDelete),
    GuildBanAdd(GuildBanAdd),
    GuildBanRemove(GuildBanRemove),
    GuildEmojisUpdate(GuildEmojisUpdate),
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    GuildMemberAdd(GuildMemberAdd),
    GuildMemberRemove(GuildMemberRemove),
    GuildMemberUpdate(GuildMemberUpdate),
    GuildMembersChunk(GuildMembersChunk),
    GuildRoleCreate(GuildRoleCreate),
    GuildRoleUpdate(GuildRoleUpdate),
    GuildRoleDelete(GuildRoleDelete),
    InviteCreate(InviteCreate),
    InviteDelete(InviteDelete),
    MessageCreate(MessageCreate),
    MessageUpdate(MessageUpdate),
    MessageDelete(MessageDelete),
    MessageDeleteBulk(MessageDeleteBulk),
    MessageReactionAdd(MessageReactionAdd),
    MessageReactionRemove(MessageReactionRemove),
    MessageReactionRemoveAll(MessageReactionRemoveAll),
    MessageReactionRemoveEmoji(MessageReactionRemoveEmoji),
    PresenceUpdate(PresenceUpdate),
    TypingStart(TypingStart),
    UserUpdate(UserUpdate),
    VoiceStateUpdate(VoiceStateUpdate),
    VoiceServerUpdate(VoiceServerUpdate),
    WebhooksUpdate(WebhooksUpdate),
}
