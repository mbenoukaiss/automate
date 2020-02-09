use async_trait::async_trait;
use std::boxed::Box;
use crate::gateway::*;
use crate::{Session, Error};

macro_rules! listener {
    {$($name:ident: $type:ty),*} => {
        #[doc="Trait with all the events a gateway"]
        #[doc="may receive and transmit to the listeners."]
        #[async_trait]
        pub trait Listener {
            $(
            async fn $name(self: &mut Self, _session: &Session, _data: &$type) -> Result<(), Error> {
                Ok(())
            }
            )*
        }
    }
}

listener! {
    on_ready: ReadyDispatch,
    on_channel_create: ChannelCreateDispatch,
    on_channel_update: ChannelUpdateDispatch,
    on_channel_delete: ChannelDeleteDispatch,
    on_channel_pins_update: ChannelPinsUpdateDispatch,
    on_guild_create: GuildCreateDispatch,
    on_guild_update: GuildUpdateDispatch,
    on_guild_delete: GuildDeleteDispatch,
    on_guild_ban_add: GuildBanAddDispatch,
    on_guild_ban_remove: GuildBanRemoveDispatch,
    on_guild_emojis_update: GuildEmojisUpdateDispatch,
    on_guild_integrations_update: GuildIntegrationsUpdateDispatch,
    on_guild_member_add: GuildMemberAddDispatch,
    on_guild_member_remove: GuildMemberRemoveDispatch,
    on_guild_member_update: GuildMemberUpdateDispatch,
    on_guild_members_chunk: GuildMembersChunkDispatch,
    on_guild_role_create: GuildRoleCreateDispatch,
    on_guild_role_update: GuildRoleUpdateDispatch,
    on_guild_role_delete: GuildRoleDeleteDispatch,
    on_message_create: MessageCreateDispatch,
    on_message_update: MessageUpdateDispatch,
    on_message_delete: MessageDeleteDispatch,
    on_message_delete_bulk: MessageDeleteBulkDispatch,
    on_reaction_add: MessageReactionAddDispatch,
    on_reaction_remove: MessageReactionRemoveDispatch,
    on_reaction_remove_all: MessageReactionRemoveAllDispatch,
    on_presence_update: PresenceUpdateDispatch,
    on_typing_start: TypingStartDispatch,
    on_user_update: UserUpdateDispatch,
    on_voice_state_update: VoiceStateUpdateDispatch,
    on_voice_server_update: VoiceServerUpdateDispatch,
    on_webhooks_update: WebhooksUpdateDispatch
}
