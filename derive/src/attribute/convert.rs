use proc_macro2::TokenStream as TokenStream2;
use proc_macro::{TokenStream, TokenTree};
use syn::{parse_macro_input, Ident, Expr, ItemEnum};
use quote::{quote, ToTokens};

fn extract_variants(item: &ItemEnum) -> Result<(Vec<&Ident>, Vec<&Expr>), TokenStream> {
    let mut fields_ident: Vec<&Ident> = Vec::new();
    let mut fields_expr: Vec<&Expr> = Vec::new();

    for variant in &item.variants {
        if variant.discriminant.is_none() {
            compile_error!(err "Convert attribute only works with C-like enums")
        }

        let (_, expr) = variant.discriminant.as_ref().unwrap();

        fields_ident.push(&variant.ident);
        fields_expr.push(expr);
    }

    Ok((fields_ident, fields_expr))
}

pub fn convert(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let item: ItemEnum = parse_macro_input!(item);
    let struct_name: &Ident = &item.ident;

    let convertion_type: Ident = match metadata.into_iter().next() {
        Some(TokenTree::Ident(ty)) => {
            Ident::new(&ty.to_string(), ty.span().into())
        }
        _ => compile_error!("Expected arguments under the format (type)")
    };

    let (fields_ident, fields_expr) = unwrap!(extract_variants(&item));

    let mut output: TokenStream2 = quote!(#[derive(Copy, Clone, Debug)]);
    item.to_tokens(&mut output);

    output.extend(quote! {
        #[automatically_derived]
        impl serde::Serialize for #struct_name {
            fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> where S: serde::Serializer {
                let value = match self {
                    #(#struct_name::#fields_ident => #struct_name::#fields_ident as #convertion_type),*
                };

                serde::Serialize::serialize(&value, serializer)
            }
        }

        #[automatically_derived]
        impl<'de> serde::Deserialize<'de> for #struct_name {
            #[allow(non_upper_case_globals)]
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error> where D: serde::Deserializer<'de> {
                #(const #fields_ident: #convertion_type = #fields_expr;)*

                match <#convertion_type as serde::Deserialize>::deserialize(deserializer)? {
                    #(#fields_ident => core::result::Result::Ok(#struct_name::#fields_ident),)*
                    other => core::result::Result::Err(serde::de::Error::custom(format!("No variant of {} found for {}", stringify!(#struct_name), other)))    ,
                }
            }
        }
    });

    TokenStream::from(output)
}