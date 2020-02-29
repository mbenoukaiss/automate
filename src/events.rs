#![allow(unused_variables)]

use async_trait::async_trait;
use crate::gateway::*;
use crate::{Session, Error};

#[async_trait]
pub trait Ready: Send + Sync + 'static {
    async fn on_ready(&mut self, session: &Session, data: &ReadyDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait ChannelCreate: Send + Sync + 'static {
    async fn on_channel_create(&mut self, session: &Session, data: &ChannelCreateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait ChannelUpdate: Send + Sync + 'static {
    async fn on_channel_update(&mut self, session: &Session, data: &ChannelUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait ChannelDelete: Send + Sync + 'static {
    async fn on_channel_delete(&mut self, session: &Session, data: &ChannelDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait ChannelPinsUpdate: Send + Sync + 'static {
    async fn on_channel_pins_update(&mut self, session: &Session, data: &ChannelPinsUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait GuildCreate: Send + Sync + 'static {
    async fn on_guild_create(&mut self, session: &Session, data: &GuildCreateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait GuildUpdate: Send + Sync + 'static {
    async fn on_guild_update(&mut self, session: &Session, data: &GuildUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait GuildDelete: Send + Sync + 'static {
    async fn on_guild_delete(&mut self, session: &Session, data: &GuildDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait GuildBanAdd: Send + Sync + 'static {
    async fn on_guild_ban_add(&mut self, session: &Session, data: &GuildBanAddDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait GuildBanRemove: Send + Sync + 'static {
    async fn on_guild_ban_remove(&mut self, session: &Session, data: &GuildBanRemoveDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait GuildEmojisUpdate: Send + Sync + 'static {
    async fn on_guild_emojis_update(&mut self, session: &Session, data: &GuildEmojisUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait GuildIntegrationsUpdate: Send + Sync + 'static {
    async fn on_guild_integrations_update(&mut self, session: &Session, data: &GuildIntegrationsUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait GuildMemberAdd: Send + Sync + 'static {
    async fn on_guild_member_add(&mut self, session: &Session, data: &GuildMemberAddDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait GuildMemberRemove: Send + Sync + 'static {
    async fn on_guild_member_remove(&mut self, session: &Session, data: &GuildMemberRemoveDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait GuildMemberUpdate: Send + Sync + 'static {
    async fn on_guild_member_update(&mut self, session: &Session, data: &GuildMemberUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait GuildMembersChunk: Send + Sync + 'static {
    async fn on_guild_members_chunk(&mut self, session: &Session, data: &GuildMembersChunkDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait GuildRoleCreate: Send + Sync + 'static {
    async fn on_guild_role_create(&mut self, session: &Session, data: &GuildRoleCreateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait GuildRoleUpdate: Send + Sync + 'static {
    async fn on_guild_role_update(&mut self, session: &Session, data: &GuildRoleUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait GuildRoleDelete: Send + Sync + 'static {
    async fn on_guild_role_delete(&mut self, session: &Session, data: &GuildRoleDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait InviteCreate: Send + Sync + 'static {
    async fn on_invite_create(&mut self, session: &Session, data: &InviteCreateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait InviteDelete: Send + Sync + 'static {
    async fn on_invite_delete(&mut self, session: &Session, data: &InviteDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait MessageCreate: Send + Sync + 'static {
    async fn on_message_create(&mut self, session: &Session, data: &MessageCreateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait MessageUpdate: Send + Sync + 'static {
    async fn on_message_update(&mut self, session: &Session, data: &MessageUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait MessageDelete: Send + Sync + 'static {
    async fn on_message_delete(&mut self, session: &Session, data: &MessageDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait MessageDeleteBulk: Send + Sync + 'static {
    async fn on_message_delete_bulk(&mut self, session: &Session, data: &MessageDeleteBulkDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait MessageReactionAdd: Send + Sync + 'static {
    async fn on_reaction_add(&mut self, session: &Session, data: &MessageReactionAddDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait MessageReactionRemove: Send + Sync + 'static {
    async fn on_reaction_remove(&mut self, session: &Session, data: &MessageReactionRemoveDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait MessageReactionRemoveAll: Send + Sync + 'static {
    async fn on_reaction_remove_all(&mut self, session: &Session, data: &MessageReactionRemoveAllDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait PresenceUpdate: Send + Sync + 'static {
    async fn on_presence_update(&mut self, session: &Session, data: &PresenceUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait TypingStart: Send + Sync + 'static {
    async fn on_typing_start(&mut self, session: &Session, data: &TypingStartDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait UserUpdate: Send + Sync + 'static {
    async fn on_user_update(&mut self, session: &Session, data: &UserUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait VoiceStateUpdate: Send + Sync + 'static {
    async fn on_voice_state_update(&mut self, session: &Session, data: &VoiceStateUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait VoiceServerUpdate: Send + Sync + 'static {
    async fn on_voice_server_update(&mut self, session: &Session, data: &VoiceServerUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
pub trait WebhooksUpdate: Send + Sync + 'static {
    async fn on_webhooks_update(&mut self, session: &Session, data: &WebhooksUpdateDispatch) -> Result<(), Error> {
        Ok(())
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
