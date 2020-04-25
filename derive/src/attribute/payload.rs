use proc_macro2::{TokenStream as TokenStream2, Span};
use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, Attribute, Field, Fields, Ident, ItemStruct};
use syn::parse::Parser;
use quote::{quote, ToTokens};
use darling::FromMeta;
use crate::utils;
use crate::utils::StructSide;

/// Parses the list of variables for a gateway payload
///   `#[payload(op = 0, event = "READY", server)]`
///   `#[payload(op = 8, client)]`
#[derive(FromMeta)]
struct Args {
    op: u8,
    #[darling(default)]
    event: Option<String>,
    #[darling(default)]
    default: bool,

    #[darling(default)]
    client: bool,
    #[darling(default)]
    server: bool,
    #[darling(default)]
    both: bool,
}

impl Args {
    fn side(&self) -> Result<StructSide, TokenStream> {
        if self.client {
            Ok(StructSide::Client)
        } else if self.server {
            Ok(StructSide::Server)
        } else if self.both {
            Ok(StructSide::Both)
        } else {
            compile_error!(err "Expected side in payload attribute")
        }
    }
}

pub fn payload(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let args: AttributeArgs = parse_macro_input!(metadata);
    let args: Args = match Args::from_list(&args) {
        Ok(v) => v,
        Err(e) => { return e.write_errors().into(); }
    };

    let side: StructSide = unwrap!(args.side());

    let mut item: ItemStruct = parse_macro_input!(item);
    utils::replace_attributes(&mut item);

    let struct_name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    let trace_struct = if cfg!(feature = "strict-deserializer") {
        if let Fields::Named(mut fields) = item.fields.clone() {
            let derives = side.appropriate_derive(args.default);
            let trace_name = format!("__trace_{}", item.ident);
            let trace_ident = Ident::new(&trace_name, Span::call_site());

            //retrieve field names to use in the From<>
            let field_names = fields.named.iter()
                .map(|f| f.ident.clone())
                .collect::<Option<Vec<Ident>>>();

            //add the extra _trace field
            fields.named.push(Field::parse_named
                .parse2(quote!(_trace: Option<Vec<String>>))
                .unwrap());

            //create a similar struct with the additional `_trace` to allow deserializing
            //with the strict deserializer without the presence of `_trace` returning err
            if let Some(field_names) = field_names {
                let attrs = Attribute::parse_outer
                    .parse2(quote!(#[serde(from = #trace_name)]))
                    .unwrap();

                item.attrs.extend(attrs);

                quote! {
                    #derives
                    struct #trace_ident #ty_generics #fields

                    impl #impl_generics core::convert::From<#trace_ident #ty_generics> for #struct_name #ty_generics {
                        fn from(trace: #trace_ident #ty_generics) -> Self {
                            #struct_name #ty_generics {
                                #(#field_names: trace.#field_names),*
                            }
                        }
                    }
                }
            } else {
                TokenStream2::new()
            }
        } else {
            TokenStream2::new()
        }
    } else {
        TokenStream2::new()
    };

    let mut output = side.appropriate_derive(args.default);
    item.to_tokens(&mut output);
    output.extend(trace_struct);

    if let Some(event_name) = args.event {
        output.extend(quote! {
            impl #impl_generics #struct_name #ty_generics #where_clause {
                pub const EVENT_NAME: &'static str = #event_name;
            }
        });
    }

    unwrap!(utils::extend_with_deref(&item, &mut output));

    if let StructSide::Client = side {
        utils::append_client_quote(&item, args.op, &mut output);
    } else if let StructSide::Server = side {
        utils::append_server_quote(&item, &mut output);
    } else {
        utils::append_client_quote(&item, args.op, &mut output);
        utils::append_server_quote(&item, &mut output);
    }

    TokenStream::from(output)
}