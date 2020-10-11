#![allow(unused_variables)]
//! Defines all the types and macros required to make
//! and register listeners.

use async_trait::async_trait;
use crate::gateway::*;
use crate::{Context, Error};
use std::pin::Pin;
use std::future::Future;

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

macro_rules! fn_types {
    (($fn:ident, $slf:ident, $slfmut: ident), $ty:ty) => {
        pub type $fn = for<'a> fn(&'a Context<'_>, &'a $ty) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
        pub type $slf<T> = for<'a> fn(&'a T, &'a Context<'_>, &'a $ty) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
        pub type $slfmut<T> = for<'a> fn(&'a mut T, &'a Context<'_>, &'a $ty) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>>;
    }
}

fn_types!((Ready, ReadySelf, ReadySelfMut), ReadyDispatch);
fn_types!((ChannelCreate, ChannelCreateSelf, ChannelCreateSelfMut), ChannelCreateDispatch);
fn_types!((ChannelUpdate, ChannelUpdateSelf, ChannelUpdateSelfMut), ChannelUpdateDispatch);
fn_types!((ChannelDelete, ChannelDeleteSelf, ChannelDeleteSelfMut), ChannelDeleteDispatch);
fn_types!((ChannelPinsUpdate, ChannelPinsUpdateSelf, ChannelPinsUpdateSelfMut), ChannelPinsUpdateDispatch);
fn_types!((GuildCreate, GuildCreateSelf, GuildCreateSelfMut), GuildCreateDispatch);
fn_types!((GuildUpdate, GuildUpdateSelf, GuildUpdateSelfMut), GuildUpdateDispatch);
fn_types!((GuildDelete, GuildDeleteSelf, GuildDeleteSelfMut), GuildDeleteDispatch);
fn_types!((GuildBanAdd, GuildBanAddSelf, GuildBanAddSelfMut), GuildBanAddDispatch);
fn_types!((GuildBanRemove, GuildBanRemoveSelf, GuildBanRemoveSelfMut), GuildBanRemoveDispatch);
fn_types!((GuildEmojisUpdate, GuildEmojisUpdateSelf, GuildEmojisUpdateSelfMut), GuildEmojisUpdateDispatch);
fn_types!((GuildIntegrationsUpdate, GuildIntegrationsUpdateSelf, GuildIntegrationsUpdateSelfMut), GuildIntegrationsUpdateDispatch);
fn_types!((GuildMemberAdd, GuildMemberAddSelf, GuildMemberAddSelfMut), GuildMemberAddDispatch);
fn_types!((GuildMemberRemove, GuildMemberRemoveSelf, GuildMemberRemoveSelfMut), GuildMemberRemoveDispatch);
fn_types!((GuildMemberUpdate, GuildMemberUpdateSelf, GuildMemberUpdateSelfMut), GuildMemberUpdateDispatch);
fn_types!((GuildMembersChunk, GuildMembersChunkSelf, GuildMembersChunkSelfMut), GuildMembersChunkDispatch);
fn_types!((GuildRoleCreate, GuildRoleCreateSelf, GuildRoleCreateSelfMut), GuildRoleCreateDispatch);
fn_types!((GuildRoleUpdate, GuildRoleUpdateSelf, GuildRoleUpdateSelfMut), GuildRoleUpdateDispatch);
fn_types!((GuildRoleDelete, GuildRoleDeleteSelf, GuildRoleDeleteSelfMut), GuildRoleDeleteDispatch);
fn_types!((InviteCreate, InviteCreateSelf, InviteCreateSelfMut), InviteCreateDispatch);
fn_types!((InviteDelete, InviteDeleteSelf, InviteDeleteSelfMut), InviteDeleteDispatch);
fn_types!((MessageCreate, MessageCreateSelf, MessageCreateSelfMut), MessageCreateDispatch);
fn_types!((MessageUpdate, MessageUpdateSelf, MessageUpdateSelfMut), MessageUpdateDispatch);
fn_types!((MessageDelete, MessageDeleteSelf, MessageDeleteSelfMut), MessageDeleteDispatch);
fn_types!((MessageDeleteBulk, MessageDeleteBulkSelf, MessageDeleteBulkSelfMut), MessageDeleteBulkDispatch);
fn_types!((MessageReactionAdd, MessageReactionAddSelf, MessageReactionAddSelfMut), MessageReactionAddDispatch);
fn_types!((MessageReactionRemove, MessageReactionRemoveSelf, MessageReactionRemoveSelfMut), MessageReactionRemoveDispatch);
fn_types!((MessageReactionRemoveAll, MessageReactionRemoveAllSelf, MessageReactionRemoveAllSelfMut), MessageReactionRemoveAllDispatch);
fn_types!((MessageReactionRemoveEmoji, MessageReactionRemoveEmojiSelf, MessageReactionRemoveEmojiSelfMut), MessageReactionRemoveEmojiDispatch);
fn_types!((PresenceUpdate, PresenceUpdateSelf, PresenceUpdateSelfMut), PresenceUpdateDispatch);
fn_types!((TypingStart, TypingStartSelf, TypingStartSelfMut), TypingStartDispatch);
fn_types!((UserUpdate, UserUpdateSelf, UserUpdateSelfMut), UserUpdateDispatch);
fn_types!((VoiceStateUpdate, VoiceStateUpdateSelf, VoiceStateUpdateSelfMut), VoiceStateUpdateDispatch);
fn_types!((VoiceServerUpdate, VoiceServerUpdateSelf, VoiceServerUpdateSelfMut), VoiceServerUpdateDispatch);
fn_types!((WebhooksUpdate, WebhooksUpdateSelf, WebhooksUpdateSelfMut), WebhooksUpdateDispatch);

