use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Ident, Span};
use syn::{ItemFn, AttributeArgs, FnArg, Receiver};
use quote::quote;
use darling::FromMeta;
use crate::utils;

#[allow(clippy::cognitive_complexity)]
fn infer_event_type(dispatch_type: &str) -> Option<&'static str> {
    match dispatch_type {
        t if t.contains("ReadyDispatch") => Some("Ready"),
        t if t.contains("ChannelCreateDispatch") => Some("ChannelCreate"),
        t if t.contains("ChannelUpdateDispatch") => Some("ChannelUpdate"),
        t if t.contains("ChannelDeleteDispatch") => Some("ChannelDelete"),
        t if t.contains("ChannelPinsUpdateDispatch") => Some("ChannelPinsUpdate"),
        t if t.contains("GuildCreateDispatch") => Some("GuildCreate"),
        t if t.contains("GuildUpdateDispatch") => Some("GuildUpdate"),
        t if t.contains("GuildDeleteDispatch") => Some("GuildDelete"),
        t if t.contains("GuildBanAddDispatch") => Some("GuildBanAdd"),
        t if t.contains("GuildBanRemoveDispatch") => Some("GuildBanRemove"),
        t if t.contains("GuildEmojisUpdateDispatch") => Some("GuildEmojisUpdate"),
        t if t.contains("GuildIntegrationsUpdateDispatch") => Some("GuildIntegrationsUpdate"),
        t if t.contains("GuildMemberAddDispatch") => Some("GuildMemberAdd"),
        t if t.contains("GuildMemberRemoveDispatch") => Some("GuildMemberRemove"),
        t if t.contains("GuildMemberUpdateDispatch") => Some("GuildMemberUpdate"),
        t if t.contains("GuildMembersChunkDispatch") => Some("GuildMembersChunk"),
        t if t.contains("GuildRoleCreateDispatch") => Some("GuildRoleCreate"),
        t if t.contains("GuildRoleUpdateDispatch") => Some("GuildRoleUpdate"),
        t if t.contains("GuildRoleDeleteDispatch") => Some("GuildRoleDelete"),
        t if t.contains("MessageCreateDispatch") => Some("MessageCreate"),
        t if t.contains("MessageUpdateDispatch") => Some("MessageUpdate"),
        t if t.contains("MessageDeleteDispatch") => Some("MessageDelete"),
        t if t.contains("MessageDeleteBulkDispatch") => Some("MessageDeleteBulk"),
        t if t.contains("MessageReactionAddDispatch") => Some("MessageReactionAdd"),
        t if t.contains("MessageReactionRemoveDispatch") => Some("MessageReactionRemove"),
        t if t.contains("MessageReactionRemoveAllDispatch") => Some("MessageReactionRemoveAll"),
        t if t.contains("MessageReactionRemoveEmojiDispatch") => Some("MessageReactionRemoveEmoji"),
        t if t.contains("PresenceUpdateDispatch") => Some("PresenceUpdate"),
        t if t.contains("TypingStartDispatch") => Some("TypingStart"),
        t if t.contains("UserUpdateDispatch") => Some("UserUpdate"),
        t if t.contains("VoiceStateUpdateDispatch") => Some("VoiceStateUpdate"),
        t if t.contains("VoiceServerUpdateDispatch") => Some("VoiceServerUpdate"),
        t if t.contains("WebhooksUpdateDispatch") => Some("WebhooksUpdate"),
        _ => None
    }
}

