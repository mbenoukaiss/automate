use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{ItemStruct, Ident, Fields, FnArg, Pat, Signature};
use syn::spanned::Spanned;
use quote::ToTokens;
use crate::discord::StructSide;

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

pub fn replace_attributes(input: &mut ItemStruct, side: &StructSide) {
    for field in &mut input.fields {
        field.attrs.retain(|attr| {
            let attr_name = attr.path.segments.last().unwrap().ident.to_string();

            //if it's not going to derive AsJson but still has nullable, remove it
            attr_name != "nullable" || side.is_client()
        });

        for attr in &mut field.attrs {
            let attr_name = attr.path.segments.last().unwrap().ident.to_string();

            if attr_name == "option_nullable" {
                let path = &mut attr.path.segments.last_mut().unwrap();
                path.ident = Ident::new("serde", Span::call_site());

                attr.tokens = "(default, deserialize_with = \"automate::encode::json::double_option\")".parse().unwrap();
            }
        }
    }
}