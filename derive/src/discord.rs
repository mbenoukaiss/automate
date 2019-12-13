use proc_macro::TokenStream;
use syn::DeriveInput;
use quote::quote;
use crate::utils::Arguments;

pub const OBJECT_ERROR: &str = "Expected arguments under the format: ([client|server|both])";
pub const PAYLOAD_ERROR: &str = "Expected arguments under the format: (op = <u8>, [client|server|both])";

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

        if args.contains_key("default") {
            default_traits.push(quote!(Default));
        }

        match self {
            StructSide::Client => quote!(#[derive(#(#default_traits),*, AsJson)]),
            StructSide::Server => quote!(#[derive(#(#default_traits),*, FromJson)]),
            StructSide::Both => quote!(#[derive(#(#default_traits),*, AsJson, FromJson)])
        }.into()
    }


    pub fn from_args(arguments: &Arguments) -> Self {
        if arguments.contains_key("client") {
            StructSide::Client
        } else if arguments.contains_key("server") {
            StructSide::Server
        } else if arguments.contains_key("both") {
            StructSide::Both
        } else {
            panic!(OBJECT_ERROR);
        }
    }
}

pub fn append_client_quote(input: &DeriveInput, opcode: u8, quote: &mut TokenStream) {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let message_from = quote! {
            impl #impl_generics From<#struct_name #ty_generics> for ::ws::Message #where_clause {
                fn from(origin: #struct_name #ty_generics) -> Self {
                    let mut msg = String::with_capacity(14);
                    msg.push_str(concat!("{\"op\":", #opcode, ",\"d\":"));
                    msg.push_str(&::automate::encode::AsJson::as_json(&origin));
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