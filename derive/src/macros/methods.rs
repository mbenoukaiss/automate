use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, Path, PathSegment, Token};
use syn::spanned::Spanned;

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

        let mut methods = Vec::new();

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

            methods.push(path);
        }

        Ok(Listeners {
            strct,
            methods
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