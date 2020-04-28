use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Path};
use crate::utils;

/// Parses a comma separated list of path to
/// functions with the `#[listener]` attribute
struct Listeners(Vec<Path>);

impl Parse for Listeners {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Listeners(utils::parse_functions_list(&input)))
    }
}

pub fn functions(input: TokenStream) -> TokenStream {
    let functions = parse_macro_input!(input as Listeners).0;

    let expanded = quote! {
        vec![#(#functions),*]
    };

    TokenStream::from(expanded)
}