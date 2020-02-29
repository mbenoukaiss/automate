use proc_macro::{TokenStream};
use proc_macro2::{TokenStream as TokenStream2, Ident, Span};
use syn::{ItemFn, FnArg, Pat, AttributeArgs};
use quote::ToTokens;
use quote::quote;
use darling::FromMeta;

fn create_trait(names: (&Ident, Ident, Ident), event: String, item: TokenStream2) -> TokenStream {
    let (func, trt) = match event.as_str() {
        "ready" => ("on_ready", "Ready"),
        "channel_create" => ("on_channel_create", "ChannelCreate"),
        "channel_update" => ("on_channel_update", "ChannelUpdate"),
        "channel_delete" => ("on_channel_delete", "ChannelDelete"),
        "channel_pins_update" => ("on_channel_pins_update", "ChannelPinsUpdate"),
        "guild_create" => ("on_guild_create", "GuildCreate"),
        "guild_update" => ("on_guild_update", "GuildUpdate"),
        "guild_delete" => ("on_guild_delete", "GuildDelete"),
        "guild_ban_add" => ("on_guild_ban_add", "GuildBanAdd"),
        "guild_ban_remove" => ("on_guild_ban_remove", "GuildBanRemove"),
        "guild_emojis_update" => ("on_guild_emojis_update", "GuildEmojisUpdate"),
        "guild_integrations_update" => ("on_guild_integrations_update", "GuildIntegrationsUpdate"),
        "guild_member_add" => ("on_guild_member_add", "GuildMemberAdd"),
        "guild_member_remove" => ("on_guild_member_remove", "GuildMemberRemove"),
        "guild_member_update" => ("on_guild_member_update", "GuildMemberUpdate"),
        "guild_members_chunk" => ("on_guild_members_chunk", "GuildMembersChunk"),
        "guild_role_create" => ("on_guild_role_create", "GuildRoleCreate"),
        "guild_role_update" => ("on_guild_role_update", "GuildRoleUpdate"),
        "guild_role_delete" => ("on_guild_role_delete", "GuildRoleDelete"),
        "message_create" => ("on_message_create", "MessageCreate"),
        "message_update" => ("on_message_update", "MessageUpdate"),
        "message_delete" => ("on_message_delete", "MessageDelete"),
        "message_delete_bulk" => ("on_message_delete_bulk", "MessageDeleteBulk"),
        "reaction_add" => ("on_reaction_add", "MessageReactionAdd"),
        "reaction_remove" => ("on_reaction_remove", "MessageReactionRemove"),
        "reaction_remove_all" => ("on_reaction_remove_all", "MessageReactionRemoveAll"),
        "presence_update" => ("on_presence_update", "PresenceUpdate"),
        "typing_start" => ("on_typing_start", "TypingStart"),
        "user_update" => ("on_user_update", "UserUpdate"),
        "voice_state_update" => ("on_voice_state_update", "VoiceStateUpdate"),
        "voice_server_update" => ("on_voice_server_update", "VoiceServerUpdate"),
        "webhooks_update" => ("on_webhooks_update", "WebhooksUpdate"),
        unknown => panic!("Unknown event type {}", unknown)
    };

    let (fn_name, session_name, data_name) = names;

    let func = Ident::new(&func, Span::call_site());
    let trt = Ident::new(&trt, Span::call_site());
    let dispatch = Ident::new(&format!("{}Dispatch", trt), Span::call_site());

    let quote = quote! {
        #[allow(non_camel_case_types)]
        struct #fn_name;

        #[::automate::async_trait]
        impl ::automate::events::#trt for #fn_name {
            async fn #func(&mut self, #session_name: &Session, #data_name: &#dispatch) -> Result<(), Error> {
                #item
            }
        }
    };

    quote.into()
}

/// Parses a list of variable and their values separated by commas.
///
///   #[listener(event = "reaction_add", priority = 5)]
///              ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^
#[derive(Debug, FromMeta)]
struct Args {
    event: String,
    #[darling(default)]
    priority: Option<u8>,
}

pub fn listener(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let args: AttributeArgs = parse_macro_input!(metadata as AttributeArgs);
    let args: Args = match Args::from_list(&args) {
        Ok(v) => v,
        Err(e) => { return e.write_errors().into(); }
    };

    let input: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let content = &input.block;

    let mut session_name = None;
    let mut data_name = None;
    for arg in input.sig.inputs {
        if let FnArg::Typed(arg) = arg {
            if let Pat::Ident(name) = *arg.pat {
                if session_name.is_none() {
                    session_name = Some(name.ident);
                } else if data_name.is_none() {
                    data_name = Some(name.ident);
                } else {
                    panic!("Expected 2 typed arguments but found 3");
                }
            }
        }
    }

    let session_name = session_name.expect("Could not find session argument name");
    let data_name = data_name.expect("Could not find data argument name");

    create_trait((fn_name, session_name, data_name), args.event, content.to_token_stream())
}