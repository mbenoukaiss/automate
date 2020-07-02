use proc_macro2::TokenStream as TokenStream2;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{ItemStruct, Ident, Fields, FnArg, Pat, Path, PathSegment, Type, Attribute, Signature};
use syn::parse::{Parser, ParseStream};
use syn::spanned::Spanned;

/// Read the arguments in a function's signature and
/// returns a vec with tuples of the function name and
/// the type as a string
pub fn read_function_arguments(signature: &Signature) -> Vec<(Ident, String)> {
    let mut args = Vec::new();

    for arg in &signature.inputs {
        let arg = match arg {
            FnArg::Receiver(_) => continue,
            FnArg::Typed(arg) => match &*arg.pat {
                Pat::Ident(name) => (name.ident.clone(), arg.ty.to_token_stream().to_string()),
                Pat::Wild(wild) => (Ident::new("_", wild.span()), arg.ty.to_token_stream().to_string()),
                unknown => panic!("Received unknown argument name pattern: {:?}", unknown)
            }
        };

        args.push(arg);
    }

    args
}

/// Extends [Deref](std::ops::Deref) and [DerefMut](std::ops::DerefMut)
/// on tuple struct of one element.
pub fn extend_with_deref(input: &ItemStruct, quote: &mut TokenStream2) -> Result<(), TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();


    if let Fields::Unnamed(unnamed) = &input.fields {
        if unnamed.unnamed.len() == 1 {
            let underlying = &unnamed.unnamed.first().unwrap().ty;

            quote.extend(quote! {
                impl #impl_generics ::std::ops::Deref for #name #ty_generics #where_clause {
                    type Target = #underlying;

                    #[inline]
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }

                impl #impl_generics ::std::ops::DerefMut for #name #ty_generics #where_clause {

                    #[inline]
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.0
                    }
                }
            });
        } else {
            compile_error!(err input, "Only tuple structs with one field can be dereferenced")
        }
    }

    Ok(())
}

pub fn is_option(path: &Path) -> bool {
    path.segments.last().unwrap().ident == "Option"
}

enum FieldType {
    Normal,
    Nullable,
    OptionNullable
}

impl FieldType {
    fn from_str(input: &str) -> FieldType {
        match input {
            "nullable" => FieldType::Nullable,
            "option_nullable" => FieldType::OptionNullable,
            _ => FieldType::Normal
        }
    }
}

pub fn replace_attributes(input: &mut ItemStruct) {
    for field in &mut input.fields {
        //if it's not an option, skip it
        match &field.ty {
            Type::Path(p) if is_option(&p.path) => (),
            _ => continue
        }

        let mut field_type =  FieldType::Normal;

        field.attrs.retain(|attr| {
            let attr_name = attr.path.segments.last().unwrap().ident.to_string();
            field_type = FieldType::from_str(&attr_name);

            match field_type {
                FieldType::Nullable | FieldType::OptionNullable => false,
                FieldType::Normal => true
            }
        });

        let skip_none = quote!(skip_serializing_if = "Option::is_none");
        let double_option = quote!(deserialize_with = "automate::encode::json::double_option");

        match field_type {
            //normal optional fields should not be serialized
            FieldType::Normal => {
                let attrs = Attribute::parse_outer
                    .parse2(quote!(#[serde(#skip_none)]))
                    .unwrap();

                field.attrs.extend(attrs);
            },
            //nullable fields must be serialized (as null if they're none) which is serde's
            //default behaviour, so don't add any attribute
            FieldType::Nullable => (),
            //nullable options use the double option function to deserialize
            FieldType::OptionNullable => {
                let attrs = Attribute::parse_outer
                    .parse2(quote!(#[serde(default, #double_option, #skip_none)]))
                    .unwrap();
                field.attrs.extend(attrs);
            }
        };
    }
}

/// Which side is creating and sending this struct
/// mostly useful to avoid implementing `AsJson` or
/// `Deserialize` on types that don't need them.
pub enum StructSide {
    Server = 1,
    Client = 2,
    Both = 3,
}

impl StructSide {
    pub fn appropriate_derive(&self, default: bool) -> TokenStream2 {
        let mut default_traits = vec![quote!(Debug)];

        if default {
            default_traits.push(quote!(Default));
        }

        let mut tokens = match self {
            StructSide::Client => quote!(#[derive(#(#default_traits),*, Clone, serde::Serialize)]),
            StructSide::Server => quote!(#[derive(#(#default_traits),*, Clone, serde::Deserialize)]),
            StructSide::Both => quote!(#[derive(#(#default_traits),*, Clone, serde::Serialize, serde::Deserialize)])
        };

        if cfg!(feature = "strict-deserializer") {
            tokens.extend(quote!(#[cfg_attr(feature = "strict-deserializer", serde(deny_unknown_fields))]));
        }

        tokens
    }
}

pub fn append_client_quote(input: &ItemStruct, opcode: u8, quote: &mut TokenStream2) {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote.extend(quote! {
        impl #impl_generics From<#struct_name #ty_generics> for ::tktungstenite::tungstenite::Message #where_clause {
            fn from(origin: #struct_name #ty_generics) -> Self {
                let mut msg = String::with_capacity(14);
                msg.push_str(concat!("{\"op\":", #opcode, ",\"d\":"));
                msg.push_str(&serde_json::to_string(&origin).expect(concat!("Failed to serialize ", stringify!(#struct_name))));
                msg.push('}');

                ::tktungstenite::tungstenite::Message::Text(msg)
            }
        }
    });
}

#[allow(unused_variables)]
pub fn append_server_quote(input: &ItemStruct, quote: &mut TokenStream2) {}

pub fn parse_functions_list(input: ParseStream) -> Vec<Path> {
    let mut functions = Vec::new();

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

        functions.push(path);
    }

    functions
}