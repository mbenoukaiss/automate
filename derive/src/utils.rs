use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, Ident, Fields, FnArg, Pat, Path, Type, Attribute, Signature};
use syn::parse::Parser;
use syn::spanned::Spanned;
use quote::ToTokens;

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
pub fn extend_with_deref(input: &ItemStruct, quote: &mut TokenStream) -> Result<(), TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();


    if let Fields::Unnamed(unnamed) = &input.fields {
        if unnamed.unnamed.len() == 1 {
            let underlying = &unnamed.unnamed.first().unwrap().ty;

            let deref = quote! {
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
                };

            quote.extend(TokenStream::from(deref));
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