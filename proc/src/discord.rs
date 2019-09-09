use proc_macro::{TokenStream, TokenTree};
use syn::DeriveInput;
use quote::quote;
use std::collections::HashMap;

pub const OBJECT_ERROR: &'static str = "Expected arguments under the format: ([client|server|both])";
pub const PAYLOAD_ERROR: &'static str = "Expected arguments under the format: (op = <u8>, [client|server|both])";

pub enum StructSide {
    Server = 1,
    Client = 2,
    Both = 3,
}

impl StructSide {
    pub fn appropriate_derive(&self) -> TokenStream {
        match self {
            StructSide::Client => quote!(#[derive(Debug, AsJson)]),
            StructSide::Server => quote!(#[derive(Debug, FromJson)]),
            StructSide::Both => quote!(#[derive(Debug, AsJson, FromJson)])
        }.into()
    }
}

impl From<String> for StructSide {
    fn from(side: String) -> Self {
        match side.as_str() {
            "server" => StructSide::Server,
            "client" => StructSide::Client,
            "both" => StructSide::Both,
            _ => panic!("Unknown side '{}', expected 'server', 'client', or 'both'", side)
        }
    }
}

pub fn parse_arguments_list(metadata: TokenStream) -> HashMap<String, Vec<String>> {
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

pub fn append_client_quote(input: &DeriveInput, opcode: u8, quote: &mut TokenStream) {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let message_from = quote! {
            impl #impl_generics From<#struct_name #ty_generics> for ::ws::Message #where_clause {
                fn from(origin: #struct_name #ty_generics) -> Self {
                    ::ws::Message::Text(format!(r#"{{"op":{},"d":{}}}"#,
                        #opcode,
                        ::automatea::AsJson::as_json(&origin)
                    ))
                }
            }
        };

    quote.extend(TokenStream::from(message_from));
}

pub fn append_server_quote(input: &DeriveInput, quote: &mut TokenStream) {

}