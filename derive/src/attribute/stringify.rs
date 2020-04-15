use proc_macro2::TokenStream as TokenStream2;
use proc_macro::{TokenStream, TokenTree};
use syn::{parse_macro_input, ItemEnum};
use quote::{quote, ToTokens};

pub fn stringify(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let item: ItemEnum = parse_macro_input!(item);

    let case: String = match metadata.into_iter().next() {
        Some(TokenTree::Ident(ident)) => ident.to_string().to_ascii_lowercase(),
        _ => compile_error!("Expected arguments under the format (snake_case|upper_snake_case|camel_case|pascal_case)")
    };

    let serde_case = match case.as_str() {
        "snake_case" => "snake_case",
        "upper_snake_case" => "SCREAMING_SNAKE_CASE",
        "camel_case" => "camelCase",
        "pascal_case" => "PascalCase",
        _ => compile_error!("Expected arguments under the format (snake_case|upper_snake_case|camel_case|pascal_case)")
    };

    let mut output: TokenStream2 = quote!(#[derive(Clone, Debug, Serialize, Deserialize)]);
    output.extend(quote!(#[serde(rename_all(deserialize = #serde_case))]));
    item.to_tokens(&mut output);

    TokenStream::from(output)
}