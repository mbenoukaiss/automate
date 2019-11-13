use proc_macro::{TokenStream, TokenTree};
use syn::DeriveInput;
use quote::quote;
use std::collections::HashMap;

pub const OBJECT_ERROR: &str = "Expected arguments under the format: ([client|server|both])";
pub const PAYLOAD_ERROR: &str = "Expected arguments under the format: (op = <u8>, [client|server|both])";

pub type Arguments = HashMap<String, Vec<String>>;

/// Which side is creating and sending this struct
/// mostly useful to avoid implementing `AsJson` or
/// `FromJson` on types that don't need them.
pub enum StructSide {
    Server = 1,
    Client = 2,
    Both = 3,
}

impl StructSide {
    pub fn appropriate_derive(&self, args: &Arguments) -> TokenStream {
        let mut default_traits = vec![quote!(Debug)];

        if let Some(_) = args.get("default") {
            default_traits.push(quote!(Default));
        }

        match self {
            StructSide::Client => quote!(#[derive(#(#default_traits),*, AsJson)]),
            StructSide::Server => quote!(#[derive(#(#default_traits),*, FromJson)]),
            StructSide::Both => quote!(#[derive(#(#default_traits),*, AsJson, FromJson)])
        }.into()
    }


    pub fn from_args(arguments: &Arguments) -> Self {
        if let Some(_) = arguments.get("client") {
            StructSide::Client
        } else if let Some(_) = arguments.get("server") {
            StructSide::Server
        } else if let Some(_) = arguments.get("both") {
            StructSide::Both
        } else {
            panic!(OBJECT_ERROR);
        }
    }
}

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

pub fn append_client_quote(input: &DeriveInput, opcode: u8, quote: &mut TokenStream) {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let message_from = quote! {
            impl #impl_generics From<#struct_name #ty_generics> for ::ws::Message #where_clause {
                fn from(origin: #struct_name #ty_generics) -> Self {
                    let mut msg = String::with_capacity(14);
                    msg.push_str(concat!("{\"op\":", #opcode, ",\"d\":"));
                    msg.push_str(&::automate::AsJson::as_json(&origin));
                    msg.push('}');

                    ::ws::Message::Text(msg)
                }
            }
        };

    quote.extend(TokenStream::from(message_from));
}

#[allow(unused_variables)]
pub fn append_server_quote(input: &DeriveInput, quote: &mut TokenStream) {

}