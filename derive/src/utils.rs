use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{DeriveInput, Data, Fields};
use std::collections::HashMap;

pub type Arguments = HashMap<String, Vec<String>>;

/// Parses the list of arguments.
/// Returns a vector associating the name of an argument
/// such as `op` to the tokens of this argument.
pub fn parse_arguments_list(metadata: TokenStream) -> Arguments {
    let mut arguments: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_arg: Option<String> = None;

    for token in metadata {
        if let Some(arg) = current_arg.as_ref() {
            match token {
                TokenTree::Ident(ident) => arguments.get_mut(arg).unwrap().push(ident.to_string()),
                TokenTree::Literal(lit) => arguments.get_mut(arg).unwrap().push(lit.to_string()),
                TokenTree::Group(group) => arguments.get_mut(arg).unwrap().push(group.to_string()),
                TokenTree::Punct(punct) => {
                    if punct.as_char() == ',' {
                        current_arg = None;
                        continue;
                    }

                    arguments.get_mut(arg).unwrap().push(punct.to_string());
                }
            }
        } else {
            current_arg = Some(extract_token!(Ident in token));
            arguments.insert(current_arg.clone().unwrap(), Vec::new());
        }
    }

    arguments
}

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
                //TODO: raise warning or idk
            }
        }
    }
}