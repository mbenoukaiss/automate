use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use proc_macro2::Span;
use syn::{parse_macro_input, DeriveInput, Ident};
use quote::quote;

pub fn stored(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let name = &input.ident;
    let mut storage = None;

    for attr in &input.attrs {
        let name = attr.path.segments.last().unwrap().ident.to_string();
        if name == "storage" {
            let mut tokens = attr.tokens.clone().into_iter();

            if let Some(TokenTree::Group(group)) = tokens.next() {
                if let Some(TokenTree::Ident(ident)) = group.stream().into_iter().next() {
                    storage = Some(ident);
                } else {
                    compile_error!(attr, "Expected identifier");
                }
            } else {
                compile_error!(attr, "Invalid syntax, expected something similar to `#[storage(StructNameStorage)]`");
            }
        }
    }

    let storage = if let Some(storage) = storage {
        storage
    } else {
        Ident::new(&format!("{}Storage", input.ident), Span::call_site())
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    TokenStream::from(quote! {
        #[automatically_derived]
        impl #impl_generics ::automate::storage::Stored for #name #ty_generics #where_clause {
            type Storage = #storage;
        }
    })
}