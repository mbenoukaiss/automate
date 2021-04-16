mod guild;
mod channel;
mod user;

pub use guild::*;
pub use channel::*;
pub use user::*;

use crate::gateway::*;
use std::collections::HashMap;
use std::any::{TypeId, Any};
use crate::{Identifiable, Snowflake};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub trait Stored {
    type Storage: Storage;
}

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

    pub(crate) fn add_initializer<F: Fn(&mut StorageContainer) + Send + Sync + 'static>(&mut self, initializer: F) {
        if !self.storages.is_empty() {
            panic!("Adding initializer to an already initialized storage");
        }

        self.init.push(Box::new(initializer));
    }

    /// Initialize the storage with a default
    /// empty storage.
    pub fn initialize<T: Stored + 'static>(&mut self) where T::Storage: Default {
        self.storages.insert(TypeId::of::<T>(), Box::new(RwLock::new(T::Storage::default())));
    }

    /// Initialize the storage with the provided
    /// existing storage instance.
    pub fn existing<T: Stored + 'static>(&mut self, storage: T::Storage) {
        self.storages.insert(TypeId::of::<T>(), Box::new(RwLock::new(storage)));
    }

    pub async fn read<T: Stored + 'static>(&self) -> RwLockReadGuard<'_, T::Storage> {
        self.storages
            .get(&TypeId::of::<T>()).expect("Storage has never been initialized")
            .downcast_ref::<RwLock<T::Storage>>().expect("Failed to downcast storage")
            .read().await
    }

    pub async fn write<T: Stored + 'static>(&self) -> RwLockWriteGuard<'_, T::Storage> {
        self.storages
            .get(&TypeId::of::<T>()).expect("Storage has never been initialized")
            .downcast_ref::<RwLock<T::Storage>>().expect("Failed to downcast storage")
            .write().await
    }
}

/// Implementation of the utility functions
/// to insert objects
impl StorageContainer {
    /// Insert a guild, its channels and users in
    /// the respective storages.
    #[inline]
    async fn insert_guild(&mut self, guild: &Guild) {
        {
            let mut guilds = self.write::<Guild>().await;
            let mut new_guild = Clone::clone(guild);

            //channels and members are not sent for guild updates, so transfer them from
            //the previous guild object
            if let Some(guild) = guilds.remove(guild.id) {
                new_guild.members = guild.members;
                new_guild.channels = guild.channels;
            }

            guilds.insert(new_guild);
        }

        {
            let mut channels = self.write::<Channel>().await;
            for channel in guild.channels.values() {
                channels.insert(Channel::from_guild(channel));
            }
        }

        {
            let mut users = self.write::<User>().await;
            for member in guild.members.values() {
                Self::insert_user(&mut users, &member.user, Some(guild.id))
            }
        }
    }

    /// Adds a channel and insert its recipients in
    /// the user storage if it is a group channel.
    #[inline]
    async fn insert_channel(&mut self, channel: &Channel) {
        self.write::<Channel>().await.insert(Clone::clone(&channel));

        //insert group channel recipients
        if let Channel::Group(channel) = &channel {
            let mut users = self.write::<User>().await;
            for user in channel.recipients.values() {
                Self::insert_user(&mut users, user, None)
            }
        }
    }

    /// Adds a new role to its guild.
    #[inline]
    async fn insert_role(&mut self, role: &Role, guild: Snowflake) {
        let mut guilds = self.write::<Guild>().await;

        if let Some(guild) = guilds.get_mut(guild) {
            guild.roles.insert(role.id, Clone::clone(&role));
        }
    }

    /// Inserts the user and add it to the given guild
    /// if the user was not already saved, else just add
    /// the guild to the currently saved user.
    #[inline]
    fn insert_user(storage: &mut RwLockWriteGuard<'_, UserStorage>, user: &User, guild: Option<Snowflake>) {
        if let Some(user) = storage.get_mut(user.id) {
            if let Some(guild) = guild {
                user.guilds.insert(guild);
            }
        } else {
            let mut user = Clone::clone(user);
            if let Some(guild) = guild {
                user.guilds.insert(guild);
            }

            storage.insert(user);
        }
    }
}

