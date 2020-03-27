#![allow(unused_variables)]
//! Defines all the types required to make listeners
//!
//! This module should not be explicitly imported by
//! projects using this library except for the
//! [Listener](automate::Listener) trait which is
//! re-exported in the crate root.

use async_trait::async_trait;
use crate::gateway::*;
use crate::{Context, Error};
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

/// A Discord event listener.
/// After an instance of a struct implementing
/// this trait is registered, methods will be called
/// when the library receives the corresponding events.
///
/// Structs implementing this listener must derive
/// [Clone](std::clone::Clone).
#[async_trait]
pub trait Listener: ListenerClone + Send + 'static {
    /// Method called when a [Ready](automate::gateway::ReadyDispatch) event is received.
    async fn on_ready(&mut self, ctx: &mut Context, data: &ReadyDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [ChannelCreate](automate::gateway::ChannelCreateDispatch) event is received.
    async fn on_channel_create(&mut self, ctx: &mut Context, data: &ChannelCreateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [ChannelUpdate](automate::gateway::ChannelUpdateDispatch) event is received.
    async fn on_channel_update(&mut self, ctx: &mut Context, data: &ChannelUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [ChannelDelete](automate::gateway::ChannelDeleteDispatch) event is received.
    async fn on_channel_delete(&mut self, ctx: &mut Context, data: &ChannelDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [ChannelPinsUpdate](automate::gateway::ChannelPinsUpdateDispatch) event is received.
    async fn on_channel_pins_update(&mut self, ctx: &mut Context, data: &ChannelPinsUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [GuildCreate](automate::gateway::GuildCreateDispatch) event is received.
    async fn on_guild_create(&mut self, ctx: &mut Context, data: &GuildCreateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [GuildUpdate](automate::gateway::GuildUpdateDispatch) event is received.
    async fn on_guild_update(&mut self, ctx: &mut Context, data: &GuildUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [GuildDelete](automate::gateway::GuildDeleteDispatch) event is received.
    async fn on_guild_delete(&mut self, ctx: &mut Context, data: &GuildDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [GuildBanAdd](automate::gateway::GuildBanAddDispatch) event is received.
    async fn on_guild_ban_add(&mut self, ctx: &mut Context, data: &GuildBanAddDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [GuildBanRemove](automate::gateway::GuildBanRemoveDispatch) event is received.
    async fn on_guild_ban_remove(&mut self, ctx: &mut Context, data: &GuildBanRemoveDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [GuildEmojisUpdate](automate::gateway::GuildEmojisUpdateDispatch) event is received.
    async fn on_guild_emojis_update(&mut self, ctx: &mut Context, data: &GuildEmojisUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [GuildIntegrationsUpdate](automate::gateway::GuildIntegrationsUpdateDispatch) event is received.
    async fn on_guild_integrations_update(&mut self, ctx: &mut Context, data: &GuildIntegrationsUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [GuildMemberAdd](automate::gateway::GuildMemberAddDispatch) event is received.
    async fn on_guild_member_add(&mut self, ctx: &mut Context, data: &GuildMemberAddDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [GuildMemberRemove](automate::gateway::GuildMemberRemoveDispatch) event is received.
    async fn on_guild_member_remove(&mut self, ctx: &mut Context, data: &GuildMemberRemoveDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [GuildMemberUpdate](automate::gateway::GuildMemberUpdateDispatch) event is received.
    async fn on_guild_member_update(&mut self, ctx: &mut Context, data: &GuildMemberUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [GuildMembersChunk](automate::gateway::GuildMembersChunkDispatch) event is received.
    async fn on_guild_members_chunk(&mut self, ctx: &mut Context, data: &GuildMembersChunkDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [GuildRoleCreate](automate::gateway::GuildRoleCreateDispatch) event is received.
    async fn on_guild_role_create(&mut self, ctx: &mut Context, data: &GuildRoleCreateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [GuildRoleUpdate](automate::gateway::GuildRoleUpdateDispatch) event is received.
    async fn on_guild_role_update(&mut self, ctx: &mut Context, data: &GuildRoleUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [GuildRoleDelete](automate::gateway::GuildRoleDeleteDispatch) event is received.
    async fn on_guild_role_delete(&mut self, ctx: &mut Context, data: &GuildRoleDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [InviteCreate](automate::gateway::InviteCreateDispatch) event is received.
    async fn on_invite_create(&mut self, ctx: &mut Context, data: &InviteCreateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [InviteDelete](automate::gateway::InviteDeleteDispatch) event is received.
    async fn on_invite_delete(&mut self, ctx: &mut Context, data: &InviteDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [MessageCreate](automate::gateway::MessageCreateDispatch) event is received.
    async fn on_message_create(&mut self, ctx: &mut Context, data: &MessageCreateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [MessageUpdate](automate::gateway::MessageUpdateDispatch) event is received.
    async fn on_message_update(&mut self, ctx: &mut Context, data: &MessageUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [MessageDelete](automate::gateway::MessageDeleteDispatch) event is received.
    async fn on_message_delete(&mut self, ctx: &mut Context, data: &MessageDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [MessageDeleteBulk](automate::gateway::MessageDeleteBulkDispatch) event is received.
    async fn on_message_delete_bulk(&mut self, ctx: &mut Context, data: &MessageDeleteBulkDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [MessageReactionAdd](automate::gateway::MessageReactionAddDispatch) event is received.
    async fn on_reaction_add(&mut self, ctx: &mut Context, data: &MessageReactionAddDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [MessageReactionRemove](automate::gateway::MessageReactionRemoveDispatch) event is received.
    async fn on_reaction_remove(&mut self, ctx: &mut Context, data: &MessageReactionRemoveDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [MessageReactionRemoveAll](automate::gateway::MessageReactionRemoveAllDispatch) event is received.
    async fn on_reaction_remove_all(&mut self, ctx: &mut Context, data: &MessageReactionRemoveAllDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [MessageReactionRemoveEmoji](automate::gateway::MessageReactionRemoveEmojiDispatch) event is received.
    async fn on_reaction_remove_emoji(&mut self, ctx: &mut Context, data: &MessageReactionRemoveEmojiDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [PresenceUpdate](automate::gateway::PresenceUpdateDispatch) event is received.
    async fn on_presence_update(&mut self, ctx: &mut Context, data: &PresenceUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [TypingStart](automate::gateway::TypingStartDispatch) event is received.
    async fn on_typing_start(&mut self, ctx: &mut Context, data: &TypingStartDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [UserUpdate](automate::gateway::UserUpdateDispatch) event is received.
    async fn on_user_update(&mut self, ctx: &mut Context, data: &UserUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [VoiceStateUpdate](automate::gateway::VoiceStateUpdateDispatch) event is received.
    async fn on_voice_state_update(&mut self, ctx: &mut Context, data: &VoiceStateUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [VoiceServerUpdate](automate::gateway::VoiceServerUpdateDispatch) event is received.
    async fn on_voice_server_update(&mut self, ctx: &mut Context, data: &VoiceServerUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    /// Method called when a [WebhooksUpdate](automate::gateway::WebhooksUpdateDispatch) event is received.
    async fn on_webhooks_update(&mut self, ctx: &mut Context, data: &WebhooksUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

/// Internal type used to allow cloning the
/// list of event listeners by implementing
/// a boxed clone for the listener trait.
#[doc(hidden)]
pub trait ListenerClone {
    fn clone_box(&self) -> Box<dyn Listener>;
}

impl<T> ListenerClone for T where T: Listener + Clone {
    fn clone_box(&self) -> Box<dyn Listener> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Listener> {
    fn clone(&self) -> Box<dyn Listener> {
        self.clone_box()
    }
}

/// Function listening to [Ready](automate::gateway::ReadyDispatch) events.
pub type Ready = for<'a> fn(&'a mut Context, &'a ReadyDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [ChannelCreate](automate::gateway::ChannelCreateDispatch) events.
pub type ChannelCreate = for<'a> fn(&'a mut Context, &'a ChannelCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [ChannelUpdate](automate::gateway::ChannelUpdateDispatch) events.
pub type ChannelUpdate = for<'a> fn(&'a mut Context, &'a ChannelUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [ChannelDelete](automate::gateway::ChannelDeleteDispatch) events.
pub type ChannelDelete = for<'a> fn(&'a mut Context, &'a ChannelDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [ChannelPinsUpdate](automate::gateway::ChannelPinsUpdateDispatch) events.
pub type ChannelPinsUpdate = for<'a> fn(&'a mut Context, &'a ChannelPinsUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [GuildCreate](automate::gateway::GuildCreateDispatch) events.
pub type GuildCreate = for<'a> fn(&'a mut Context, &'a GuildCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [GuildUpdate](automate::gateway::GuildUpdateDispatch) events.
pub type GuildUpdate = for<'a> fn(&'a mut Context, &'a GuildUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [GuildDelete](automate::gateway::GuildDeleteDispatch) events.
pub type GuildDelete = for<'a> fn(&'a mut Context, &'a GuildDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [GuildBanAdd](automate::gateway::GuildBanAddDispatch) events.
pub type GuildBanAdd = for<'a> fn(&'a mut Context, &'a GuildBanAddDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [GuildBanRemove](automate::gateway::GuildBanRemoveDispatch) events.
pub type GuildBanRemove = for<'a> fn(&'a mut Context, &'a GuildBanRemoveDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [GuildEmojisUpdate](automate::gateway::GuildEmojisUpdateDispatch) events.
pub type GuildEmojisUpdate = for<'a> fn(&'a mut Context, &'a GuildEmojisUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [GuildIntegrationsUpdate](automate::gateway::GuildIntegrationsUpdateDispatch) events.
pub type GuildIntegrationsUpdate = for<'a> fn(&'a mut Context, &'a GuildIntegrationsUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [GuildMemberAdd](automate::gateway::GuildMemberAddDispatch) events.
pub type GuildMemberAdd = for<'a> fn(&'a mut Context, &'a GuildMemberAddDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [GuildMemberRemove](automate::gateway::GuildMemberRemoveDispatch) events.
pub type GuildMemberRemove = for<'a> fn(&'a mut Context, &'a GuildMemberRemoveDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [GuildMemberUpdate](automate::gateway::GuildMemberUpdateDispatch) events.
pub type GuildMemberUpdate = for<'a> fn(&'a mut Context, &'a GuildMemberUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [GuildMembersChunk](automate::gateway::GuildMembersChunkDispatch) events.
pub type GuildMembersChunk = for<'a> fn(&'a mut Context, &'a GuildMembersChunkDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [GuildRoleCreate](automate::gateway::GuildRoleCreateDispatch) events.
pub type GuildRoleCreate = for<'a> fn(&'a mut Context, &'a GuildRoleCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [GuildRoleUpdate](automate::gateway::GuildRoleUpdateDispatch) events.
pub type GuildRoleUpdate = for<'a> fn(&'a mut Context, &'a GuildRoleUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [GuildRoleDelete](automate::gateway::GuildRoleDeleteDispatch) events.
pub type GuildRoleDelete = for<'a> fn(&'a mut Context, &'a GuildRoleDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [InviteCreate](automate::gateway::InviteCreateDispatch) events.
pub type InviteCreate = for<'a> fn(&'a mut Context, &'a InviteCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [InviteDelete](automate::gateway::InviteDeleteDispatch) events.
pub type InviteDelete = for<'a> fn(&'a mut Context, &'a InviteDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [MessageCreate](automate::gateway::MessageCreateDispatch) events.
pub type MessageCreate = for<'a> fn(&'a mut Context, &'a MessageCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [MessageUpdate](automate::gateway::MessageUpdateDispatch) events.
pub type MessageUpdate = for<'a> fn(&'a mut Context, &'a MessageUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [MessageDelete](automate::gateway::MessageDeleteDispatch) events.
pub type MessageDelete = for<'a> fn(&'a mut Context, &'a MessageDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [MessageDeleteBulk](automate::gateway::MessageDeleteBulkDispatch) events.
pub type MessageDeleteBulk = for<'a> fn(&'a mut Context, &'a MessageDeleteBulkDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [MessageReactionAdd](automate::gateway::MessageReactionAddDispatch) events.
pub type MessageReactionAdd = for<'a> fn(&'a mut Context, &'a MessageReactionAddDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [MessageReactionRemove](automate::gateway::MessageReactionRemoveDispatch) events.
pub type MessageReactionRemove = for<'a> fn(&'a mut Context, &'a MessageReactionRemoveDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [MessageReactionRemoveAll](automate::gateway::MessageReactionRemoveAllDispatch) events.
pub type MessageReactionRemoveAll = for<'a> fn(&'a mut Context, &'a MessageReactionRemoveAllDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [MessageReactionRemoveEmoji](automate::gateway::MessageReactionRemoveEmojiDispatch) events.
pub type MessageReactionRemoveEmoji = for<'a> fn(&'a mut Context, &'a MessageReactionRemoveEmojiDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [PresenceUpdate](automate::gateway::PresenceUpdateDispatch) events.
pub type PresenceUpdate = for<'a> fn(&'a mut Context, &'a PresenceUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [TypingStart](automate::gateway::TypingStartDispatch) events.
pub type TypingStart = for<'a> fn(&'a mut Context, &'a TypingStartDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [UserUpdate](automate::gateway::UserUpdateDispatch) events.
pub type UserUpdate = for<'a> fn(&'a mut Context, &'a UserUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [VoiceStateUpdate](automate::gateway::VoiceStateUpdateDispatch) events.
pub type VoiceStateUpdate = for<'a> fn(&'a mut Context, &'a VoiceStateUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [VoiceServerUpdate](automate::gateway::VoiceServerUpdateDispatch) events.
pub type VoiceServerUpdate = for<'a> fn(&'a mut Context, &'a VoiceServerUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Function listening to [WebhooksUpdate](automate::gateway::WebhooksUpdateDispatch) events.
pub type WebhooksUpdate = for<'a> fn(&'a mut Context, &'a WebhooksUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

/// Internal enum used to pass listeners
/// to the library.
#[doc(hidden)]
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
