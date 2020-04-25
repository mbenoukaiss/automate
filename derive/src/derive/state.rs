use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, DeriveInput, Ident};
use quote::quote;

static EVENTS_LIST: &[(&str, &str)] = &[
    ("ready", "ReadyDispatch"),
    ("channel_create", "ChannelCreateDispatch"),
    ("channel_update", "ChannelUpdateDispatch"),
    ("channel_delete", "ChannelDeleteDispatch"),
    ("channel_pins_update", "ChannelPinsUpdateDispatch"),
    ("guild_create", "GuildCreateDispatch"),
    ("guild_update", "GuildUpdateDispatch"),
    ("guild_delete", "GuildDeleteDispatch"),
    ("guild_ban_add", "GuildBanAddDispatch"),
    ("guild_ban_remove", "GuildBanRemoveDispatch"),
    ("guild_emojis_update", "GuildEmojisUpdateDispatch"),
    ("guild_integrations_update", "GuildIntegrationsUpdateDispatch"),
    ("guild_member_add", "GuildMemberAddDispatch"),
    ("guild_member_remove", "GuildMemberRemoveDispatch"),
    ("guild_member_update", "GuildMemberUpdateDispatch"),
    ("guild_members_chunk", "GuildMembersChunkDispatch"),
    ("guild_role_create", "GuildRoleCreateDispatch"),
    ("guild_role_update", "GuildRoleUpdateDispatch"),
    ("guild_role_delete", "GuildRoleDeleteDispatch"),
    ("invite_create", "InviteCreateDispatch"),
    ("invite_delete", "InviteDeleteDispatch"),
    ("message_create", "MessageCreateDispatch"),
    ("message_update", "MessageUpdateDispatch"),
    ("message_delete", "MessageDeleteDispatch"),
    ("message_delete_bulk", "MessageDeleteBulkDispatch"),
    ("reaction_add", "MessageReactionAddDispatch"),
    ("reaction_remove", "MessageReactionRemoveDispatch"),
    ("reaction_remove_all", "MessageReactionRemoveAllDispatch"),
    ("reaction_remove_emoji", "MessageReactionRemoveEmojiDispatch"),
    ("presence_update", "PresenceUpdateDispatch"),
    ("typing_start", "TypingStartDispatch"),
    ("user_update", "UserUpdateDispatch"),
    ("voice_state_update", "VoiceStateUpdateDispatch"),
    ("voice_server_update", "VoiceServerUpdateDispatch"),
    ("webhooks_update", "WebhooksUpdateDispatch"),
];

fn events_list() -> (Vec<Ident>, Vec<Ident>, Vec<Ident>, Vec<Ident>) {
    let functions = EVENTS_LIST.iter()
        .map(|(f, _)| Ident::new(&format!("on_{}", f), Span::call_site()))
        .collect::<Vec<Ident>>();

    let immutables = EVENTS_LIST.iter()
        .map(|(f, _)| Ident::new(f, Span::call_site()))
        .collect::<Vec<Ident>>();

    let mutables = EVENTS_LIST.iter()
        .map(|(f, _)| Ident::new(&format!("{}_mut", f), Span::call_site()))
        .collect::<Vec<Ident>>();

    let dispatches = EVENTS_LIST.iter()
            .map(|(_, d)| Ident::new(d, Span::call_site()))
            .collect::<Vec<Ident>>();
    
    (functions, immutables, mutables, dispatches)
}

pub fn state(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let name = input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let methods_storage_name = Ident::new(&format!("__methods_{}", input.ident).to_uppercase(), Span::call_site());
    let (functions, immutables, mutables, dispatches) = events_list();
    
    TokenStream::from(quote! {
        ::automate::lazy_static::lazy_static! {
            static ref #methods_storage_name: ::automate::events::StatefulListenerStorage<#name #ty_generics> = {
                let mut storage: ::automate::events::StatefulListenerStorage<#name #ty_generics> = Default::default();
                storage.register(<#name::<#ty_generics> as automate::events::Initializable>::initialize());

                storage
            };
        }

        #[::automate::async_trait]
        impl #impl_generics ::automate::events::State for #name #ty_generics #where_clause {
            #(
                async fn #functions(&mut self, ctx: &mut ::automate::Context, event: &::automate::gateway::#dispatches) {
                    for listener in &#methods_storage_name.#immutables {
                        if let Err(error) = listener(self, ctx, event).await {
                            ::automate::log::error!("Listener to {} failed with: {}", stringify!(#functions), error);
                        }
                    }
            
                    for listener in &#methods_storage_name.#mutables {
                        if let Err(error) = listener(self, ctx, event).await {
                            ::automate::log::error!("Listener to {} failed with: {}", stringify!(#functions), error);
                        }
                    }
                }
            )*
        }
    })
}