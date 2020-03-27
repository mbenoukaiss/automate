#![feature(proc_macro_diagnostic)]

#[macro_use]
extern crate syn;

use proc_macro_hack::proc_macro_hack;
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Ident;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Expr, Error};
use quote::quote;
use crate::discord::StructSide;
use syn::spanned::Spanned;

macro_rules! compile_error {
    ($tokens:expr, $msg:literal) => {
        return ::syn::Error::new_spanned($tokens, $msg)
                .to_compile_error()
                .into();
    };
    ($msg:literal) => {
        return ::syn::Error::new_spanned(::proc_macro2::Span::call_site(), $msg)
                .to_compile_error()
                .into();
    };
    (err $tokens:expr, $msg:literal) => {
        return Err(::syn::Error::new_spanned($tokens, $msg)
                .to_compile_error()
                .into());
    };
    (err $msg:literal) => {
        return Err(::syn::Error::new_spanned(::proc_macro2::Span::call_site(), $msg)
                .to_compile_error()
                .into());
    };
}

macro_rules! extract_token {
    ($type:ident in $token:ident) => {
        match $token {
            ::proc_macro::TokenTree::$type(ident) => ident.to_string(),
            _ => panic!("Not enough arguments provided to derive macro")
        }
    };
    ($type:ident in $token:expr) => {
        match $token {
            Some(::proc_macro::TokenTree::$type(ident)) => ident.to_string(),
            _ => panic!("Not enough arguments provided to derive macro")
        }
    };
}

mod attributes;
mod json;
mod discord;
mod utils;