macro_rules! container {
    ($($ty:ident -> $var:ident),*) => {
        #[doc(hidden)]
        pub enum ListenerType {
            Stateful(Box<dyn State>),
            $(
             $ty($ty),
            )*
        }

        #[derive(Default, Clone)]
        pub(crate) struct ListenerContainer {
            pub(crate) stateful_listeners: Vec<Box<dyn State>>,

            $(
             pub(crate) $var: Vec<$ty>,
            )*
        }

        impl ListenerContainer {
            pub(crate) fn register(&mut self, listeners: Vec<ListenerType>) {
                for l in listeners {
                    match l {
                        ListenerType::Stateful(l) => self.stateful_listeners.push(l),

                        $(
                         ListenerType::$ty(l) => self.$var.push(l),
                        )*
                    }
                }
            }
        }
    }
}

container!(
    Ready -> ready,
    ChannelCreate -> channel_create,
    ChannelUpdate -> channel_update,
    ChannelDelete -> channel_delete,
    ChannelPinsUpdate -> channel_pins_update,
    GuildCreate -> guild_create,
    GuildUpdate -> guild_update,
    GuildDelete -> guild_delete,
    GuildBanAdd -> guild_ban_add,
    GuildBanRemove -> guild_ban_remove,
    GuildEmojisUpdate -> guild_emojis_update,
    GuildIntegrationsUpdate -> guild_integrations_update,
    GuildMemberAdd -> guild_member_add,
    GuildMemberRemove -> guild_member_remove,
    GuildMemberUpdate -> guild_member_update,
    GuildMembersChunk -> guild_members_chunk,
    GuildRoleCreate -> guild_role_create,
    GuildRoleUpdate -> guild_role_update,
    GuildRoleDelete -> guild_role_delete,
    InviteCreate -> invite_create,
    InviteDelete -> invite_delete,
    MessageCreate -> message_create,
    MessageUpdate -> message_update,
    MessageDelete -> message_delete,
    MessageDeleteBulk -> message_delete_bulk,
    MessageReactionAdd -> reaction_add,
    MessageReactionRemove -> reaction_remove,
    MessageReactionRemoveAll -> reaction_remove_all,
    MessageReactionRemoveEmoji -> reaction_remove_emoji,
    PresenceUpdate -> presence_update,
    TypingStart -> typing_start,
    UserUpdate -> user_update,
    VoiceStateUpdate -> voice_state_update,
    VoiceServerUpdate -> voice_server_update,
    WebhooksUpdate -> webhooks_update
);

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