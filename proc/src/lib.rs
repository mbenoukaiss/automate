extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Ident;
use syn::{parse_macro_input, DeriveInput, Data, Type};
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
            StructSide::Client => quote!(#[derive(Debug, AsJson)]),
            StructSide::Server => quote!(#[derive(Debug, ::serde::Deserialize)]),
            StructSide::Both => quote!(#[derive(Debug, AsJson, ::serde::Deserialize)])
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

fn create_json_structure(input: &DeriveInput) -> (String, Vec<&Ident>, Vec<&Ident>) {
    if let Data::Struct(data_struct) = &input.data {
        let mut json = String::new();
        let mut fields = Vec::new();
        let mut options = Vec::new();

        for field in &data_struct.fields {
            let ident = field.ident.as_ref().expect("Expected ident for field");

            if let Type::Path(path) = &field.ty {
                if path.path.segments.len() == 1 && path.path.segments.first().unwrap().ident == "Option" {
                    options.push(ident);
                    continue;
                }
            }

            json.push_str(&format!(r#""{}":{{}},"#, ident));
            fields.push(ident);
        }

        json.pop(); //remove trailing comma

        return (json, fields, options);
    } else {
        panic!("AsJson can only be applied to structs"); //Expected struct for proc macros 'object' and 'payload'
    }
}

#[proc_macro_derive(AsJson)]
pub fn as_json(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (format, fields, options) = create_json_structure(&input);
    let quote = quote! {
           impl #impl_generics ::automate::AsJson for #name #ty_generics #where_clause {
               fn as_json(&self) -> String {
                   let mut json = format!(#format, #(::automate::AsJson::as_json(&self.#fields)),*);

                   #(
                    if let Some(optional) = self.#options {
                        json.push_str(&format!(r#","{}":{}"#, stringify!(#options), ::automate::AsJson::as_json(&optional)));
                    }
                   )*

                   json
               }
           }
       };

    quote.into()
}

#[proc_macro_attribute]
pub fn object(metadata: TokenStream, item: TokenStream) -> TokenStream {
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
pub fn payload(metadata: TokenStream, item: TokenStream) -> TokenStream {
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
        }
        _ => panic!(PAYLOAD_ERROR)
    };

    let mut quote = side.appropriate_derive();
    quote.extend(item.clone());

    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if let StructSide::Client = side {
        let message_from = quote! {
            impl #impl_generics From<#name #ty_generics> for ::ws::Message #where_clause {
                fn from(origin: #name #ty_generics) -> Self {
                    ::ws::Message::Text(format!(r#"{{"op":{},"d":{{{}}}}}"#,
                        #opcode,
                        ::automate::AsJson::as_json(&origin)
                    ))
                }
            }
        };

        quote.extend(TokenStream::from(message_from));
    }

    quote
}