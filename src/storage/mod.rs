mod guild;
mod channel;
mod private_channel;
mod user;

pub use guild::*;
pub use channel::*;
pub use private_channel::*;
pub use user::*;

use crate::gateway::*;
use futures::lock::{Mutex, MutexGuard};
use std::collections::HashMap;
use std::any::{TypeId, Any};
use crate::Identifiable;

//TODO: proc macro to auto derive
pub trait Stored {
    type Storage: Storage;
}

//TODO: proc macro to auto derive
pub trait Storage: Send + Sync {}

pub struct StorageContainer {
    init: Vec<Box<dyn Fn(&mut StorageContainer) + Send + Sync>>,
    storages: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
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
            storages: HashMap::new(),
        }
    }

    pub(crate) fn for_use(capacity: usize) -> StorageContainer {
        StorageContainer {
            init: Vec::new(),
            storages: HashMap::with_capacity(capacity),
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

    fn write<T: Stored + 'static, C>(&mut self, callback: C)
        where T::Storage: Default,
              C: FnOnce(&mut T::Storage) {
        if let Some(storage) = self.storages.get_mut(&TypeId::of::<T>()) {
            callback(storage.downcast_mut::<Mutex<T::Storage>>().unwrap().get_mut());
        } else {
            let mut storage = T::Storage::default();
            callback(&mut storage);

            self.storages.insert(TypeId::of::<T>(), Box::new(Mutex::new(storage)));
        }
    }
}

/// Implementation of the utility functions
/// to insert objects
impl StorageContainer {
    fn insert_guild(&mut self, guild: &Guild) {
        self.write::<Guild, _>(|storage| {
            storage.insert(Clone::clone(guild));
        });

        self.write::<Channel, _>(|storage| {
            for channel in guild.channels.values() {
                storage.insert(Channel::from_guild(channel));
            }
        });

        self.write::<User, _>(|storage| {
            for member in guild.members.values() {
                storage.insert(Clone::clone(&member.user));
            }
        });
    }
}

impl StorageContainer {
    pub async fn on_ready(&mut self, event: &ReadyDispatch) {
        self.initialize::<Guild>();

        self.write::<PrivateChannel, _>(|storage| {
            for channel in &event.private_channels {
                storage.insert(Clone::clone(channel));
            }
        });

        self.write::<User, _>(|storage| {
            storage.insert(Clone::clone(&event.user));
        });
    }

    pub async fn on_channel_create(&mut self, event: &ChannelCreateDispatch) {
        self.write::<Channel, _>(|storage| {
            storage.insert(Clone::clone(&event.0));
        });
    }

    pub async fn on_channel_update(&mut self, event: &ChannelUpdateDispatch) {
        self.write::<Channel, _>(|storage| {
            storage.insert(Clone::clone(&event.0));
        });
    }

    pub async fn on_channel_delete(&mut self, event: &ChannelDeleteDispatch) {
        self.write::<Channel, _>(|storage| {
            storage.remove(event.0.id());
        });
    }

    pub async fn on_channel_pins_update(&mut self, event: &ChannelPinsUpdateDispatch) {
        //TODO: figure out what this event does
    }

    pub async fn on_guild_create(&mut self, event: &GuildCreateDispatch) {
        self.insert_guild(&event.0);
    }

    pub async fn on_guild_update(&mut self, event: &GuildUpdateDispatch) {
        self.insert_guild(&event.0);
    }

    //users are not removed because they can be in many guilds
    pub async fn on_guild_delete(&mut self, event: &GuildDeleteDispatch) {
        let id = event.id;
        let guild: Guild = Guild::clone(self.lock::<Guild>().await.get(id));

        self.write::<Channel, _>(|storage| {
            for channel in guild.channels.keys() {
                storage.remove(*channel);
            }
        });

        self.write::<Guild, _>(|storage| {
            storage.remove(id);
        });
    }

    pub async fn on_guild_ban_add(&mut self, event: &GuildBanAddDispatch) {}

    pub async fn on_guild_ban_remove(&mut self, event: &GuildBanRemoveDispatch) {}

    pub async fn on_guild_emojis_update(&mut self, event: &GuildEmojisUpdateDispatch) {}

    pub async fn on_guild_integrations_update(&mut self, event: &GuildIntegrationsUpdateDispatch) {}

    pub async fn on_guild_member_add(&mut self, event: &GuildMemberAddDispatch) {
        self.write::<User, _>(|storage| {
            storage.insert(Clone::clone(&event.member.user));
        });
    }

    pub async fn on_guild_member_remove(&mut self, event: &GuildMemberRemoveDispatch) {}

    pub async fn on_guild_member_update(&mut self, event: &GuildMemberUpdateDispatch) {
        self.write::<Guild, _>(|storage| {
            let mut member = storage.get_mut(event.guild_id)
            .members
            .get_mut(&event.user.id)
            .unwrap();

            member.user = event.user.clone();
            member.nick = event.nick.clone();
            member.roles = event.roles.clone();
            member.premium_since = event.premium_since;
        });

        self.write::<User, _>(|storage| {
            storage.insert(Clone::clone(&event.user));
        });
    }

    pub async fn on_guild_members_chunk(&mut self, event: &GuildMembersChunkDispatch) {}

    pub async fn on_guild_role_create(&mut self, event: &GuildRoleCreateDispatch) {}

    pub async fn on_guild_role_update(&mut self, event: &GuildRoleUpdateDispatch) {}

    pub async fn on_guild_role_delete(&mut self, event: &GuildRoleDeleteDispatch) {}

    pub async fn on_invite_create(&mut self, event: &InviteCreateDispatch) {}

    pub async fn on_invite_delete(&mut self, event: &InviteDeleteDispatch) {}

    pub async fn on_message_create(&mut self, event: &MessageCreateDispatch) {}

    pub async fn on_message_update(&mut self, event: &MessageUpdateDispatch) {}

    pub async fn on_message_delete(&mut self, event: &MessageDeleteDispatch) {}

    pub async fn on_message_delete_bulk(&mut self, event: &MessageDeleteBulkDispatch) {}

    pub async fn on_reaction_add(&mut self, event: &MessageReactionAddDispatch) {}

    pub async fn on_reaction_remove(&mut self, event: &MessageReactionRemoveDispatch) {}

    pub async fn on_reaction_remove_all(&mut self, event: &MessageReactionRemoveAllDispatch) {}

    pub async fn on_reaction_remove_emoji(&mut self, event: &MessageReactionRemoveEmojiDispatch) {}

    pub async fn on_presence_update(&mut self, event: &PresenceUpdateDispatch) {
        let update = &event.0;

        self.write::<Guild, _>(|storage| {
            let mut member = storage.get_mut(update.guild_id)
                .members
                .get_mut(&update.user.id)
                .unwrap();

            //not updating user because we only receive a partial one
            member.roles = update.roles.clone();
            member.premium_since = update.premium_since;

            if let Some(nick) = &update.nick {
                member.nick = nick.clone();
            }
        });
    }

    pub async fn on_typing_start(&mut self, event: &TypingStartDispatch) {}

    pub async fn on_user_update(&mut self, event: &UserUpdateDispatch) {
        //TODO: guild members don't get updated?
        self.write::<User, _>(|storage| {
            storage.insert(Clone::clone(&event.0));
        });
    }

    pub async fn on_voice_state_update(&mut self, event: &VoiceStateUpdateDispatch) {}

    pub async fn on_voice_server_update(&mut self, event: &VoiceServerUpdateDispatch) {}

    pub async fn on_webhooks_update(&mut self, event: &WebhooksUpdateDispatch) {}
}