fn adapt_method(item: &ItemFn, rcv: &Receiver, arguments: (&Ident, &Ident), event: String) -> TokenStream {
    let (ctx_name, data_name) = arguments;

    //TODO: don't hardcode this
    let self_tokens = if rcv.mutability.is_some() {
        quote!(&'a mut self)
    } else {
        quote!(&'q self)
    };

    let func = &item.sig.ident;
    let reg_name = Ident::new(&format!("__register_{}", item.sig.ident), Span::call_site());
    let dispatch = Ident::new(&format!("{}Dispatch", event), Span::call_site());
    let content = &item.block;

    let event = if rcv.mutability.is_some() {
        Ident::new(&format!("{}Mut", event), Span::call_site())
    } else {
        Ident::new(&event, Span::call_site())
    };

    let quote = quote! {
        //generate an instance of ListenerType struct for registering
        const #reg_name: ::automate::events::StatefulListener<Self> = ::automate::events::StatefulListener::#event(Self::#func);

        //wrapping the function to remove the async and make it compatible with fn pointer by returning a pin
        fn #func<'a>(#self_tokens, #ctx_name: &'a mut Context, #data_name: &'a #dispatch) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = Result<(), Error>> + Send + 'a>> {
            Box::pin(async move {
                #content
            })
        }
    };

    quote.into()
}

fn adapt_function(item: &ItemFn, arguments: (&Ident, &Ident), event: String) -> TokenStream {
    let (ctx_name, data_name) = arguments;

    let func = &item.sig.ident;
    let reg_name = Ident::new(&format!("__register_{}", item.sig.ident), Span::call_site());
    let dispatch = Ident::new(&format!("{}Dispatch", event), Span::call_site());
    let event = Ident::new(&event, Span::call_site());
    let content = &item.block;

    let quote = quote! {
        //generate an instance of ListenerType struct for registering
        const #reg_name: ::automate::events::ListenerType = ::automate::events::ListenerType::#event(#func);

        //wrapping the function to remove the async and make it compatible with fn pointer by returning a pin
        fn #func<'a>(#ctx_name: &'a mut Context, #data_name: &'a #dispatch) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = Result<(), Error>> + Send + 'a>> {
            Box::pin(async move {
                #content
            })
        }
    };

    quote.into()
}

/// Parses the list of arguments for the listener attribute.
///   `#[listener(priority = 5)]`
#[derive(FromMeta)]
struct Args {
    /// The name of the event, not in use anymore.
    #[darling(default)]
    event: Option<String>,
    /// The priority of the event. Not currently
    /// in use.
    #[darling(default)]
    _priority: Option<u8>,
}

pub fn listener(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let metadata_error = metadata.clone();

    let args: AttributeArgs = parse_macro_input!(metadata);
    let args: Args = match Args::from_list(&args) {
        Ok(v) => v,
        Err(e) => { return e.write_errors().into(); }
    };

    let mut input: ItemFn = parse_macro_input!(item);

    if args.event.is_some() {
        compile_error!(TokenStream2::from(metadata_error), "The event type is now automatically inferred for listener functions, please cut down the attribute to `#[listener]`")
    }

    if input.sig.asyncness.is_none() {
        compile_error!(input.sig, "Listener functions must be asynchronous")
    } else {
        //remove the async keyword since the real function won't be async, it will just
        //return a pinned future
        input.sig.asyncness.take();
    }

    let arguments = utils::read_function_arguments(&input.sig);

    if arguments.len() != 2 {
        compile_error!(input.sig.inputs, "Listener functions must take 2 arguments: the first argument should be the session and the second the event dispatch object")
    }

    let (ctx_name, ctx_ty) = &arguments.get(0).unwrap();
    let data_name = &arguments.get(1).unwrap().0;

    if ctx_ty.to_lowercase().contains("session") {
        compile_error!(input.sig.inputs, "&Session argument was replaced by &mut Context, see README.md for examples")
    }

    let event = arguments.get(1).and_then(|(_, ty)| infer_event_type(ty).map(String::from));

    //failed to infer type, raise an error
    if event.is_none() {
        compile_error!(input.sig.inputs, "Could not infer event type: the first argument should be the session and the second the event dispatch object. Make sure you use a correct event dispatch type")
    }

    if let Some(FnArg::Receiver(rcv)) = input.sig.receiver() {
        if rcv.reference.is_none() {
            compile_error!(rcv, "Listener methods must take self by reference")
        }

        adapt_method(&input, rcv, (ctx_name, data_name), event.unwrap())
    } else {
        adapt_function(&input, (ctx_name, data_name), event.unwrap())
    }
}