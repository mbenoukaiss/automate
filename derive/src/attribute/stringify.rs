use proc_macro::{TokenStream, TokenTree};
use syn::{parse_macro_input, Ident, Data, DeriveInput};
use quote::quote;

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

pub fn stringify(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let cloned_item = item.clone();

    let input: DeriveInput = parse_macro_input!(item);
    let struct_name: &Ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

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

    let mut fields_ident: Vec<&Ident> = Vec::new();
    let mut fields_str: Vec<String> = Vec::new();

    if let Data::Enum(en) = &input.data {
        for variant in &en.variants {
            if variant.fields.iter().count() > 0 || variant.discriminant.is_some() {
                compile_error!(variant, "Stringify attribute only supports enums without fields")
            }

            let name = match case.as_str() {
                "snake_case" => pascal_to_snake(variant.ident.to_string()),
                "upper_snake_case" => pascal_to_upper_snake(variant.ident.to_string()),
                "camel_case" => pascal_to_camel(variant.ident.to_string()),
                "pascal_case" => variant.ident.to_string(),
                _ => compile_error!("Expected arguments under the format (snake_case|upper_snake_case|camel_case|pascal_case)")
            };

            fields_ident.push(&variant.ident);
            fields_str.push(name);
        }
    } else {
        compile_error!(input, "The stringify attribute only works on enums")
    }

    let mut convertible: TokenStream = quote!(#[derive(Clone, Debug, Deserialize)]).into();
    convertible.extend(TokenStream::from(quote!(#[serde(rename_all(deserialize = #serde_case))])));
    convertible.extend(cloned_item);

    convertible.extend(TokenStream::from(quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[cfg_attr(feature = "aggressive-inlining", inline)]
            fn as_string(&self) -> &'static str {
                match self {
                    #(
                        #struct_name #ty_generics :: #fields_ident => #fields_str
                    ),*
                }
            }
        }

        impl #impl_generics ::automate::encode::AsJson for #struct_name #ty_generics #where_clause {
            #[cfg_attr(feature = "aggressive-inlining", inline)]
            fn as_json(&self) -> String {
                self.as_string().to_owned()
            }

            #[cfg_attr(feature = "aggressive-inlining", inline)]
            fn concat_json(&self, dest: &mut String) {
                dest.push_str(self.as_string());
            }
        }
    }));

    convertible
}