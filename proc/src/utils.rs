use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields};

pub fn extend_with_deref(input: &DeriveInput, quote: &mut TokenStream) {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();


    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Unnamed(unnamed) = &data_struct.fields {
            if unnamed.unnamed.len() == 1 {
                let underlying = &unnamed.unnamed.first().unwrap().ty;

                let deref = quote! {
                    impl #impl_generics ::std::ops::Deref for #name #ty_generics #where_clause {
                        type Target = #underlying;

                        fn deref(&self) -> &Self::Target {
                            &self.0
                        }
                    }

                    impl #impl_generics ::std::ops::DerefMut for #name #ty_generics #where_clause {

                        fn deref_mut(&mut self) -> &mut Self::Target {
                            &mut self.0
                        }
                    }
                };

                quote.extend(TokenStream::from(deref));
            } else {
                //TODO: raise warning or idk
            }
        }
    }
}