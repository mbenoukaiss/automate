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
    type Storage: Storage;
}

pub trait Storage: Send + Sync {}

pub struct StorageContainer {
    init: Vec<Box<dyn Fn(&mut StorageContainer) + Send + Sync>>,
    storages: HashMap<TypeId, Box<dyn Any + Send + Sync>>
}

/// This implementation of clone is a bit special since
/// instead of cloning the storages which is not possible
/// since they are behind a mutex, it clones the  ̀init`
/// field into the `storages` field.
impl Clone for StorageContainer {
    fn clone(&self) -> Self {
        let mut container = StorageContainer::for_use(self.init.len());

        for callback in &self.init {
            callback(&mut container);
        }

        container
    }
}

impl StorageContainer {
    pub(crate) fn for_initialization() -> StorageContainer {
        StorageContainer {
            init: Vec::with_capacity(5),
            storages: HashMap::new()
        }
    }

    pub(crate) fn for_use(capacity: usize) -> StorageContainer {
        StorageContainer {
            init: Vec::new(),
            storages: HashMap::with_capacity(capacity)
        }
    }

    pub fn add_initializer<F: Fn(&mut StorageContainer) + Send + Sync + 'static>(&mut self, initializer: F) {
        self.init.push(Box::new(initializer));
    }

    pub async fn lock<T: Stored + 'static>(&self) -> MutexGuard<'_, T::Storage> {
        self.storages
            .get(&TypeId::of::<T>()).expect("Storage has never been initialized")
            .downcast_ref::<Mutex<T::Storage>>().expect("Failed to downcast storage")
            .lock().await
    }

    pub fn initialize<T: Stored + 'static>(&mut self) where T::Storage: Default {
        self.storages.insert(TypeId::of::<T>(), Box::new(Mutex::new(T::Storage::default())));
    }

    pub fn existing<T: Stored + 'static>(&mut self, storage: T::Storage) {
        self.storages.insert(TypeId::of::<T>(), Box::new(Mutex::new(storage)));
    }

    fn write<T: Stored + 'static, F>(&mut self, callback: F)
        where T::Storage: Default,
              F: FnOnce(&mut T::Storage) {
        if let Some(storage) = self.storages.get_mut(&TypeId::of::<T>()) {
            callback(storage.downcast_mut::<Mutex<T::Storage>>().unwrap().get_mut());
        } else {
            let mut storage = T::Storage::default();
            callback(&mut storage);

            self.storages.insert(TypeId::of::<T>(), Box::new(Mutex::new(storage)));
        }
    }
}

impl StorageContainer {
    pub fn on_ready(&mut self, event: &ReadyDispatch) {
        self.write::<Guild, _>(|_| {});

        self.write::<Channel, _>(|storage| {
            for channel in &event.private_channels {
                storage.insert(channel)
            }
        });

        self.write::<User, _>(|storage| {
            storage.insert(&event.user);
        });
    }

    pub fn on_channel_create(&mut self, event: &ChannelCreateDispatch) {
        self.write::<Channel, _>(|storage| {
            storage.insert(&event.0);
        });
    }

    pub fn on_channel_update(&mut self, event: &ChannelUpdateDispatch) {
        self.write::<Channel, _>(|storage| {
            storage.insert(&event.0); //insert replaces the previous channel
        });
    }

    pub fn on_channel_delete(&mut self, event: &ChannelDeleteDispatch) {
        self.write::<Channel, _>(|storage| {
            storage.remove(&event.0);
        });
    }

    pub fn on_channel_pins_update(&mut self, event: &ChannelPinsUpdateDispatch) {
        //TODO: figure out what this event does
    }

    pub fn on_guild_create(&mut self, event: &GuildCreateDispatch) {
        let guild = &event.0;

        self.write::<Guild, _>(|storage| {
            storage.insert(guild);
        });

        self.write::<Channel, _>(|storage| {
            if let Some(channels) = &guild.channels {
                for channel in channels {
                    storage.insert(channel)
                }
            }
        });

        self.write::<User, _>(|storage| {
            if let Some(members) = &guild.members {
                for member in members {
                    storage.insert(&member.user);
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
