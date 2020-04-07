use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, DeriveInput};
use darling::FromMeta;
use crate::utils;
use crate::discord::StructSide;

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

    let side: StructSide = unwrap!(args.side());

    let mut output = side.appropriate_derive(args.default);
    output.extend(item.clone());

    let input: DeriveInput = parse_macro_input!(item);
    utils::extend_with_deref(&input, &mut output);

    output
}