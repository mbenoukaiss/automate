use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, Path, PathSegment, Token};
use syn::spanned::Spanned;

/// Parses a comma separated list of path to
/// functions with the `#[listener]` attribute
struct Listeners(Vec<Path>);

impl Parse for Listeners {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut functions = Vec::new();

        while let Ok(mut path) = input.parse::<Path>() {
            if input.parse::<Token![,]>().is_err() && input.peek(Ident) {
                path.span()
                    .unwrap()
                    .error("Expected `,` after listener")
                    .emit();
            }

            let original = path.segments.pop().unwrap().into_value().ident;

            //take the instance of ListenerType struct for registering
            path.segments.push(PathSegment::from(Ident::new(&format!("__register_{}", original), original.span())));

            functions.push(path);
        }

        Ok(Listeners(functions))
    }
}

pub fn functions(input: TokenStream) -> TokenStream {
    let functions = parse_macro_input!(input as Listeners).0;

    let expanded = quote! {
        vec![#(#functions),*]
    };

    TokenStream::from(expanded)
}