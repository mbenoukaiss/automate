extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Ident;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Expr, Error};
use quote::{format_ident, quote};
use crate::discord::StructSide;
use syn::spanned::Spanned;

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
                    impl #impl_generics ::automate::json::AsJson for #name #ty_generics #where_clause {
                        #[inline]
                        fn as_json(&self) -> String {
                            self.0.as_json()
                        }

                        #[inline]
                        fn concat_json(&self, dest: &mut String) {
                            self.0.concat_json(dest)
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
            impl #impl_generics ::automate::json::AsJson for #name #ty_generics #where_clause {
                #[inline]
                fn as_json(&self) -> String {
                    let mut json = String::with_capacity(#recommended_size);
                    json.push('{');

                    #(
                     json.push_str(concat!("\"", #fns, "\":"));
                     ::automate::json::AsJson::concat_json(&self.#fs, &mut json);
                     json.push(',');
                    )*

                    #(
                     if let Some(optional) = &self.#os {
                         json.push_str(concat!("\"", #ons, "\":"));
                         ::automate::json::AsJson::concat_json(optional, &mut json);
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
                     ::automate::json::AsJson::concat_json(&self.#fs, dest);
                     dest.push(',');
                    )*

                    #(
                     if let Some(optional) = &self.#os {
                         dest.push_str(concat!("\"", #ons, "\":"));
                         ::automate::json::AsJson::concat_json(optional, dest);
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

#[proc_macro_derive(FromJson)]
pub fn from_json(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Unnamed(unnamed) = &data_struct.fields {
            if unnamed.unnamed.len() == 1 {
                let quote = quote! {
                    impl #impl_generics ::automate::json::FromJson for #name #ty_generics #where_clause {
                        #[inline]
                        fn from_json(json: &str) -> Result<#name #ty_generics, ::automate::json::JsonError> {
                            Ok(#name #ty_generics(::automate::json::FromJson::from_json(json)?))
                        }
                    }
                };

                return quote.into();
            } else {
                panic!("Structs with multiple unnamed fields are not supported yet");
            }
        }

        let ((fs, fns), (os, ons), _) = json::extract_fields(data_struct);

        //The fs fields escaped with _ to not interact with
        //other variables in from_json like nesting_level
        let mut fs_escaped = Vec::new();
        for ident in &fs {
            fs_escaped.push(format_ident!("_{}", ident));
        }

        //The os fields escaped with _
        let mut os_escaped = Vec::new();
        for ident in &os {
            os_escaped.push(format_ident!("_{}", ident));
        }

        let quote = quote! {
            impl #impl_generics ::automate::json::FromJson for #name #ty_generics #where_clause {
                #[inline]
                fn from_json(json: &str) -> Result<#name #ty_generics, ::automate::json::JsonError> {
                    //let map = ::automate::json::json_object_to_map(json)?;
                    #(let mut #fs_escaped = None;)*
                    #(let mut #os_escaped = None;)*

                    let mut nesting_level = 0;
                    let mut key_idxs: [usize; 2] = [0; 2];
                    let mut val_idxs: [usize; 2] = [0; 2];

                    for (i, c) in json.char_indices() {
                        if c == '{' || c == '[' {
                            nesting_level += 1;
                        } else if c == '}' || c == ']' {
                            nesting_level -= 1;

                            //we hit end of json, but because there isn't a final comma, there is still 1 key/value
                            //pair waiting to be added to the map
                            if nesting_level == 0 && val_idxs[0] != 0 {
                                val_idxs[1] = i;

                                match &json[key_idxs[0]..key_idxs[1]] {
                                    #(#fns => #fs_escaped = Some(::automate::json::FromJson::from_json((&json[val_idxs[0]..val_idxs[1]]).trim())?),)*
                                    #(#ons => #os_escaped = Some(::automate::json::FromJson::from_json((&json[val_idxs[0]..val_idxs[1]]).trim())?),)*

                                    #[cfg(feature = "strict_deserializer")]
                                    _ => error!("Unknown field ({}) found in {} while deserializing {}", &json[key_idxs[0]..key_idxs[1]], stringify!(#name), json),
                                    #[cfg(not(feature = "strict_deserializer"))]
                                    _ => (),
                                }

                                break;
                            }
                        } else if nesting_level == 1 {
                            if c == '"' {
                                if key_idxs[0] == 0 {
                                    key_idxs[0] = i + 1;
                                } else if key_idxs[1] == 0 {
                                    key_idxs[1] = i;
                                }
                            } else if val_idxs[0] == 0 && c == ':' {
                                val_idxs[0] = i + 1;
                            } else if val_idxs[1] == 0 && c == ',' {
                                val_idxs[1] = i;
                                match &json[key_idxs[0]..key_idxs[1]] {
                                    #(#fns => #fs_escaped = Some(::automate::json::FromJson::from_json((&json[val_idxs[0]..val_idxs[1]]).trim())?),)*
                                    #(#ons => #os_escaped = Some(::automate::json::FromJson::from_json((&json[val_idxs[0]..val_idxs[1]]).trim())?),)*

                                    #[cfg(feature = "strict_deserializer")]
                                    _ => error!("Unknown field ({}) found in {} while deserializing {}", &json[key_idxs[0]..key_idxs[1]], stringify!(#name), json),
                                    #[cfg(not(feature = "strict_deserializer"))]
                                    _ => (),
                                }

                                key_idxs = [0; 2];
                                val_idxs = [0; 2];
                            }
                        }
                    }

                    Ok(#name {
                        #(#fs: #fs_escaped.expect(concat!("Could not find ", #fns, " in JSON input")),)*
                        #(#os: #os_escaped,)*
                    })
                }
            }
        };

       quote.into()
    } else {
        panic!("FromJson can only be applied to structs");
    }
}

#[proc_macro_attribute]
pub fn object(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let arguments = discord::parse_arguments_list(metadata);

    let mut quote = StructSide::from_args(&arguments).appropriate_derive(&arguments);
    quote.extend(item.clone());

    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    utils::extend_with_deref(&input, &mut quote);

    quote
}

#[proc_macro_attribute]
pub fn payload(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let arguments = discord::parse_arguments_list(metadata);

    let opcode: u8 = if let Some(tokens) = arguments.get("op") {
        if tokens.len() != 2 {
            panic!(discord::PAYLOAD_ERROR);
        }

        if tokens.get(0).unwrap() != "=" {
            panic!(discord::PAYLOAD_ERROR);
        }

        tokens.get(1).unwrap()
            .parse::<u8>()
            .expect("Expected u8 argument for 'op'")
    } else {
        panic!(discord::PAYLOAD_ERROR);
    };


    let event_name: Option<String> = match arguments.get("event") {
        Some(tokens) => {
            if tokens.len() != 2 {
                panic!(discord::PAYLOAD_ERROR);
            }

            if tokens.get(0).unwrap() != "=" {
                panic!(discord::PAYLOAD_ERROR);
            }

            let name = tokens.get(1).unwrap();
            if name.len() < 3 {
                panic!(discord::PAYLOAD_ERROR);
            }

            Some((&name[1..name.len() - 1]).to_owned())
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
    let mut convertible: TokenStream = quote!(#[derive(Debug)]).into();
    convertible.extend(item.clone());

    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    let struct_name: &Ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (as_method_name, from_method_name, convertion_type): (Ident, Ident, Ident) = match metadata.into_iter().next() {
        Some(TokenTree::Ident(ty)) => {
            let as_method = Ident::new(&format!("as_{}", ty.to_string()), ty.span().into());
            let from_method = Ident::new(&format!("from_{}", ty.to_string()), ty.span().into());
            let ty = Ident::new(&ty.to_string(), ty.span().into());

            (as_method, from_method, ty)
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

    let as_impl = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            fn #as_method_name(&self) -> #convertion_type {
                match self {
                    #(
                     #struct_name #ty_generics :: #fields_ident => #fields_expr
                    ),*
                }
            }

            fn #from_method_name(num: #convertion_type) -> #struct_name #ty_generics {
                match num {
                    #(
                     v if #fields_expr == v => #struct_name #ty_generics :: #fields_ident,
                    )*
                    _ => panic!(format!("{} does not match with any of {}'s values", num, stringify!(#struct_name)))
                }
            }
        }

        impl #impl_generics ::automate::json::AsJson for #struct_name #ty_generics #where_clause {
            #[inline]
            fn as_json(&self) -> String {
                self.#as_method_name().to_string()
            }

            #[inline]
            fn concat_json(&self, dest: &mut String) {
                ::std::fmt::Write::write_fmt(dest, format_args!("{}", self.#as_method_name())).expect("A Display implementation returned an error unexpectedly");
            }
        }

        impl #impl_generics ::automate::json::FromJson for #struct_name #ty_generics #where_clause {
            #[inline]
            fn from_json(json: &str) -> Result<#struct_name #ty_generics, ::automate::json::JsonError> {
                return match json.parse::<#convertion_type>() {
                    #(
                     Ok(v) if #fields_expr == v => Ok(#struct_name #ty_generics :: #fields_ident),
                    )*
                    Ok(v) => ::automate::json::JsonError::err(format!("{} is not a variant of {}", v, stringify!(#struct_name))),
                    Err(err) => ::automate::json::JsonError::err(format!("Failed to parse {} to {}", json, stringify!(#struct_name)))
                }
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
    let mut convertible: TokenStream = quote!(#[derive(Debug)]).into();
    convertible.extend(item.clone());

    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    let struct_name: &Ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let case: String = match metadata.into_iter().next() {
        Some(TokenTree::Ident(ident)) => ident.to_string().to_ascii_lowercase(),
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

            #[inline]
            fn from_string(val: &str) -> #struct_name #ty_generics {
                match val {
                    #(
                        #fields_str => #struct_name #ty_generics :: #fields_ident,
                    )*
                    _ => panic!(format!("{} does not match with any of {}'s values", val, stringify!(#struct_name)))
                }
            }
        }

        impl #impl_generics ::automate::json::AsJson for #struct_name #ty_generics #where_clause {
            #[inline]
            fn as_json(&self) -> String {
                self.as_string().to_owned()
            }

            #[inline]
            fn concat_json(&self, dest: &mut String) {
                dest.push_str(self.as_string());
            }
        }

        impl #impl_generics ::automate::json::FromJson for #struct_name #ty_generics #where_clause {
            #[inline]
            fn from_json(json: &str) -> Result<#struct_name #ty_generics, ::automate::json::JsonError> {
                if json.len() >= 2 && json.starts_with('"') && json.ends_with('"') {
                    return match &json[1..json.len()-1] {
                        #(
                            #fields_str => Ok(#struct_name #ty_generics :: #fields_ident),
                        )*
                        unknown => ::automate::json::JsonError::err(format!("{} is not a variant of {}", unknown, stringify!(#struct_name)))
                    }
                }

                ::automate::json::JsonError::err("Given JSON is not a string")
            }
        }
    };

    convertible.extend(TokenStream::from(as_impl));

    convertible
}
