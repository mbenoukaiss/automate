mod guild;
mod channel;
mod user;

pub use guild::*;
pub use channel::*;
pub use user::*;

use crate::gateway::*;
use futures::lock::{Mutex, MutexGuard};
use std::collections::HashMap;
use std::any::{TypeId, Any};

pub trait Stored {
    type Storage: Default + Send + Sync;
}

pub trait Storage: Any + Send + Sync {
    type Key;
    type Stored: Clone;

    /// Finds an element by its id. If the element does not
    /// exist, the function will panic. If you are not sure
    /// whether the element is in the storage, you should
    /// use [Storage::find](automate::storage::Storage::find)
    /// instead.
    fn get(&self, id: &Self::Key) -> &Self::Stored;

    /// Finds an element by its id. If the element does not
    /// exist, none will be returned.
    fn find(&self, id: &Self::Key) -> Option<&Self::Stored>;

    fn insert(&mut self, key: &Self::Key, val: &Self::Stored);
}

pub struct StorageContainer {
    storages: HashMap<TypeId, Box<dyn Any + Send + Sync>>
}

impl StorageContainer {
    pub(crate) fn empty() -> StorageContainer {
        StorageContainer {
            storages: HashMap::with_capacity(5)
        }
    }

    pub async fn initialize<T: Stored + 'static>(&mut self) {
        if !self.storages.contains_key(&TypeId::of::<T>()) {
            self.storages.insert(TypeId::of::<T>(), Box::new(Mutex::new(T::Storage::default())));
        }
    }

    pub async fn lock<T: Stored + 'static>(&self) -> MutexGuard<'_, T::Storage> {
        self.storages
            .get(&TypeId::of::<T>()).expect("Storage has never been initialized")
            .downcast_ref::<Mutex<T::Storage>>().unwrap()
            .lock().await
    }

    fn write_in<T: Stored + 'static, F>(&mut self, callback: F)
        where F: FnOnce(&mut T::Storage) {
        if let Some(storage) = self.storages.get_mut(&TypeId::of::<T>()) {
            callback(storage.downcast_mut::<Mutex<T::Storage>>().unwrap().get_mut());
        } else {
            let mut storage = T::Storage::default();
            callback(&mut storage);

            self.storages.insert(TypeId::of::<T>(), Box::new(Mutex::new(storage)));
        }
    }

    pub fn on_ready(&mut self, event: &ReadyDispatch) {
        self.write_in::<Guild, _>(|_| {});

        self.write_in::<Channel, _>(|storage| {
            for channel in &event.private_channels {
                storage.insert(&channel.id, channel)
            }
        });

        self.write_in::<User, _>(|storage| {
            storage.insert(&event.user.id, &event.user);
        });
    }

    pub fn on_channel_create(&mut self, event: &ChannelCreateDispatch) {}

    pub fn on_channel_update(&mut self, event: &ChannelUpdateDispatch) {}

    pub fn on_channel_delete(&mut self, event: &ChannelDeleteDispatch) {}

    pub fn on_channel_pins_update(&mut self, event: &ChannelPinsUpdateDispatch) {}

    pub fn on_guild_create(&mut self, event: &GuildCreateDispatch) {
        let guild = &event.0;

        self.write_in::<Guild, _>(|storage| {
            storage.insert(&guild.id, &guild);
        });

        self.write_in::<Channel, _>(|storage| {
            if let Some(channels) = &guild.channels {
                for channel in channels {
                    storage.insert(&channel.id, channel)
                }
            }
        });

        self.write_in::<User, _>(|storage| {
            if let Some(members) = &guild.members {
                for member in members {
                    storage.insert(&guild.id, &member.user);
                }
            }
        });
    }

    pub fn on_guild_update(&mut self, event: &GuildUpdateDispatch) {}

    pub fn on_guild_delete(&mut self, event: &GuildDeleteDispatch) {}

    pub fn on_guild_ban_add(&mut self, event: &GuildBanAddDispatch) {}

    pub fn on_guild_ban_remove(&mut self, event: &GuildBanRemoveDispatch) {}

    pub fn on_guild_emojis_update(&mut self, event: &GuildEmojisUpdateDispatch) {}

    pub fn on_guild_integrations_update(&mut self, event: &GuildIntegrationsUpdateDispatch) {}

    pub fn on_guild_member_add(&mut self, event: &GuildMemberAddDispatch) {}

    pub fn on_guild_member_remove(&mut self, event: &GuildMemberRemoveDispatch) {}

    pub fn on_guild_member_update(&mut self, event: &GuildMemberUpdateDispatch) {}

    pub fn on_guild_members_chunk(&mut self, event: &GuildMembersChunkDispatch) {}

    pub fn on_guild_role_create(&mut self, event: &GuildRoleCreateDispatch) {}

    pub fn on_guild_role_update(&mut self, event: &GuildRoleUpdateDispatch) {}

    pub fn on_guild_role_delete(&mut self, event: &GuildRoleDeleteDispatch) {}

    pub fn on_invite_create(&mut self, event: &InviteCreateDispatch) {}

    pub fn on_invite_delete(&mut self, event: &InviteDeleteDispatch) {}

    pub fn on_message_create(&mut self, event: &MessageCreateDispatch) {}

    pub fn on_message_update(&mut self, event: &MessageUpdateDispatch) {}

    pub fn on_message_delete(&mut self, event: &MessageDeleteDispatch) {}

    pub fn on_message_delete_bulk(&mut self, event: &MessageDeleteBulkDispatch) {}

    pub fn on_reaction_add(&mut self, event: &MessageReactionAddDispatch) {}

    pub fn on_reaction_remove(&mut self, event: &MessageReactionRemoveDispatch) {}

    pub fn on_reaction_remove_all(&mut self, event: &MessageReactionRemoveAllDispatch) {}

    pub fn on_reaction_remove_emoji(&mut self, event: &MessageReactionRemoveEmojiDispatch) {}

    pub fn on_presence_update(&mut self, event: &PresenceUpdateDispatch) {}

    pub fn on_typing_start(&mut self, event: &TypingStartDispatch) {}

    pub fn on_user_update(&mut self, event: &UserUpdateDispatch) {}

    pub fn on_voice_state_update(&mut self, event: &VoiceStateUpdateDispatch) {}

    pub fn on_voice_server_update(&mut self, event: &VoiceServerUpdateDispatch) {}

    pub fn on_webhooks_update(&mut self, event: &WebhooksUpdateDispatch) {}
}
