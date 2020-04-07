use proc_macro::{TokenStream, TokenTree};
use syn::{parse_macro_input, Ident, Expr, Data, DeriveInput};
use quote::quote;

pub fn convert(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let cloned_item = item.clone();

    let input: DeriveInput = parse_macro_input!(item);
    let struct_name: &Ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (as_method_name, convertion_type): (Ident, Ident) = match metadata.into_iter().next() {
        Some(TokenTree::Ident(ty)) => {
            let as_method = Ident::new(&format!("as_{}", ty.to_string()), ty.span().into());
            let ty = Ident::new(&ty.to_string(), ty.span().into());

            (as_method, ty)
        }
        _ => compile_error!("Expected arguments under the format (type)")
    };

    let mut fields_ident: Vec<&Ident> = Vec::new();
    let mut fields_expr: Vec<&Expr> = Vec::new();

    if let Data::Enum(en) = &input.data {
        for variant in &en.variants {
            if variant.discriminant.is_none() {
                compile_error!("Convert attribute only works with C-like enums")
            }

            let (_, expr) = variant.discriminant.as_ref().unwrap();

            fields_ident.push(&variant.ident);
            fields_expr.push(expr);
        }
    } else {
        compile_error!( "Convert attribute only works on enums")
    }

    let mut output: TokenStream = quote!(#[derive(Clone, Debug, ::serde_repr::Deserialize_repr)]#[repr(#convertion_type)]).into();
    output.extend(cloned_item);

    output.extend(TokenStream::from(quote! {
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
            #[cfg_attr(feature = "aggressive-inlining", inline)]
            fn as_json(&self) -> String {
                self.#as_method_name().to_string()
            }

            #[cfg_attr(feature = "aggressive-inlining", inline)]
            fn concat_json(&self, dest: &mut String) {
                ::std::fmt::Write::write_fmt(dest, format_args!("{}", self.#as_method_name())).expect("A Display implementation returned an error unexpectedly");
            }
        }
    }));

    output
}