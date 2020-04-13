use proc_macro::TokenStream;
use syn::ItemStruct;
use quote::quote;

/// Which side is creating and sending this struct
/// mostly useful to avoid implementing `AsJson` or
/// `Deserialize` on types that don't need them.
pub enum StructSide {
    Server = 1,
    Client = 2,
    Both = 3,
}

impl StructSide {
    #[allow(dead_code)]
    pub fn is_server(&self) -> bool {
        match self {
            StructSide::Server | StructSide::Both => true,
            StructSide::Client => false,
        }
    }

    pub fn is_client(&self) -> bool {
        match self {
            StructSide::Client | StructSide::Both => true,
            StructSide::Server => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_both(&self) -> bool {
        match self {
            StructSide::Both => true,
            _ => false,
        }
    }

    pub fn appropriate_derive(&self, default: bool) -> TokenStream {
        let mut default_traits = vec![quote!(Debug)];

        if default {
            default_traits.push(quote!(Default));
        }

        match self {
            StructSide::Client => quote!(#[derive(#(#default_traits),*, Clone, AsJson)]),
            StructSide::Server => quote!(#[derive(#(#default_traits),*, Clone, ::serde::Deserialize)]),
            StructSide::Both => quote!(#[derive(#(#default_traits),*, Clone, AsJson, ::serde::Deserialize)])
        }.into()
    }
}

pub fn append_client_quote(input: &ItemStruct, opcode: u8, quote: &mut TokenStream) {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let message_from = quote! {
            impl #impl_generics From<#struct_name #ty_generics> for ::tungstenite::Message #where_clause {
                fn from(origin: #struct_name #ty_generics) -> Self {
                    let mut msg = String::with_capacity(14);
                    msg.push_str(concat!("{\"op\":", #opcode, ",\"d\":"));
                    msg.push_str(&::automate::encode::AsJson::as_json(&origin));
                    msg.push('}');

                    ::tungstenite::Message::Text(msg)
                }
            }
    };

    quote.extend(TokenStream::from(message_from));
}

#[allow(unused_variables)]
pub fn append_server_quote(input: &ItemStruct, quote: &mut TokenStream) {}