use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Ident, Span};
use syn::{ItemFn, FnArg, Signature, Pat, AttributeArgs, Error};
use syn::spanned::Spanned;
use quote::ToTokens;
use quote::quote;
use darling::FromMeta;

fn read_function_arguments(signature: &Signature) -> Vec<(Ident, String)> {
    let mut args = Vec::new();

    for arg in &signature.inputs {
        let arg = match arg {
            FnArg::Receiver(rcv) => (Ident::new("self", rcv.span()), rcv.to_token_stream().to_string()),
            FnArg::Typed(arg) => match &*arg.pat {
                Pat::Ident(name) => (name.ident.clone(), arg.ty.to_token_stream().to_string()),
                Pat::Wild(wild) => (Ident::new("_", wild.span()), arg.ty.to_token_stream().to_string()),
                unknown => panic!("Received unknown argument name pattern: {:?}", unknown)
            }
        };

        args.push(arg);
    }

    args
}

#[allow(clippy::cognitive_complexity)]
fn infer_event_type(dispatch_type: &str) -> Option<&'static str> {
    match dispatch_type {
        t if t.contains("ReadyDispatch") => Some("ready"),
        t if t.contains("ChannelCreateDispatch") => Some("channel_create"),
        t if t.contains("ChannelUpdateDispatch") => Some("channel_update"),
        t if t.contains("ChannelDeleteDispatch") => Some("channel_delete"),
        t if t.contains("ChannelPinsUpdateDispatch") => Some("channel_pins_update"),
        t if t.contains("GuildCreateDispatch") => Some("guild_create"),
        t if t.contains("GuildUpdateDispatch") => Some("guild_update"),
        t if t.contains("GuildDeleteDispatch") => Some("guild_delete"),
        t if t.contains("GuildBanAddDispatch") => Some("guild_ban_add"),
        t if t.contains("GuildBanRemoveDispatch") => Some("guild_ban_remove"),
        t if t.contains("GuildEmojisUpdateDispatch") => Some("guild_emojis_update"),
        t if t.contains("GuildIntegrationsUpdateDispatch") => Some("guild_integrations_update"),
        t if t.contains("GuildMemberAddDispatch") => Some("guild_member_add"),
        t if t.contains("GuildMemberRemoveDispatch") => Some("guild_member_remove"),
        t if t.contains("GuildMemberUpdateDispatch") => Some("guild_member_update"),
        t if t.contains("GuildMembersChunkDispatch") => Some("guild_members_chunk"),
        t if t.contains("GuildRoleCreateDispatch") => Some("guild_role_create"),
        t if t.contains("GuildRoleUpdateDispatch") => Some("guild_role_update"),
        t if t.contains("GuildRoleDeleteDispatch") => Some("guild_role_delete"),
        t if t.contains("MessageCreateDispatch") => Some("message_create"),
        t if t.contains("MessageUpdateDispatch") => Some("message_update"),
        t if t.contains("MessageDeleteDispatch") => Some("message_delete"),
        t if t.contains("MessageDeleteBulkDispatch") => Some("message_delete_bulk"),
        t if t.contains("MessageReactionAddDispatch") => Some("reaction_add"),
        t if t.contains("MessageReactionRemoveDispatch") => Some("reaction_remove"),
        t if t.contains("MessageReactionRemoveAllDispatch") => Some("reaction_remove_all"),
        t if t.contains("PresenceUpdateDispatch") => Some("presence_update"),
        t if t.contains("TypingStartDispatch") => Some("typing_start"),
        t if t.contains("UserUpdateDispatch") => Some("user_update"),
        t if t.contains("VoiceStateUpdateDispatch") => Some("voice_state_update"),
        t if t.contains("VoiceServerUpdateDispatch") => Some("voice_server_update"),
        t if t.contains("WebhooksUpdateDispatch") => Some("webhooks_update"),
        _ => None
    }
}

fn create_trait(names: (&Ident, &Ident, &Ident), event: String, item: TokenStream2) -> TokenStream {
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

    let fn_name = Ident::new(&format!("__listener_{}", fn_name), Span::call_site());
    let func = Ident::new(&func, Span::call_site());
    let trt = Ident::new(&trt, Span::call_site());
    let dispatch = Ident::new(&format!("{}Dispatch", trt), Span::call_site());

    let quote = quote! {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        struct #fn_name;

        #[::automate::async_trait]
        impl ::automate::events::#trt for #fn_name {
            async fn #func(&mut self, #session_name: &Session, #data_name: &#dispatch) -> Result<(), Error> {
                #item
            }
        }

        impl ::automate::events::ListenerMarker for #fn_name {
            fn downcast(self: Box<Self>) -> ::automate::events::ListenerType {
                ::automate::events::ListenerType::#trt(self)
            }
        }
    };

    quote.into()
}

/// Parses a list of variable and their values separated by commas.
///
///   #[listener(event = "reaction_add", priority = 5)]
///              ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^
#[derive(FromMeta)]
struct Args {
    /// The name of the event. It should be
    /// automatically inferred.
    #[darling(default)]
    event: Option<String>,

//    /// The priority of the event. Not currently
//    /// in use.
//    #[darling(default)]
//    priority: Option<u8>,
}

/// An event listener function.
/// The function takes two arguments, the first being the
/// session which contains data about the bot and methods
/// to send instructions to discord. The second argument
/// is the event dispatch which contains data about the
/// event.
/// The library will call this function each time it
/// receives an event of the type of the second argument.
///
/// # Example
/// ```ignore
/// #[listener]
/// async fn hello(_: &Session, _: &MessageCreateDispatch) -> Result<(), Error> {
///     println!("Hello!");
///     Ok(())
/// }
/// ```
pub fn listener(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let args: AttributeArgs = parse_macro_input!(metadata);
    let mut args: Args = match Args::from_list(&args) {
        Ok(v) => v,
        Err(e) => { return e.write_errors().into(); }
    };

    let input: ItemFn = parse_macro_input!(item);
    let fn_name = &input.sig.ident;
    let content = &input.block;

    let arguments = read_function_arguments(&input.sig);

    if let Some((ident, _)) = arguments.get(0) {
        if *ident == "self" {
            return Error::new(input.sig.inputs.span(), "Listener methods in impl blocks are not yet supported.")
                .to_compile_error()
                .into();
        }
    }
    
    let session_name = &arguments.get(0).expect("Could not find session argument name").0;
    let data_name = &arguments.get(1).expect("Could not find data argument name").0;

    //infer type if event was not specified in attribute
    if args.event.is_none() {
        if let Some((_, ty)) = arguments.get(1) {
            args.event = infer_event_type(ty).map(String::from);
        }
    }

    //failed to infer type, raise an error
    if args.event.is_none() {
        return Error::new(input.sig.inputs.span(), "Could not infer event type: the first argument should be the session and the second the event dispatch object. Make sure you use a correct event dispatch type.")
            .to_compile_error()
            .into();
    }

    create_trait((fn_name, session_name, data_name), args.event.unwrap(), content.to_token_stream())
}