#[proc_macro_derive(AsJson)]
pub fn as_json(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Unnamed(unnamed) = &data_struct.fields {
            if unnamed.unnamed.len() == 1 {
                let quote = quote! {
                    impl #impl_generics ::automate::encode::AsJson for #name #ty_generics #where_clause {
                        #[inline]
                        fn as_json(&self) -> String {
                            ::automate::encode::AsJson::as_json(&self.0)
                        }

                        #[inline]
                        fn concat_json(&self, dest: &mut String) {
                            ::automate::encode::AsJson::concat_json(&self.0, dest)
                        }
                    }
                };

                return quote.into();
            } else {
                panic!("Structs with multiple unnamed fields are not supported yet");
            }
        }

        let ((fs, fns), (os, ons), recommended_size) = json::extract_fields(data_struct);

        let quote = quote! {
            impl #impl_generics ::automate::encode::AsJson for #name #ty_generics #where_clause {
                #[inline]
                fn as_json(&self) -> String {
                    let mut json = String::with_capacity(#recommended_size);
                    json.push('{');

                    #(
                     json.push_str(concat!("\"", #fns, "\":"));
                     ::automate::encode::AsJson::concat_json(&self.#fs, &mut json);
                     json.push(',');
                    )*

                    #(
                     if let Some(optional) = &self.#os {
                         json.push_str(concat!("\"", #ons, "\":"));
                         ::automate::encode::AsJson::concat_json(optional, &mut json);
                         json.push(',');
                     }
                    )*

                    if json.len() > 1 {
                        json.pop(); //remove last comma
                    }

                    json.push('}');

                    json
                }

                #[inline]
                fn concat_json(&self, dest: &mut String) {
                    let original_len = dest.len();
                    dest.push('{');

                    #(
                     dest.push_str(concat!("\"", #fns, "\":"));
                     ::automate::encode::AsJson::concat_json(&self.#fs, dest);
                     dest.push(',');
                    )*

                    #(
                     if let Some(optional) = &self.#os {
                         dest.push_str(concat!("\"", #ons, "\":"));
                         ::automate::encode::AsJson::concat_json(optional, dest);
                         dest.push(',');
                     }
                    )*

                    if dest.len() > original_len + 1 {
                        dest.pop(); //remove last comma
                    }

                    dest.push('}');
                }
            }
        };

        quote.into()
    } else {
        panic!("AsJson can only be applied to structs");
    }
}

#[proc_macro_attribute]
pub fn object(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let arguments = utils::parse_arguments_list(metadata);

    let mut quote = StructSide::from_args(&arguments).appropriate_derive(&arguments);
    quote.extend(item.clone());

    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    utils::extend_with_deref(&input, &mut quote);

    quote
}

#[proc_macro_attribute]
pub fn payload(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let arguments = utils::parse_arguments_list(metadata);

    let opcode: u8 = if let Some(tokens) = arguments.get("op") {
        if tokens.len() != 1 {
            panic!(discord::PAYLOAD_ERROR);
        }

        tokens.get(0).unwrap()
            .to_string()
            .parse::<u8>()
            .expect("Expected u8 argument for 'op'")
    } else {
        panic!(discord::PAYLOAD_ERROR);
    };


    let event_name: Option<String> = match arguments.get("event") {
        Some(tokens) => {
            if tokens.len() != 1 {
                panic!(discord::PAYLOAD_ERROR);
            }

            Some(tokens.get(0).unwrap().to_string().trim_matches('"').to_owned())
        }
        None => None
    };

    let side = StructSide::from_args(&arguments);

    let mut quote = side.appropriate_derive(&arguments);
    quote.extend(item.clone());

    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if let Some(event_name) = event_name {
        let constant_impl = quote! {
            impl #impl_generics #struct_name #ty_generics #where_clause {
                pub const EVENT_NAME: &'static str = #event_name;
            }
        };

        quote.extend(TokenStream::from(constant_impl));
    }

    utils::extend_with_deref(&input, &mut quote);

    if let StructSide::Client = side {
        discord::append_client_quote(&input, opcode, &mut quote);
    } else if let StructSide::Server = side {
        discord::append_server_quote(&input, &mut quote);
    } else {
        discord::append_client_quote(&input, opcode, &mut quote);
        discord::append_server_quote(&input, &mut quote);
    }

    quote
}

#[proc_macro_attribute]
pub fn convert(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let cloned_item = item.clone();

    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    let struct_name: &Ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (as_method_name, convertion_type): (Ident, Ident) = match metadata.into_iter().next() {
        Some(TokenTree::Ident(ty)) => {
            let as_method = Ident::new(&format!("as_{}", ty.to_string()), ty.span().into());
            let ty = Ident::new(&ty.to_string(), ty.span().into());

            (as_method, ty)
        }
        _ => panic!("Expected arguments under the format (type)")
    };

    let mut fields_ident: Vec<&Ident> = Vec::new();
    let mut fields_expr: Vec<&Expr> = Vec::new();

    if let Data::Enum(en) = &input.data {
        for variant in &en.variants {
            if variant.discriminant.is_none() {
                return Error::new(variant.span(), "Convert attribute only supports C-like enums")
                    .to_compile_error()
                    .into();
            }

            let (_, expr) = variant.discriminant.as_ref().unwrap();

            fields_ident.push(&variant.ident);
            fields_expr.push(expr);
        }
    } else {
        return Error::new(input.span(), "The convert attribute only works on enums")
            .to_compile_error()
            .into();
    }

    let mut convertible: TokenStream = quote!(#[derive(Clone, Debug, ::serde_repr::Deserialize_repr)]#[repr(#convertion_type)]).into();
    convertible.extend(cloned_item);

    let as_impl = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            fn #as_method_name(&self) -> #convertion_type {
                match self {
                    #(
                     #struct_name #ty_generics :: #fields_ident => #fields_expr
                    ),*
                }
            }
        }

        impl #impl_generics ::automate::encode::AsJson for #struct_name #ty_generics #where_clause {
            #[inline]
            fn as_json(&self) -> String {
                self.#as_method_name().to_string()
            }

            #[inline]
            fn concat_json(&self, dest: &mut String) {
                ::std::fmt::Write::write_fmt(dest, format_args!("{}", self.#as_method_name())).expect("A Display implementation returned an error unexpectedly");
            }
        }
    };

    convertible.extend(TokenStream::from(as_impl));

    convertible
}

fn pascal_to_snake(val: String) -> String {
    let mut snake = String::new();

    for c in val.chars() {
        let lc = c.to_ascii_lowercase();

        if !snake.is_empty() && lc != c {
            snake.push('_');
        }

        snake.push(lc);
    }

    snake
}

