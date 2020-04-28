#![allow(unused_variables)]
#![allow(deprecated)]
//! Defines all the types and macros required to make
//! and register listeners.

use async_trait::async_trait;
use crate::gateway::*;
use crate::{Context, Error};
use std::pin::Pin;
use std::future::Future;

/// Parses a list of struct listeners before sending them to the
/// [Configuration::register](automate::Configuration::register) method.
#[deprecated(since = "0.3.1", note = "Use stateful listeners instead (see the doc)")]
#[macro_export]
macro_rules! structs {
    ($($listener:expr),*) => {
        vec![$(::automate::events::ListenerType::Impl(Box::new($listener))),*]
    }
}

/// Parses a list of state structs before sending them to the
/// [Configuration::register](automate::Configuration::register) method.
#[macro_export]
macro_rules! stateful {
    ($($listener:expr),*) => {
        vec![$(::automate::events::ListenerType::Stateful(Box::new($listener))),*]
    }
}

/// Provides the initialize method for the state
/// structs which define the stateful listener functions.
pub trait Initializable {
    /// Returns the list of listeners. Gets called once
    /// when the library is setting up listeners.
    fn initialize() -> Vec<StatefulListener<Self>> where Self: Sized;
}

/// A stateful event listener.
/// After an instance of a struct implementing
/// this trait is registered, methods will be called
/// when the library receives the corresponding events.
///
/// Please do not implement this manually as the structure
/// is subject to changes until version 0.4.
///
/// Structs implementing this listener must derive [Clone](std::clone::Clone) and
/// [Initializable](automate::events::Initializable) in order to be registered.
#[async_trait]
pub trait State: StateClone + Send + 'static {
    async fn on_ready(&mut self, ctx: &Context<'_>, event: &ReadyDispatch) -> Result<(), Error>;
    async fn on_channel_create(&mut self, ctx: &Context<'_>, event: &ChannelCreateDispatch) -> Result<(), Error>;
    async fn on_channel_update(&mut self, ctx: &Context<'_>, event: &ChannelUpdateDispatch) -> Result<(), Error>;
    async fn on_channel_delete(&mut self, ctx: &Context<'_>, event: &ChannelDeleteDispatch) -> Result<(), Error>;
    async fn on_channel_pins_update(&mut self, ctx: &Context<'_>, event: &ChannelPinsUpdateDispatch) -> Result<(), Error>;
    async fn on_guild_create(&mut self, ctx: &Context<'_>, event: &GuildCreateDispatch) -> Result<(), Error>;
    async fn on_guild_update(&mut self, ctx: &Context<'_>, event: &GuildUpdateDispatch) -> Result<(), Error>;
    async fn on_guild_delete(&mut self, ctx: &Context<'_>, event: &GuildDeleteDispatch) -> Result<(), Error>;
    async fn on_guild_ban_add(&mut self, ctx: &Context<'_>, event: &GuildBanAddDispatch) -> Result<(), Error>;
    async fn on_guild_ban_remove(&mut self, ctx: &Context<'_>, event: &GuildBanRemoveDispatch) -> Result<(), Error>;
    async fn on_guild_emojis_update(&mut self, ctx: &Context<'_>, event: &GuildEmojisUpdateDispatch) -> Result<(), Error>;
    async fn on_guild_integrations_update(&mut self, ctx: &Context<'_>, event: &GuildIntegrationsUpdateDispatch) -> Result<(), Error>;
    async fn on_guild_member_add(&mut self, ctx: &Context<'_>, event: &GuildMemberAddDispatch) -> Result<(), Error>;
    async fn on_guild_member_remove(&mut self, ctx: &Context<'_>, event: &GuildMemberRemoveDispatch) -> Result<(), Error>;
    async fn on_guild_member_update(&mut self, ctx: &Context<'_>, event: &GuildMemberUpdateDispatch) -> Result<(), Error>;
    async fn on_guild_members_chunk(&mut self, ctx: &Context<'_>, event: &GuildMembersChunkDispatch) -> Result<(), Error>;
    async fn on_guild_role_create(&mut self, ctx: &Context<'_>, event: &GuildRoleCreateDispatch) -> Result<(), Error>;
    async fn on_guild_role_update(&mut self, ctx: &Context<'_>, event: &GuildRoleUpdateDispatch) -> Result<(), Error>;
    async fn on_guild_role_delete(&mut self, ctx: &Context<'_>, event: &GuildRoleDeleteDispatch) -> Result<(), Error>;
    async fn on_invite_create(&mut self, ctx: &Context<'_>, event: &InviteCreateDispatch) -> Result<(), Error>;
    async fn on_invite_delete(&mut self, ctx: &Context<'_>, event: &InviteDeleteDispatch) -> Result<(), Error>;
    async fn on_message_create(&mut self, ctx: &Context<'_>, event: &MessageCreateDispatch) -> Result<(), Error>;
    async fn on_message_update(&mut self, ctx: &Context<'_>, event: &MessageUpdateDispatch) -> Result<(), Error>;
    async fn on_message_delete(&mut self, ctx: &Context<'_>, event: &MessageDeleteDispatch) -> Result<(), Error>;
    async fn on_message_delete_bulk(&mut self, ctx: &Context<'_>, event: &MessageDeleteBulkDispatch) -> Result<(), Error>;
    async fn on_reaction_add(&mut self, ctx: &Context<'_>, event: &MessageReactionAddDispatch) -> Result<(), Error>;
    async fn on_reaction_remove(&mut self, ctx: &Context<'_>, event: &MessageReactionRemoveDispatch) -> Result<(), Error>;
    async fn on_reaction_remove_all(&mut self, ctx: &Context<'_>, event: &MessageReactionRemoveAllDispatch) -> Result<(), Error>;
    async fn on_reaction_remove_emoji(&mut self, ctx: &Context<'_>, event: &MessageReactionRemoveEmojiDispatch) -> Result<(), Error>;
    async fn on_presence_update(&mut self, ctx: &Context<'_>, event: &PresenceUpdateDispatch) -> Result<(), Error>;
    async fn on_typing_start(&mut self, ctx: &Context<'_>, event: &TypingStartDispatch) -> Result<(), Error>;
    async fn on_user_update(&mut self, ctx: &Context<'_>, event: &UserUpdateDispatch) -> Result<(), Error>;
    async fn on_voice_state_update(&mut self, ctx: &Context<'_>, event: &VoiceStateUpdateDispatch) -> Result<(), Error>;
    async fn on_voice_server_update(&mut self, ctx: &Context<'_>, event: &VoiceServerUpdateDispatch) -> Result<(), Error>;
    async fn on_webhooks_update(&mut self, ctx: &Context<'_>, event: &WebhooksUpdateDispatch) -> Result<(), Error>;
}

/// Internal type used to allow cloning the
/// list of event listeners by implementing
/// a boxed clone for the listener trait.
#[doc(hidden)]
pub trait StateClone {
    fn clone_box(&self) -> Box<dyn State>;
}

impl<T> StateClone for T where T: State + Clone {
    fn clone_box(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn State> {
    fn clone(&self) -> Box<dyn State> {
        self.clone_box()
    }
}

pub type Ready = for<'a> fn(&'a Context<'_>, &'a ReadyDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type ChannelCreate = for<'a> fn(&'a Context<'_>, &'a ChannelCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type ChannelUpdate = for<'a> fn(&'a Context<'_>, &'a ChannelUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type ChannelDelete = for<'a> fn(&'a Context<'_>, &'a ChannelDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type ChannelPinsUpdate = for<'a> fn(&'a Context<'_>, &'a ChannelPinsUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildCreate = for<'a> fn(&'a Context<'_>, &'a GuildCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildUpdate = for<'a> fn(&'a Context<'_>, &'a GuildUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildDelete = for<'a> fn(&'a Context<'_>, &'a GuildDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildBanAdd = for<'a> fn(&'a Context<'_>, &'a GuildBanAddDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildBanRemove = for<'a> fn(&'a Context<'_>, &'a GuildBanRemoveDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildEmojisUpdate = for<'a> fn(&'a Context<'_>, &'a GuildEmojisUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildIntegrationsUpdate = for<'a> fn(&'a Context<'_>, &'a GuildIntegrationsUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildMemberAdd = for<'a> fn(&'a Context<'_>, &'a GuildMemberAddDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildMemberRemove = for<'a> fn(&'a Context<'_>, &'a GuildMemberRemoveDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildMemberUpdate = for<'a> fn(&'a Context<'_>, &'a GuildMemberUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildMembersChunk = for<'a> fn(&'a Context<'_>, &'a GuildMembersChunkDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildRoleCreate = for<'a> fn(&'a Context<'_>, &'a GuildRoleCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildRoleUpdate = for<'a> fn(&'a Context<'_>, &'a GuildRoleUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildRoleDelete = for<'a> fn(&'a Context<'_>, &'a GuildRoleDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type InviteCreate = for<'a> fn(&'a Context<'_>, &'a InviteCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type InviteDelete = for<'a> fn(&'a Context<'_>, &'a InviteDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageCreate = for<'a> fn(&'a Context<'_>, &'a MessageCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageUpdate = for<'a> fn(&'a Context<'_>, &'a MessageUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageDelete = for<'a> fn(&'a Context<'_>, &'a MessageDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageDeleteBulk = for<'a> fn(&'a Context<'_>, &'a MessageDeleteBulkDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageReactionAdd = for<'a> fn(&'a Context<'_>, &'a MessageReactionAddDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageReactionRemove = for<'a> fn(&'a Context<'_>, &'a MessageReactionRemoveDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageReactionRemoveAll = for<'a> fn(&'a Context<'_>, &'a MessageReactionRemoveAllDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageReactionRemoveEmoji = for<'a> fn(&'a Context<'_>, &'a MessageReactionRemoveEmojiDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type PresenceUpdate = for<'a> fn(&'a Context<'_>, &'a PresenceUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type TypingStart = for<'a> fn(&'a Context<'_>, &'a TypingStartDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type UserUpdate = for<'a> fn(&'a Context<'_>, &'a UserUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type VoiceStateUpdate = for<'a> fn(&'a Context<'_>, &'a VoiceStateUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type VoiceServerUpdate = for<'a> fn(&'a Context<'_>, &'a VoiceServerUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type WebhooksUpdate = for<'a> fn(&'a Context<'_>, &'a WebhooksUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

pub type ReadySelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a ReadyDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type ChannelCreateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a ChannelCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type ChannelUpdateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a ChannelUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type ChannelDeleteSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a ChannelDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type ChannelPinsUpdateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a ChannelPinsUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildCreateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a GuildCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildUpdateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a GuildUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildDeleteSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a GuildDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildBanAddSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a GuildBanAddDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildBanRemoveSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a GuildBanRemoveDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildEmojisUpdateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a GuildEmojisUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildIntegrationsUpdateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a GuildIntegrationsUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildMemberAddSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a GuildMemberAddDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildMemberRemoveSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a GuildMemberRemoveDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildMemberUpdateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a GuildMemberUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildMembersChunkSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a GuildMembersChunkDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildRoleCreateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a GuildRoleCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildRoleUpdateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a GuildRoleUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildRoleDeleteSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a GuildRoleDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type InviteCreateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a InviteCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type InviteDeleteSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a InviteDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageCreateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a MessageCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageUpdateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a MessageUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageDeleteSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a MessageDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageDeleteBulkSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a MessageDeleteBulkDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageReactionAddSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a MessageReactionAddDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageReactionRemoveSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a MessageReactionRemoveDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageReactionRemoveAllSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a MessageReactionRemoveAllDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageReactionRemoveEmojiSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a MessageReactionRemoveEmojiDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type PresenceUpdateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a PresenceUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type TypingStartSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a TypingStartDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type UserUpdateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a UserUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type VoiceStateUpdateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a VoiceStateUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type VoiceServerUpdateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a VoiceServerUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type WebhooksUpdateSelf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a WebhooksUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

pub type ReadySelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a ReadyDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type ChannelCreateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a ChannelCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type ChannelUpdateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a ChannelUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type ChannelDeleteSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a ChannelDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type ChannelPinsUpdateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a ChannelPinsUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildCreateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a GuildCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildUpdateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a GuildUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildDeleteSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a GuildDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildBanAddSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a GuildBanAddDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildBanRemoveSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a GuildBanRemoveDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildEmojisUpdateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a GuildEmojisUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildIntegrationsUpdateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a GuildIntegrationsUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildMemberAddSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a GuildMemberAddDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildMemberRemoveSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a GuildMemberRemoveDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildMemberUpdateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a GuildMemberUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildMembersChunkSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a GuildMembersChunkDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildRoleCreateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a GuildRoleCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildRoleUpdateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a GuildRoleUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type GuildRoleDeleteSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a GuildRoleDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type InviteCreateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a InviteCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type InviteDeleteSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a InviteDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageCreateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a MessageCreateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageUpdateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a MessageUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageDeleteSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a MessageDeleteDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageDeleteBulkSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a MessageDeleteBulkDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageReactionAddSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a MessageReactionAddDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageReactionRemoveSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a MessageReactionRemoveDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageReactionRemoveAllSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a MessageReactionRemoveAllDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type MessageReactionRemoveEmojiSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a MessageReactionRemoveEmojiDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type PresenceUpdateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a PresenceUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type TypingStartSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a TypingStartDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type UserUpdateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a UserUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type VoiceStateUpdateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a VoiceStateUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type VoiceServerUpdateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a VoiceServerUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
pub type WebhooksUpdateSelfMut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a WebhooksUpdateDispatch) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;

#[doc(hidden)]
pub enum ListenerType {
    Stateful(Box<dyn State>),
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

#[derive(Default, Clone)]
pub(crate) struct ListenerContainer {
    pub(crate) stateful_listeners: Vec<Box<dyn State>>,
    pub(crate) ready: Vec<Ready>,
    pub(crate) channel_create: Vec<ChannelCreate>,
    pub(crate) channel_update: Vec<ChannelUpdate>,
    pub(crate) channel_delete: Vec<ChannelDelete>,
    pub(crate) channel_pins_update: Vec<ChannelPinsUpdate>,
    pub(crate) guild_create: Vec<GuildCreate>,
    pub(crate) guild_update: Vec<GuildUpdate>,
    pub(crate) guild_delete: Vec<GuildDelete>,
    pub(crate) guild_ban_add: Vec<GuildBanAdd>,
    pub(crate) guild_ban_remove: Vec<GuildBanRemove>,
    pub(crate) guild_emojis_update: Vec<GuildEmojisUpdate>,
    pub(crate) guild_integrations_update: Vec<GuildIntegrationsUpdate>,
    pub(crate) guild_member_add: Vec<GuildMemberAdd>,
    pub(crate) guild_member_remove: Vec<GuildMemberRemove>,
    pub(crate) guild_member_update: Vec<GuildMemberUpdate>,
    pub(crate) guild_members_chunk: Vec<GuildMembersChunk>,
    pub(crate) guild_role_create: Vec<GuildRoleCreate>,
    pub(crate) guild_role_update: Vec<GuildRoleUpdate>,
    pub(crate) guild_role_delete: Vec<GuildRoleDelete>,
    pub(crate) invite_create: Vec<InviteCreate>,
    pub(crate) invite_delete: Vec<InviteDelete>,
    pub(crate) message_create: Vec<MessageCreate>,
    pub(crate) message_update: Vec<MessageUpdate>,
    pub(crate) message_delete: Vec<MessageDelete>,
    pub(crate) message_delete_bulk: Vec<MessageDeleteBulk>,
    pub(crate) reaction_add: Vec<MessageReactionAdd>,
    pub(crate) reaction_remove: Vec<MessageReactionRemove>,
    pub(crate) reaction_remove_all: Vec<MessageReactionRemoveAll>,
    pub(crate) reaction_remove_emoji: Vec<MessageReactionRemoveEmoji>,
    pub(crate) presence_update: Vec<PresenceUpdate>,
    pub(crate) typing_start: Vec<TypingStart>,
    pub(crate) user_update: Vec<UserUpdate>,
    pub(crate) voice_state_update: Vec<VoiceStateUpdate>,
    pub(crate) voice_server_update: Vec<VoiceServerUpdate>,
    pub(crate) webhooks_update: Vec<WebhooksUpdate>,
}

impl ListenerContainer {
    pub(crate) fn register(&mut self, listeners: Vec<ListenerType>) {
        for l in listeners {
            match l {
                ListenerType::Stateful(l) => self.stateful_listeners.push(l),
                ListenerType::Ready(l) => self.ready.push(l),
                ListenerType::ChannelCreate(l) => self.channel_create.push(l),
                ListenerType::ChannelUpdate(l) => self.channel_update.push(l),
                ListenerType::ChannelDelete(l) => self.channel_delete.push(l),
                ListenerType::ChannelPinsUpdate(l) => self.channel_pins_update.push(l),
                ListenerType::GuildCreate(l) => self.guild_create.push(l),
                ListenerType::GuildUpdate(l) => self.guild_update.push(l),
                ListenerType::GuildDelete(l) => self.guild_delete.push(l),
                ListenerType::GuildBanAdd(l) => self.guild_ban_add.push(l),
                ListenerType::GuildBanRemove(l) => self.guild_ban_remove.push(l),
                ListenerType::GuildEmojisUpdate(l) => self.guild_emojis_update.push(l),
                ListenerType::GuildIntegrationsUpdate(l) => self.guild_integrations_update.push(l),
                ListenerType::GuildMemberAdd(l) => self.guild_member_add.push(l),
                ListenerType::GuildMemberRemove(l) => self.guild_member_remove.push(l),
                ListenerType::GuildMemberUpdate(l) => self.guild_member_update.push(l),
                ListenerType::GuildMembersChunk(l) => self.guild_members_chunk.push(l),
                ListenerType::GuildRoleCreate(l) => self.guild_role_create.push(l),
                ListenerType::GuildRoleUpdate(l) => self.guild_role_update.push(l),
                ListenerType::GuildRoleDelete(l) => self.guild_role_delete.push(l),
                ListenerType::InviteCreate(l) => self.invite_create.push(l),
                ListenerType::InviteDelete(l) => self.invite_delete.push(l),
                ListenerType::MessageCreate(l) => self.message_create.push(l),
                ListenerType::MessageUpdate(l) => self.message_update.push(l),
                ListenerType::MessageDelete(l) => self.message_delete.push(l),
                ListenerType::MessageDeleteBulk(l) => self.message_delete_bulk.push(l),
                ListenerType::MessageReactionAdd(l) => self.reaction_add.push(l),
                ListenerType::MessageReactionRemove(l) => self.reaction_remove.push(l),
                ListenerType::MessageReactionRemoveAll(l) => self.reaction_remove_all.push(l),
                ListenerType::MessageReactionRemoveEmoji(l) => self.reaction_remove_emoji.push(l),
                ListenerType::PresenceUpdate(l) => self.presence_update.push(l),
                ListenerType::TypingStart(l) => self.typing_start.push(l),
                ListenerType::UserUpdate(l) => self.user_update.push(l),
                ListenerType::VoiceStateUpdate(l) => self.voice_state_update.push(l),
                ListenerType::VoiceServerUpdate(l) => self.voice_server_update.push(l),
                ListenerType::WebhooksUpdate(l) => self.webhooks_update.push(l),
            }
        }
    }
}

#[doc(hidden)]
pub enum StatefulListener<T> {
    Ready(ReadySelf<T>),
    ChannelCreate(ChannelCreateSelf<T>),
    ChannelUpdate(ChannelUpdateSelf<T>),
    ChannelDelete(ChannelDeleteSelf<T>),
    ChannelPinsUpdate(ChannelPinsUpdateSelf<T>),
    GuildCreate(GuildCreateSelf<T>),
    GuildUpdate(GuildUpdateSelf<T>),
    GuildDelete(GuildDeleteSelf<T>),
    GuildBanAdd(GuildBanAddSelf<T>),
    GuildBanRemove(GuildBanRemoveSelf<T>),
    GuildEmojisUpdate(GuildEmojisUpdateSelf<T>),
    GuildIntegrationsUpdate(GuildIntegrationsUpdateSelf<T>),
    GuildMemberAdd(GuildMemberAddSelf<T>),
    GuildMemberRemove(GuildMemberRemoveSelf<T>),
    GuildMemberUpdate(GuildMemberUpdateSelf<T>),
    GuildMembersChunk(GuildMembersChunkSelf<T>),
    GuildRoleCreate(GuildRoleCreateSelf<T>),
    GuildRoleUpdate(GuildRoleUpdateSelf<T>),
    GuildRoleDelete(GuildRoleDeleteSelf<T>),
    InviteCreate(InviteCreateSelf<T>),
    InviteDelete(InviteDeleteSelf<T>),
    MessageCreate(MessageCreateSelf<T>),
    MessageUpdate(MessageUpdateSelf<T>),
    MessageDelete(MessageDeleteSelf<T>),
    MessageDeleteBulk(MessageDeleteBulkSelf<T>),
    MessageReactionAdd(MessageReactionAddSelf<T>),
    MessageReactionRemove(MessageReactionRemoveSelf<T>),
    MessageReactionRemoveAll(MessageReactionRemoveAllSelf<T>),
    MessageReactionRemoveEmoji(MessageReactionRemoveEmojiSelf<T>),
    PresenceUpdate(PresenceUpdateSelf<T>),
    TypingStart(TypingStartSelf<T>),
    UserUpdate(UserUpdateSelf<T>),
    VoiceStateUpdate(VoiceStateUpdateSelf<T>),
    VoiceServerUpdate(VoiceServerUpdateSelf<T>),
    WebhooksUpdate(WebhooksUpdateSelf<T>),

    ReadyMut(ReadySelfMut<T>),
    ChannelCreateMut(ChannelCreateSelfMut<T>),
    ChannelUpdateMut(ChannelUpdateSelfMut<T>),
    ChannelDeleteMut(ChannelDeleteSelfMut<T>),
    ChannelPinsUpdateMut(ChannelPinsUpdateSelfMut<T>),
    GuildCreateMut(GuildCreateSelfMut<T>),
    GuildUpdateMut(GuildUpdateSelfMut<T>),
    GuildDeleteMut(GuildDeleteSelfMut<T>),
    GuildBanAddMut(GuildBanAddSelfMut<T>),
    GuildBanRemoveMut(GuildBanRemoveSelfMut<T>),
    GuildEmojisUpdateMut(GuildEmojisUpdateSelfMut<T>),
    GuildIntegrationsUpdateMut(GuildIntegrationsUpdateSelfMut<T>),
    GuildMemberAddMut(GuildMemberAddSelfMut<T>),
    GuildMemberRemoveMut(GuildMemberRemoveSelfMut<T>),
    GuildMemberUpdateMut(GuildMemberUpdateSelfMut<T>),
    GuildMembersChunkMut(GuildMembersChunkSelfMut<T>),
    GuildRoleCreateMut(GuildRoleCreateSelfMut<T>),
    GuildRoleUpdateMut(GuildRoleUpdateSelfMut<T>),
    GuildRoleDeleteMut(GuildRoleDeleteSelfMut<T>),
    InviteCreateMut(InviteCreateSelfMut<T>),
    InviteDeleteMut(InviteDeleteSelfMut<T>),
    MessageCreateMut(MessageCreateSelfMut<T>),
    MessageUpdateMut(MessageUpdateSelfMut<T>),
    MessageDeleteMut(MessageDeleteSelfMut<T>),
    MessageDeleteBulkMut(MessageDeleteBulkSelfMut<T>),
    MessageReactionAddMut(MessageReactionAddSelfMut<T>),
    MessageReactionRemoveMut(MessageReactionRemoveSelfMut<T>),
    MessageReactionRemoveAllMut(MessageReactionRemoveAllSelfMut<T>),
    MessageReactionRemoveEmojiMut(MessageReactionRemoveEmojiSelfMut<T>),
    PresenceUpdateMut(PresenceUpdateSelfMut<T>),
    TypingStartMut(TypingStartSelfMut<T>),
    UserUpdateMut(UserUpdateSelfMut<T>),
    VoiceStateUpdateMut(VoiceStateUpdateSelfMut<T>),
    VoiceServerUpdateMut(VoiceServerUpdateSelfMut<T>),
    WebhooksUpdateMut(WebhooksUpdateSelfMut<T>),
}

#[derive(Default, Clone)]
pub struct StatefulListenerContainer<T> {
    pub ready: Vec<ReadySelf<T>>,
    pub channel_create: Vec<ChannelCreateSelf<T>>,
    pub channel_update: Vec<ChannelUpdateSelf<T>>,
    pub channel_delete: Vec<ChannelDeleteSelf<T>>,
    pub channel_pins_update: Vec<ChannelPinsUpdateSelf<T>>,
    pub guild_create: Vec<GuildCreateSelf<T>>,
    pub guild_update: Vec<GuildUpdateSelf<T>>,
    pub guild_delete: Vec<GuildDeleteSelf<T>>,
    pub guild_ban_add: Vec<GuildBanAddSelf<T>>,
    pub guild_ban_remove: Vec<GuildBanRemoveSelf<T>>,
    pub guild_emojis_update: Vec<GuildEmojisUpdateSelf<T>>,
    pub guild_integrations_update: Vec<GuildIntegrationsUpdateSelf<T>>,
    pub guild_member_add: Vec<GuildMemberAddSelf<T>>,
    pub guild_member_remove: Vec<GuildMemberRemoveSelf<T>>,
    pub guild_member_update: Vec<GuildMemberUpdateSelf<T>>,
    pub guild_members_chunk: Vec<GuildMembersChunkSelf<T>>,
    pub guild_role_create: Vec<GuildRoleCreateSelf<T>>,
    pub guild_role_update: Vec<GuildRoleUpdateSelf<T>>,
    pub guild_role_delete: Vec<GuildRoleDeleteSelf<T>>,
    pub invite_create: Vec<InviteCreateSelf<T>>,
    pub invite_delete: Vec<InviteDeleteSelf<T>>,
    pub message_create: Vec<MessageCreateSelf<T>>,
    pub message_update: Vec<MessageUpdateSelf<T>>,
    pub message_delete: Vec<MessageDeleteSelf<T>>,
    pub message_delete_bulk: Vec<MessageDeleteBulkSelf<T>>,
    pub reaction_add: Vec<MessageReactionAddSelf<T>>,
    pub reaction_remove: Vec<MessageReactionRemoveSelf<T>>,
    pub reaction_remove_all: Vec<MessageReactionRemoveAllSelf<T>>,
    pub reaction_remove_emoji: Vec<MessageReactionRemoveEmojiSelf<T>>,
    pub presence_update: Vec<PresenceUpdateSelf<T>>,
    pub typing_start: Vec<TypingStartSelf<T>>,
    pub user_update: Vec<UserUpdateSelf<T>>,
    pub voice_state_update: Vec<VoiceStateUpdateSelf<T>>,
    pub voice_server_update: Vec<VoiceServerUpdateSelf<T>>,
    pub webhooks_update: Vec<WebhooksUpdateSelf<T>>,

    pub ready_mut: Vec<ReadySelfMut<T>>,
    pub channel_create_mut: Vec<ChannelCreateSelfMut<T>>,
    pub channel_update_mut: Vec<ChannelUpdateSelfMut<T>>,
    pub channel_delete_mut: Vec<ChannelDeleteSelfMut<T>>,
    pub channel_pins_update_mut: Vec<ChannelPinsUpdateSelfMut<T>>,
    pub guild_create_mut: Vec<GuildCreateSelfMut<T>>,
    pub guild_update_mut: Vec<GuildUpdateSelfMut<T>>,
    pub guild_delete_mut: Vec<GuildDeleteSelfMut<T>>,
    pub guild_ban_add_mut: Vec<GuildBanAddSelfMut<T>>,
    pub guild_ban_remove_mut: Vec<GuildBanRemoveSelfMut<T>>,
    pub guild_emojis_update_mut: Vec<GuildEmojisUpdateSelfMut<T>>,
    pub guild_integrations_update_mut: Vec<GuildIntegrationsUpdateSelfMut<T>>,
    pub guild_member_add_mut: Vec<GuildMemberAddSelfMut<T>>,
    pub guild_member_remove_mut: Vec<GuildMemberRemoveSelfMut<T>>,
    pub guild_member_update_mut: Vec<GuildMemberUpdateSelfMut<T>>,
    pub guild_members_chunk_mut: Vec<GuildMembersChunkSelfMut<T>>,
    pub guild_role_create_mut: Vec<GuildRoleCreateSelfMut<T>>,
    pub guild_role_update_mut: Vec<GuildRoleUpdateSelfMut<T>>,
    pub guild_role_delete_mut: Vec<GuildRoleDeleteSelfMut<T>>,
    pub invite_create_mut: Vec<InviteCreateSelfMut<T>>,
    pub invite_delete_mut: Vec<InviteDeleteSelfMut<T>>,
    pub message_create_mut: Vec<MessageCreateSelfMut<T>>,
    pub message_update_mut: Vec<MessageUpdateSelfMut<T>>,
    pub message_delete_mut: Vec<MessageDeleteSelfMut<T>>,
    pub message_delete_bulk_mut: Vec<MessageDeleteBulkSelfMut<T>>,
    pub reaction_add_mut: Vec<MessageReactionAddSelfMut<T>>,
    pub reaction_remove_mut: Vec<MessageReactionRemoveSelfMut<T>>,
    pub reaction_remove_all_mut: Vec<MessageReactionRemoveAllSelfMut<T>>,
    pub reaction_remove_emoji_mut: Vec<MessageReactionRemoveEmojiSelfMut<T>>,
    pub presence_update_mut: Vec<PresenceUpdateSelfMut<T>>,
    pub typing_start_mut: Vec<TypingStartSelfMut<T>>,
    pub user_update_mut: Vec<UserUpdateSelfMut<T>>,
    pub voice_state_update_mut: Vec<VoiceStateUpdateSelfMut<T>>,
    pub voice_server_update_mut: Vec<VoiceServerUpdateSelfMut<T>>,
    pub webhooks_update_mut: Vec<WebhooksUpdateSelfMut<T>>,
}

impl<T> StatefulListenerContainer<T> {
    pub fn register(&mut self, listeners: Vec<StatefulListener<T>>) {
        for l in listeners {
            match l {
                StatefulListener::Ready(l) => self.ready.push(l),
                StatefulListener::ChannelCreate(l) => self.channel_create.push(l),
                StatefulListener::ChannelUpdate(l) => self.channel_update.push(l),
                StatefulListener::ChannelDelete(l) => self.channel_delete.push(l),
                StatefulListener::ChannelPinsUpdate(l) => self.channel_pins_update.push(l),
                StatefulListener::GuildCreate(l) => self.guild_create.push(l),
                StatefulListener::GuildUpdate(l) => self.guild_update.push(l),
                StatefulListener::GuildDelete(l) => self.guild_delete.push(l),
                StatefulListener::GuildBanAdd(l) => self.guild_ban_add.push(l),
                StatefulListener::GuildBanRemove(l) => self.guild_ban_remove.push(l),
                StatefulListener::GuildEmojisUpdate(l) => self.guild_emojis_update.push(l),
                StatefulListener::GuildIntegrationsUpdate(l) => self.guild_integrations_update.push(l),
                StatefulListener::GuildMemberAdd(l) => self.guild_member_add.push(l),
                StatefulListener::GuildMemberRemove(l) => self.guild_member_remove.push(l),
                StatefulListener::GuildMemberUpdate(l) => self.guild_member_update.push(l),
                StatefulListener::GuildMembersChunk(l) => self.guild_members_chunk.push(l),
                StatefulListener::GuildRoleCreate(l) => self.guild_role_create.push(l),
                StatefulListener::GuildRoleUpdate(l) => self.guild_role_update.push(l),
                StatefulListener::GuildRoleDelete(l) => self.guild_role_delete.push(l),
                StatefulListener::InviteCreate(l) => self.invite_create.push(l),
                StatefulListener::InviteDelete(l) => self.invite_delete.push(l),
                StatefulListener::MessageCreate(l) => self.message_create.push(l),
                StatefulListener::MessageUpdate(l) => self.message_update.push(l),
                StatefulListener::MessageDelete(l) => self.message_delete.push(l),
                StatefulListener::MessageDeleteBulk(l) => self.message_delete_bulk.push(l),
                StatefulListener::MessageReactionAdd(l) => self.reaction_add.push(l),
                StatefulListener::MessageReactionRemove(l) => self.reaction_remove.push(l),
                StatefulListener::MessageReactionRemoveAll(l) => self.reaction_remove_all.push(l),
                StatefulListener::MessageReactionRemoveEmoji(l) => self.reaction_remove_emoji.push(l),
                StatefulListener::PresenceUpdate(l) => self.presence_update.push(l),
                StatefulListener::TypingStart(l) => self.typing_start.push(l),
                StatefulListener::UserUpdate(l) => self.user_update.push(l),
                StatefulListener::VoiceStateUpdate(l) => self.voice_state_update.push(l),
                StatefulListener::VoiceServerUpdate(l) => self.voice_server_update.push(l),
                StatefulListener::WebhooksUpdate(l) => self.webhooks_update.push(l),

                StatefulListener::ReadyMut(l) => self.ready_mut.push(l),
                StatefulListener::ChannelCreateMut(l) => self.channel_create_mut.push(l),
                StatefulListener::ChannelUpdateMut(l) => self.channel_update_mut.push(l),
                StatefulListener::ChannelDeleteMut(l) => self.channel_delete_mut.push(l),
                StatefulListener::ChannelPinsUpdateMut(l) => self.channel_pins_update_mut.push(l),
                StatefulListener::GuildCreateMut(l) => self.guild_create_mut.push(l),
                StatefulListener::GuildUpdateMut(l) => self.guild_update_mut.push(l),
                StatefulListener::GuildDeleteMut(l) => self.guild_delete_mut.push(l),
                StatefulListener::GuildBanAddMut(l) => self.guild_ban_add_mut.push(l),
                StatefulListener::GuildBanRemoveMut(l) => self.guild_ban_remove_mut.push(l),
                StatefulListener::GuildEmojisUpdateMut(l) => self.guild_emojis_update_mut.push(l),
                StatefulListener::GuildIntegrationsUpdateMut(l) => self.guild_integrations_update_mut.push(l),
                StatefulListener::GuildMemberAddMut(l) => self.guild_member_add_mut.push(l),
                StatefulListener::GuildMemberRemoveMut(l) => self.guild_member_remove_mut.push(l),
                StatefulListener::GuildMemberUpdateMut(l) => self.guild_member_update_mut.push(l),
                StatefulListener::GuildMembersChunkMut(l) => self.guild_members_chunk_mut.push(l),
                StatefulListener::GuildRoleCreateMut(l) => self.guild_role_create_mut.push(l),
                StatefulListener::GuildRoleUpdateMut(l) => self.guild_role_update_mut.push(l),
                StatefulListener::GuildRoleDeleteMut(l) => self.guild_role_delete_mut.push(l),
                StatefulListener::InviteCreateMut(l) => self.invite_create_mut.push(l),
                StatefulListener::InviteDeleteMut(l) => self.invite_delete_mut.push(l),
                StatefulListener::MessageCreateMut(l) => self.message_create_mut.push(l),
                StatefulListener::MessageUpdateMut(l) => self.message_update_mut.push(l),
                StatefulListener::MessageDeleteMut(l) => self.message_delete_mut.push(l),
                StatefulListener::MessageDeleteBulkMut(l) => self.message_delete_bulk_mut.push(l),
                StatefulListener::MessageReactionAddMut(l) => self.reaction_add_mut.push(l),
                StatefulListener::MessageReactionRemoveMut(l) => self.reaction_remove_mut.push(l),
                StatefulListener::MessageReactionRemoveAllMut(l) => self.reaction_remove_all_mut.push(l),
                StatefulListener::MessageReactionRemoveEmojiMut(l) => self.reaction_remove_emoji_mut.push(l),
                StatefulListener::PresenceUpdateMut(l) => self.presence_update_mut.push(l),
                StatefulListener::TypingStartMut(l) => self.typing_start_mut.push(l),
                StatefulListener::UserUpdateMut(l) => self.user_update_mut.push(l),
                StatefulListener::VoiceStateUpdateMut(l) => self.voice_state_update_mut.push(l),
                StatefulListener::VoiceServerUpdateMut(l) => self.voice_server_update_mut.push(l),
                StatefulListener::WebhooksUpdateMut(l) => self.webhooks_update_mut.push(l),
            }
        }
    }
}