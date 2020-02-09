use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{DeriveInput, Data, Fields};
use std::collections::HashMap;

pub type Arguments = HashMap<String, Vec<TokenTree>>;

/// Parses the list of arguments.
/// Returns a vector associating the name of an argument
/// such as `op` to the tokens of this argument, the equal
/// sign is not included.
pub fn parse_arguments_list(metadata: TokenStream) -> Arguments {
    let mut arguments = HashMap::new();
    let mut current_name: Option<String> = None;
    let mut current_args: Vec<TokenTree> = Vec::new();

    for token in metadata {
        if current_name.is_some() {
            match token {
                TokenTree::Punct(punct) => match punct.as_char() {
                    '=' => (),
                    ',' => {
                        arguments.insert(current_name.unwrap(), current_args);

                        current_name = None;
                        current_args = Vec::new();
                    }
                    _ => current_args.push(TokenTree::Punct(punct))
                },
                any => current_args.push(any)
            }
        } else {
            current_name = Some(extract_token!(Ident in token));
        }
    }

    if let Some(name) = current_name {
        arguments.insert(name, current_args);
    }

    arguments
}

pub fn extract_string(args: &Arguments, key: &str) -> String {
    let args = args.get(key).unwrap();

    if let Some(TokenTree::Literal(lit)) = args.first() {
        lit.to_string()
    } else {
        panic!("Expected quoted string such as `route = \"/gateway\"`")
    }
}

/// Extends [Deref](std::ops::Deref) and [DerefMut](std::ops::DerefMut)
/// on tuple struct of one element.
pub fn extend_with_deref(input: &DeriveInput, quote: &mut TokenStream) {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();


    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Unnamed(unnamed) = &data_struct.fields {
            if unnamed.unnamed.len() == 1 {
                let underlying = &unnamed.unnamed.first().unwrap().ty;

                let deref = quote! {
                    impl #impl_generics ::std::ops::Deref for #name #ty_generics #where_clause {
                        type Target = #underlying;

                        fn deref(&self) -> &Self::Target {
                            &self.0
                        }
                    }

                    impl #impl_generics ::std::ops::DerefMut for #name #ty_generics #where_clause {

                        fn deref_mut(&mut self) -> &mut Self::Target {
                            &mut self.0
                        }
                    }
                };

                quote.extend(TokenStream::from(deref));
            } else {
                panic!("Structs with multiple unnamed fields are not supported yet");
            }
        }
    }
}