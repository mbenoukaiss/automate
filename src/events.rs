use async_trait::async_trait;
use std::boxed::Box;
use crate::models::*;
use crate::{Session, Error};

#[async_trait]
pub trait Listener {
    async fn on_typing_start(&mut self, _session: &Session, _data: &TypingStartDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_message_create(&mut self, _session: &Session, _data: &MessageCreateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_message_update(&mut self, _session: &Session, _data: &MessageUpdateDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_message_delete(&mut self, _session: &Session, _data: &MessageDeleteDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_message_delete_bulk(&mut self, _session: &Session, _data: &MessageDeleteBulkDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_reaction_add(&mut self, _session: &Session, _data: &MessageReactionAddDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_reaction_remove(&mut self, _session: &Session, _data: &MessageReactionRemoveDispatch) -> Result<(), Error> {
        Ok(())
    }

    async fn on_reaction_remove_all(&mut self, _session: &Session, _data: &MessageReactionRemoveAllDispatch) -> Result<(), Error> {
        Ok(())
    }
}