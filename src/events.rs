use async_trait::async_trait;
use std::boxed::Box;
use crate::models::*;
use crate::{Session, Error};

macro_rules! listener {
{$($name:ident: $type:ty),*} => {
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
    on_guild_create: GuildCreateDispatch,
    on_guild_update: GuildUpdateDispatch,
    on_message_create: MessageCreateDispatch,
    on_message_update: MessageUpdateDispatch,
    on_message_delete: MessageDeleteDispatch,
    on_message_delete_bulk: MessageDeleteBulkDispatch,
    on_reaction_add: MessageReactionAddDispatch,
    on_reaction_remove: MessageReactionRemoveDispatch,
    on_reaction_remove_all: MessageReactionRemoveAllDispatch,
    on_presence_update: PresenceUpdateDispatch,
    on_typing_start: TypingStartDispatch
}