impl StorageContainer {
    pub async fn on_ready(&mut self, event: &ReadyDispatch) {
        self.initialize::<Guild>();
        self.initialize::<Channel>();
        self.initialize::<User>();

        {
            let mut channels = self.write::<Channel>().await;
            for channel in &event.private_channels {
                channels.insert(Channel::from_private(channel));
            }
        }

        self.write::<User>().await.insert(event.user.clone());
    }

    pub async fn on_channel_create(&mut self, event: &ChannelCreateDispatch) {
        self.insert_channel(&event.0).await;
    }

    pub async fn on_channel_update(&mut self, event: &ChannelUpdateDispatch) {
        self.insert_channel(&event.0).await;
    }

    pub async fn on_channel_delete(&mut self, event: &ChannelDeleteDispatch) {
        self.write::<Channel>().await.remove(event.0.id());
    }

    pub async fn on_channel_pins_update(&mut self, event: &ChannelPinsUpdateDispatch) {
        let mut channels = self.write::<Channel>().await;

        match channels.get_mut(event.channel_id) {
            Some(Channel::Text(c)) => c.last_pin_timestamp = event.last_pin_timestamp.clone(),
            Some(Channel::News(c)) => c.last_pin_timestamp = event.last_pin_timestamp.clone(),
            Some(Channel::Direct(c)) => c.last_pin_timestamp = event.last_pin_timestamp.clone(),
            Some(Channel::Group(c)) => c.last_pin_timestamp = event.last_pin_timestamp.clone(),
            None => (), //the DM was not loaded yet so we can't update it
            _ => panic!("Message-less channel received a pin update")
        };
    }

    pub async fn on_guild_create(&mut self, event: &GuildCreateDispatch) {
        self.insert_guild(&event.0).await;
    }

    pub async fn on_guild_update(&mut self, event: &GuildUpdateDispatch) {
        self.insert_guild(&event.0).await;
    }

    pub async fn on_guild_delete(&mut self, event: &GuildDeleteDispatch) {
        let id = event.id;
        let guild: Guild = Guild::clone(self.read::<Guild>().await.get(id));

        {
            let mut channels = self.write::<Channel>().await;
            for channel in guild.channels.keys() {
                channels.remove(*channel);
            }
        }

        self.write::<Guild>().await.remove(id);
    }

    /// Removal of the user will be handled
    /// by an upcoming member remove event.
    pub async fn on_guild_ban_add(&mut self, _event: &GuildBanAddDispatch) {}

    pub async fn on_guild_ban_remove(&mut self, _event: &GuildBanRemoveDispatch) {}

    pub async fn on_guild_emojis_update(&mut self, event: &GuildEmojisUpdateDispatch) {
        let mut guilds = self.write::<Guild>().await;
        if let Some(guild) = guilds.get_mut(event.guild_id) {
            guild.emojis = event.emojis.clone();
        }
    }

    pub async fn on_guild_integrations_update(&mut self, _event: &GuildIntegrationsUpdateDispatch) {}

    pub async fn on_guild_member_add(&mut self, event: &GuildMemberAddDispatch) {
        Self::insert_user(&mut self.write::<User>().await, &event.member.user, Some(event.guild_id));

        let mut guilds = self.write::<Guild>().await;
        if let Some(guild) = guilds.get_mut(event.guild_id) {
            guild.members.insert(event.member.user.id, event.member.clone());
        }
    }

    pub async fn on_guild_member_remove(&mut self, event: &GuildMemberRemoveDispatch) {
        let mut guilds = self.write::<Guild>().await;
        if let Some(guild) = guilds.get_mut(event.guild_id) {
            guild.members.remove(&event.user.id);
        }
    }

