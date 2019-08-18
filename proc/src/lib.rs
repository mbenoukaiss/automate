extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

macro_rules! extract_token {
    ($type:ident in $token:expr) => {
        match $token {
            Some(::proc_macro::TokenTree::$type(ident)) => ident.to_string(),
            _ => panic!("Not enough arguments provided to proc macro")
        }
    };
}

const OBJECT_ERROR: &'static str = "Expected arguments under the format: ([client|server|both])";
const PAYLOAD_ERROR: &'static str = "Expected arguments under the format: (op = <u8>, [client|server|both])";

enum StructSide {
    Server = 1,
    Client = 2,
    Both = 3,
}

impl StructSide {
    fn appropriate_derive(&self) -> TokenStream {
        match self {
            StructSide::Client => quote!(#[derive(Debug, Serialize)]),
            StructSide::Server => quote!(#[derive(Debug, Deserialize)]),
            StructSide::Both => quote!(#[derive(Debug, Serialize, Deserialize)])
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

#[proc_macro_attribute]
pub fn discord_object(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let metadata: Vec<TokenTree> = metadata.into_iter().collect();

    let side: StructSide = match metadata.len() {
        0 => StructSide::Both,
        1 => StructSide::from(extract_token!(Ident in metadata.get(0))),
        _ => panic!(OBJECT_ERROR)
    };

    let mut quote = side.appropriate_derive();
    quote.extend(item);

    quote
}

#[proc_macro_attribute]
pub fn discord_payload(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let metadata: Vec<TokenTree> = metadata.into_iter().collect();

    let opcode: u8 = {
        if extract_token!(Ident in metadata.get(0)) != "op" {
            panic!(PAYLOAD_ERROR);
        }

        if extract_token!(Punct in metadata.get(1)) != "=" {
            panic!(PAYLOAD_ERROR);
        }

        extract_token!(Literal in metadata.get(2))
            .parse::<u8>()
            .expect("Expected u8 argument for 'op'")
    };

    let side: StructSide = match metadata.len() {
        3 => StructSide::Both,
        5 => {
            if extract_token!(Punct in metadata.get(3)) != "," {
                panic!(PAYLOAD_ERROR);
            }

            StructSide::from(extract_token!(Ident in metadata.get(4)))
        },
        _ => panic!(PAYLOAD_ERROR)
    };

    let mut quote = side.appropriate_derive();
    quote.extend(item.clone());

    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if let StructSide::Client = side {
        let q = quote! {
            impl #impl_generics From<#name #ty_generics> for Message #where_clause {
                fn from(origin: #name #ty_generics) -> Self {
                    Message::Text(format!(r#"{{"op":{},"d":{}}}"#,
                        #opcode,
                        serde_json::to_string(&origin).expect("Failed to serialize payload")
                    ))
                }
            }
        };
        quote.extend(TokenStream::from(q));
    }

    quote
}
