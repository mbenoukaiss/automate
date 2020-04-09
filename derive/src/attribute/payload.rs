use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, DeriveInput};
use quote::quote;
use darling::FromMeta;
use crate::{utils, discord};
use crate::discord::StructSide;

/// Parses the list of variables for a gateway payload
///   `#[payload(op = 0, event = "READY", server)]`
///   `#[payload(op = 8, client)]`
#[derive(FromMeta)]
struct Args {
    op: u8,
    #[darling(default)]
    event: Option<String>,
    #[darling(default)]
    default: bool,

    #[darling(default)]
    client: bool,
    #[darling(default)]
    server: bool,
    #[darling(default)]
    both: bool,
}

impl Args {
    fn side(&self) -> Result<StructSide, TokenStream> {
        if self.client {
            Ok(StructSide::Client)
        } else if self.server {
            Ok(StructSide::Server)
        } else if self.both {
            Ok(StructSide::Both)
        } else {
            compile_error!(err "Expected side in payload attribute")
        }
    }
}

pub fn payload(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let args: AttributeArgs = parse_macro_input!(metadata);
    let args: Args = match Args::from_list(&args) {
        Ok(v) => v,
        Err(e) => { return e.write_errors().into(); }
    };

    let side: StructSide = unwrap!(args.side());

    let mut output: TokenStream = side.appropriate_derive(args.default);
    output.extend(item.clone());

    let input: DeriveInput = parse_macro_input!(item);
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if let Some(event_name) = args.event {
        let constant_impl = quote! {
            impl #impl_generics #struct_name #ty_generics #where_clause {
                pub const EVENT_NAME: &'static str = #event_name;
            }
        };

        output.extend(TokenStream::from(constant_impl));
    }

    unwrap!(utils::extend_with_deref(&input, &mut output));

    if let StructSide::Client = side {
        discord::append_client_quote(&input, args.op, &mut output);
    } else if let StructSide::Server = side {
        discord::append_server_quote(&input, &mut output);
    } else {
        discord::append_client_quote(&input, args.op, &mut output);
        discord::append_server_quote(&input, &mut output);
    }

    output
}