    pub async fn on_guild_member_update(&mut self, event: &GuildMemberUpdateDispatch) {
        let mut guilds = self.write::<Guild>().await;
        if let Some(guild) = guilds.get_mut(event.guild_id) {
            if let Some(member) = guild.members.get_mut(&event.user.id) {
                member.user = event.user.clone();
                member.nick = event.nick.clone();
                member.roles = event.roles.clone();
                member.premium_since = event.premium_since;
            }
        }
    }

    pub async fn on_guild_members_chunk(&mut self, _event: &GuildMembersChunkDispatch) {}

    pub async fn on_guild_role_create(&mut self, event: &GuildRoleCreateDispatch) {
        self.insert_role(&event.role, event.guild_id).await;
    }

    pub async fn on_guild_role_update(&mut self, event: &GuildRoleUpdateDispatch) {
        self.insert_role(&event.role, event.guild_id).await;
    }

    pub async fn on_guild_role_delete(&mut self, event: &GuildRoleDeleteDispatch) {
        let mut guilds = self.write::<Guild>().await;
        if let Some(guild) = guilds.get_mut(event.guild_id) {
            guild.roles.remove(&event.role_id);
        }
    }

    pub async fn on_invite_create(&mut self, _event: &InviteCreateDispatch) {}

    pub async fn on_invite_delete(&mut self, _event: &InviteDeleteDispatch) {}

    pub async fn on_message_create(&mut self, _event: &MessageCreateDispatch) {}

    pub async fn on_message_update(&mut self, _event: &MessageUpdateDispatch) {}

    pub async fn on_message_delete(&mut self, _event: &MessageDeleteDispatch) {}

    pub async fn on_message_delete_bulk(&mut self, _event: &MessageDeleteBulkDispatch) {}

    pub async fn on_reaction_add(&mut self, _event: &MessageReactionAddDispatch) {}

    pub async fn on_reaction_remove(&mut self, _event: &MessageReactionRemoveDispatch) {}

    pub async fn on_reaction_remove_all(&mut self, _event: &MessageReactionRemoveAllDispatch) {}

    pub async fn on_reaction_remove_emoji(&mut self, _event: &MessageReactionRemoveEmojiDispatch) {}

    pub async fn on_presence_update(&mut self, event: &PresenceUpdateDispatch) {
        let update = &event.0;

        let mut guilds = self.write::<Guild>().await;
        if let Some(guild) = guilds.get_mut(event.guild_id) {
            if let Some(member) = guild.members.get_mut(&update.user.id) {
                member.roles = update.roles.clone();
                member.premium_since = update.premium_since;

                if let Some(nick) = &update.nick {
                    member.nick = nick.clone();
                }
            }
        }
    }

    pub async fn on_typing_start(&mut self, _event: &TypingStartDispatch) {}

    pub async fn on_user_update(&mut self, event: &UserUpdateDispatch) {
        let user = &event.0;

        {
            let users = self.read::<User>().await;
            let in_guilds = &users.get(user.id).guilds;

            //update the guild member's user
            let mut guilds = self.write::<Guild>().await;
            for guild in in_guilds {
                if let Some(guild) = guilds.get_mut(*guild) {
                    if let Some(member) = guild.members.get_mut(&user.id) {
                        member.user = Clone::clone(&user);
                    }
                }
            }
        }

        let mut users = self.write::<User>().await;
        if let Some(current_user) = users.get_mut(user.id) {
            current_user.username = user.username.clone();
            current_user.discriminator = user.discriminator.clone();
            current_user.avatar = user.avatar.clone();
            current_user.bot = user.bot;
            current_user.mfa_enabled = user.mfa_enabled;
            current_user.locale = user.locale.clone();
            current_user.verified = user.verified;
            current_user.email = user.email.clone();
            current_user.flags = user.flags;
            current_user.premium_type = user.premium_type;
        }
    }

    pub async fn on_voice_state_update(&mut self, _event: &VoiceStateUpdateDispatch) {}

    pub async fn on_voice_server_update(&mut self, _event: &VoiceServerUpdateDispatch) {}

    pub async fn on_webhooks_update(&mut self, _event: &WebhooksUpdateDispatch) {}
}
