use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Ident, DataStruct, Type};
use quote::quote;

pub type StructFields<'a> = ((Vec<&'a Ident>, Vec<String>), (Vec<&'a Ident>, Vec<String>), usize);

fn rename_field(field: &Ident) -> String {
    let name = field.to_string();

    if name.starts_with('_') {
        (&name[1..]).to_owned()
    } else {
        name
    }
}

/// Retrieves all the fields in the given struct and
/// creates two pairs of two vectors associating the
/// identifier of the field with its name in the
/// json string, for both normal fields and options.
/// The last item of the tuple is a recommended minimum
/// size for the string buffer when serializing.
fn extract_fields(data_struct: &DataStruct) -> StructFields {
    let mut fields = Vec::new();
    let mut fields_names = Vec::new();
    let mut options = Vec::new();
    let mut options_names = Vec::new();
    let mut recommended_size = 0;

    for field in &data_struct.fields {
        let ident = field.ident.as_ref().expect("Expected ident for field");

        if let Type::Path(path) = &field.ty {
            if path.path.segments.len() == 1 && path.path.segments.first().unwrap().ident == "Option" {
                recommended_size += ident.to_string().len() / 2 + 5;
                options.push(ident);
                options_names.push(rename_field(ident));

                continue;
            }
        }

        recommended_size += ident.to_string().len() + 5;
        fields.push(ident);
        fields_names.push(rename_field(ident));
    }

    ((fields, fields_names), (options, options_names), recommended_size)
}

pub fn as_json(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item);

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Unnamed(unnamed) = &data_struct.fields {
            if unnamed.unnamed.len() == 1 {
                return TokenStream::from(quote! {
                    impl #impl_generics ::automate::encode::AsJson for #name #ty_generics #where_clause {
                        #[cfg_attr(feature = "aggressive-inlining", inline)]
                        fn as_json(&self) -> String {
                            ::automate::encode::AsJson::as_json(&self.0)
                        }

                        #[cfg_attr(feature = "aggressive-inlining", inline)]
                        fn concat_json(&self, dest: &mut String) {
                            ::automate::encode::AsJson::concat_json(&self.0, dest)
                        }
                    }
                });
            } else {
                compile_error!(unnamed, "Structs with multiple unnamed fields are not supported")
            }
        }

        let ((fs, fns), (os, ons), recommended_size) = extract_fields(data_struct);

        TokenStream::from(quote! {
            impl #impl_generics ::automate::encode::AsJson for #name #ty_generics #where_clause {
                #[cfg_attr(feature = "aggressive-inlining", inline)]
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

                #[cfg_attr(feature = "aggressive-inlining", inline)]
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
        })
    } else {
        compile_error!(input, "AsJson can only be applied to structs")
    }
}