fn pascal_to_upper_snake(val: String) -> String {
    pascal_to_snake(val).to_ascii_uppercase()
}

fn pascal_to_camel(val: String) -> String {
    if !val.is_empty() {
        let fc = val.chars().next().unwrap();

        if fc.to_ascii_lowercase() != fc {
            let mut camel = String::from(&val[0..1]);
            camel.push_str(&val[1..]);

            return camel;
        }
    }

    val
}

#[proc_macro_attribute]
pub fn stringify(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let cloned_item = item.clone();

    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    let struct_name: &Ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let case: String = match metadata.into_iter().next() {
        Some(TokenTree::Ident(ident)) => ident.to_string().to_ascii_lowercase(),
        _ => panic!("Expected arguments under the format (snake_case|upper_snake_case|camel_case|pascal_case)")
    };

    let serde_case = match case.as_str() {
        "snake_case" => "snake_case",
        "upper_snake_case" => "SCREAMING_SNAKE_CASE",
        "camel_case" => "camelCase",
        "pascal_case" => "PascalCase",
        _ => panic!("Expected arguments under the format (snake_case|upper_snake_case|camel_case|pascal_case)")
    };

    let mut fields_ident: Vec<&Ident> = Vec::new();
    let mut fields_str: Vec<String> = Vec::new();

    if let Data::Enum(en) = &input.data {
        for variant in &en.variants {
            if variant.fields.iter().count() > 0 || variant.discriminant.is_some() {
                return Error::new(variant.span(), "Stringify attribute only supports enums without fields")
                    .to_compile_error()
                    .into();
            }

            let name = match case.as_str() {
                "snake_case" => pascal_to_snake(variant.ident.to_string()),
                "upper_snake_case" => pascal_to_upper_snake(variant.ident.to_string()),
                "camel_case" => pascal_to_camel(variant.ident.to_string()),
                "pascal_case" => variant.ident.to_string(),
                _ => panic!("Expected arguments under the format (snake_case|upper_snake_case|camel_case|pascal_case)")
            };

            fields_ident.push(&variant.ident);
            fields_str.push(name);
        }
    } else {
        return Error::new(input.span(), "The stringify attribute only works on enums")
            .to_compile_error()
            .into();
    }

    let mut convertible: TokenStream = quote!(#[derive(Clone, Debug, Deserialize)]).into();
    convertible.extend(TokenStream::from(quote!(#[serde(rename_all(deserialize = #serde_case))])));
    convertible.extend(cloned_item);

    let as_impl = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[inline]
            fn as_string(&self) -> &'static str {
                match self {
                    #(
                        #struct_name #ty_generics :: #fields_ident => #fields_str
                    ),*
                }
            }
        }

        impl #impl_generics ::automate::encode::AsJson for #struct_name #ty_generics #where_clause {
            #[inline]
            fn as_json(&self) -> String {
                self.as_string().to_owned()
            }

            #[inline]
            fn concat_json(&self, dest: &mut String) {
                dest.push_str(self.as_string());
            }
        }
    };

    convertible.extend(TokenStream::from(as_impl));

    convertible
}

#[proc_macro_attribute]
pub fn endpoint(metadata: TokenStream, item: TokenStream) -> TokenStream {
    attributes::endpoint(metadata, item)
}

/// An event listener function.
/// The function takes two arguments, the first being the
/// session which contains data about the bot and methods
/// to send instructions to discord. The second argument
/// is the event dispatch which contains data about the
/// event.
/// The library will call this function each time it
/// receives an event of the type of the second argument.
///
/// # Example
/// ```ignore
/// use automate::{Session, Error, listener};
/// use automate::gateway::MessageCreateDispatch;
///
/// #[listener]
/// async fn hello(_: &mut Context, _: &MessageCreateDispatch) -> Result<(), Error> {
///     println!("Hello!");
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn listener(metadata: TokenStream, item: TokenStream) -> TokenStream {
    attributes::listener(metadata, item)
}

#[proc_macro_hack]
pub fn functions(input: TokenStream) -> TokenStream {
    attributes::functions(input)
}