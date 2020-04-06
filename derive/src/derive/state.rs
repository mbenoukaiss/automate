use proc_macro::{TokenStream};
use proc_macro2::Span;
use syn::{parse_macro_input, DeriveInput, Ident};
use quote::quote;

pub fn state(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let name = input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let methods_storage_name = Ident::new(&format!("METHODS_{}", input.ident).to_uppercase(), Span::call_site());

    TokenStream::from(quote! {
        ::automate::lazy_static::lazy_static! {
            static ref #methods_storage_name: ::automate::events::StatefulListenerStorage<#name #ty_generics> = {
                let mut storage: ::automate::events::StatefulListenerStorage<#name #ty_generics> = Default::default();
                storage.register(#name::<#ty_generics>::initialize());

                storage
            };
        }
        
        #[::automate::async_trait]
        impl #impl_generics ::automate::events::State for #name #ty_generics #where_clause {
            async fn on_ready(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::ReadyDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.ready {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.ready_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_channel_create(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::ChannelCreateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.channel_create {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.channel_create_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_channel_update(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::ChannelUpdateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.channel_update {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.channel_update_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_channel_delete(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::ChannelDeleteDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.channel_delete {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.channel_delete_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_channel_pins_update(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::ChannelPinsUpdateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.channel_pins_update {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.channel_pins_update_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_guild_create(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::GuildCreateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.guild_create {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.guild_create_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_guild_update(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::GuildUpdateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.guild_update {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.guild_update_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_guild_delete(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::GuildDeleteDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.guild_delete {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.guild_delete_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_guild_ban_add(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::GuildBanAddDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.guild_ban_add {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.guild_ban_add_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_guild_ban_remove(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::GuildBanRemoveDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.guild_ban_remove {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.guild_ban_remove_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_guild_emojis_update(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::GuildEmojisUpdateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.guild_emojis_update {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.guild_emojis_update_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_guild_integrations_update(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::GuildIntegrationsUpdateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.guild_integrations_update {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.guild_integrations_update_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_guild_member_add(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::GuildMemberAddDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.guild_member_add {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.guild_member_add_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_guild_member_remove(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::GuildMemberRemoveDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.guild_member_remove {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.guild_member_remove_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_guild_member_update(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::GuildMemberUpdateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.guild_member_update {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.guild_member_update_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_guild_members_chunk(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::GuildMembersChunkDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.guild_members_chunk {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.guild_members_chunk_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_guild_role_create(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::GuildRoleCreateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.guild_role_create {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.guild_role_create_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_guild_role_update(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::GuildRoleUpdateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.guild_role_update {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.guild_role_update_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_guild_role_delete(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::GuildRoleDeleteDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.guild_role_delete {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.guild_role_delete_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_invite_create(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::InviteCreateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.invite_create {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.invite_create_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_invite_delete(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::InviteDeleteDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.invite_delete {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.invite_delete_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_message_create(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::MessageCreateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.message_create {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.message_create_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_message_update(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::MessageUpdateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.message_update {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.message_update_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_message_delete(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::MessageDeleteDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.message_delete {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.message_delete_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_message_delete_bulk(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::MessageDeleteBulkDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.message_delete_bulk {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.message_delete_bulk_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_reaction_add(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::MessageReactionAddDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.reaction_add {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.reaction_add_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_reaction_remove(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::MessageReactionRemoveDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.reaction_remove {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.reaction_remove_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_reaction_remove_all(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::MessageReactionRemoveAllDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.reaction_remove_all {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.reaction_remove_all_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_reaction_remove_emoji(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::MessageReactionRemoveEmojiDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.reaction_remove_emoji {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.reaction_remove_emoji_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_presence_update(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::PresenceUpdateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.presence_update {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.presence_update_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_typing_start(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::TypingStartDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.typing_start {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.typing_start_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_user_update(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::UserUpdateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.user_update {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.user_update_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_voice_state_update(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::VoiceStateUpdateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.voice_state_update {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.voice_state_update_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_voice_server_update(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::VoiceServerUpdateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.voice_server_update {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.voice_server_update_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        
            async fn on_webhooks_update(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::WebhooksUpdateDispatch) -> Result<(), Error> {
                for listener in &#methods_storage_name.webhooks_update {
                    listener(self, ctx, event).await?;
                }
        
                for listener in &#methods_storage_name.webhooks_update_mut {
                    listener(self, ctx, event).await?;
                }
        
                Ok(())
            }
        }
    })
}