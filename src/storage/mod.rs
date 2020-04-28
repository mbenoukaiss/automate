mod guild;
mod channel;
mod user;

pub use guild::*;
pub use channel::*;
pub use user::*;

use crate::gateway::*;
use crate::Snowflake;

pub trait Stored {
    type Storage;

    fn read(container: &StorageContainer) -> &Self::Storage;
    fn write(container: &mut StorageContainer) -> &mut Self::Storage;
}

pub trait Storage {
    type Stored;

    /// Finds an element by its id. If the element does not
    /// exist, the function will panic. If you are not sure
    /// whether the element is in the storage, you should
    /// use [Storage::find](automate::storage::Storage::find)
    /// instead.
    fn get(&self, id: Snowflake) -> &Self::Stored;

    /// Finds an element by its id. If the element does not
    /// exist, none will be returned.
    fn find(&self, id: Snowflake) -> Option<&Self::Stored>;

    fn find_by<P>(&self, filter: P) -> Vec<&Self::Stored>
        where P: FnMut(&Self::Stored) -> bool;

    /// Finds the first element that matches the given filter.
    fn find_one_by<P>(&self, filter: P) -> Option<&Self::Stored>
        where P: FnMut(&Self::Stored) -> bool;

    fn insert(&mut self, val: Self::Stored);

    fn insert_all(&mut self, vals: Vec<Self::Stored>) {
        for val in vals {
            self.insert(val);
        }
    }
}

pub struct StorageContainer {
    guild_storage: GuildStorage,
    channel_storage: ChannelStorage,
    user_storage: UserStorage,
}

impl StorageContainer {
    pub(crate) fn empty() -> StorageContainer {
        StorageContainer {
            guild_storage: GuildStorage::default(),
            channel_storage: ChannelStorage::default(),
            user_storage: UserStorage::default(),
        }
    }

    pub fn read<T: Stored>(&self) -> &T::Storage {
        T::read(self)
    }

    pub fn write<T: Stored>(&mut self) -> &mut T::Storage {
        T::write(self)
    }

    pub fn on_ready(&mut self, event: ReadyDispatch) {
        self.user_storage.insert(event.user);

        for channel in event.private_channels {
            self.channel_storage.insert(channel)
        }
    }

    pub fn on_channel_create(&mut self, event: ChannelCreateDispatch) {

    }

    pub fn on_channel_update(&mut self, event: ChannelUpdateDispatch) {

    }

    pub fn on_channel_delete(&mut self, event: ChannelDeleteDispatch) {

    }

    pub fn on_channel_pins_update(&mut self, event: ChannelPinsUpdateDispatch) {

    }

    pub fn on_guild_create(&mut self, event: GuildCreateDispatch) {
        self.guild_storage.insert(event.0.clone());

        let guild = event.0;

        if let Some(channels) = guild.channels {
            self.channel_storage.insert_all(channels);
        }
        
        for members in guild.members {
            for member in members {
                self.user_storage.insert(member.user);
            }
        }
    }

    pub fn on_guild_update(&mut self, event: GuildUpdateDispatch) {

    }

    pub fn on_guild_delete(&mut self, event: GuildDeleteDispatch) {

    }

    pub fn on_guild_ban_add(&mut self, event: GuildBanAddDispatch) {

    }

    pub fn on_guild_ban_remove(&mut self, event: GuildBanRemoveDispatch) {

    }

    pub fn on_guild_emojis_update(&mut self, event: GuildEmojisUpdateDispatch) {

    }

    pub fn on_guild_integrations_update(&mut self, event: GuildIntegrationsUpdateDispatch) {

    }

    pub fn on_guild_member_add(&mut self, event: GuildMemberAddDispatch) {

    }

    pub fn on_guild_member_remove(&mut self, event: GuildMemberRemoveDispatch) {

    }

    pub fn on_guild_member_update(&mut self, event: GuildMemberUpdateDispatch) {

    }

    pub fn on_guild_members_chunk(&mut self, event: GuildMembersChunkDispatch) {

    }

    pub fn on_guild_role_create(&mut self, event: GuildRoleCreateDispatch) {

    }

    pub fn on_guild_role_update(&mut self, event: GuildRoleUpdateDispatch) {

    }

    pub fn on_guild_role_delete(&mut self, event: GuildRoleDeleteDispatch) {

    }

    pub fn on_invite_create(&mut self, event: InviteCreateDispatch) {

    }

    pub fn on_invite_delete(&mut self, event: InviteDeleteDispatch) {

    }

    pub fn on_message_create(&mut self, event: MessageCreateDispatch) {

    }

    pub fn on_message_update(&mut self, event: MessageUpdateDispatch) {

    }

    pub fn on_message_delete(&mut self, event: MessageDeleteDispatch) {

    }

    pub fn on_message_delete_bulk(&mut self, event: MessageDeleteBulkDispatch) {

    }

    pub fn on_reaction_add(&mut self, event: MessageReactionAddDispatch) {

    }

    pub fn on_reaction_remove(&mut self, event: MessageReactionRemoveDispatch) {

    }

    pub fn on_reaction_remove_all(&mut self, event: MessageReactionRemoveAllDispatch) {

    }

    pub fn on_reaction_remove_emoji(&mut self, event: MessageReactionRemoveEmojiDispatch) {

    }

    pub fn on_presence_update(&mut self, event: PresenceUpdateDispatch) {

    }

    pub fn on_typing_start(&mut self, event: TypingStartDispatch) {

    }

    pub fn on_user_update(&mut self, event: UserUpdateDispatch) {

    }

    pub fn on_voice_state_update(&mut self, event: VoiceStateUpdateDispatch) {

    }

    pub fn on_voice_server_update(&mut self, event: VoiceServerUpdateDispatch) {

    }

    pub fn on_webhooks_update(&mut self, event: WebhooksUpdateDispatch) {

    }

}
