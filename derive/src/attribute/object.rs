use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, Item};
use darling::FromMeta;
use quote::ToTokens;
use crate::utils;
use crate::utils::StructSide;

/// Parses the list of variables for a gateway payload
///   `#[object(server)]`
///   `#[object(both)]`
#[derive(FromMeta)]
struct Args {
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

pub fn object(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let args: AttributeArgs = parse_macro_input!(metadata);
    let args: Args = match Args::from_list(&args) {
        Ok(v) => v,
        Err(e) => { return e.write_errors().into(); }
    };

    let item: Item = parse_macro_input!(item);
    let side: StructSide = unwrap!(args.side());

    let mut output = side.appropriate_derive(args.default);

    match item {
        Item::Struct(mut item) => {
            utils::replace_attributes(&mut item);

            unwrap!(utils::extend_with_deref(&item, &mut output));
            item.to_tokens(&mut output);
        },
        any => any.to_tokens(&mut output)
    }

    TokenStream::from(output)
}