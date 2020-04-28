use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, Path, Token};
use syn::spanned::Spanned;
use crate::utils;

/// Parses a comma separated list of path to
/// methods with the `#[listener]` attribute
struct Listeners {
    strct: Path,
    methods: Vec<Path>
}

impl Parse for Listeners {
    fn parse(input: ParseStream) -> Result<Self> {
        let strct = input.parse::<Path>()?;
        if input.parse::<Token![:]>().is_err() && input.peek(Ident) {
            strct.span()
                .unwrap()
                .error("Expected `:` after struct name")
                .emit();
        }

        Ok(Listeners {
            strct,
            methods: utils::parse_functions_list(&input)
        })
    }
}

pub fn methods(input: TokenStream) -> TokenStream {
    let Listeners { strct, methods } = parse_macro_input!(input as Listeners);

    let expanded = quote! {
        vec![#(#strct::#methods),*]
    };

    TokenStream::from(expanded)
}