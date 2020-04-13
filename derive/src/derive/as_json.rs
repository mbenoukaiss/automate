use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput, Data, Fields, FieldsUnnamed, Ident, DataStruct, Type, GenericArgument, PathArguments};
use quote::quote;

#[derive(Default)]
struct StructFields<'a> {
    fields: Vec<&'a Ident>,
    fields_names: Vec<String>,
    options: Vec<&'a Ident>,
    options_names: Vec<String>,
    nullables: Vec<&'a Ident>,
    nullables_names: Vec<String>,
    options_nullables: Vec<&'a Ident>,
    options_nullables_names: Vec<String>,
    recommended_size: usize
}


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
    let mut f = StructFields::default();

    for field in &data_struct.fields {
        let ident = field.ident.as_ref().expect("Expected ident for field");

        let mut nullable = false;

        for attr in &field.attrs {
            let attr_name = attr.path.segments.last().unwrap().ident.to_string();

            if attr_name == "nullable" {
                nullable = true;
            }
        }

        if let Type::Path(path) = &field.ty {
            let last = path.path.segments.last().unwrap();

            if last.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &last.arguments { //double option
                    //since we're in an option, we can safely unwrap the first and only type generic argument
                    if let GenericArgument::Type(Type::Path(path)) = args.args.first().unwrap() {
                        let last = path.path.segments.last().unwrap();

                        if last.ident == "Option" {
                            f.recommended_size += ident.to_string().len() / 2 + 5;
                            f.options_nullables.push(ident);
                            f.options_nullables_names.push(rename_field(ident));

                            continue;
                        }
                    }
                }

                //if we didn't make it to the continue, it's a simple option
                f.recommended_size += ident.to_string().len() / 2 + 5;
                f.options.push(ident);
                f.options_names.push(rename_field(ident));

                continue;
            }
        }

        if nullable {
            f.nullables.push(ident);
            f.nullables_names.push(rename_field(ident));
        } else {
            f.fields.push(ident);
            f.fields_names.push(rename_field(ident));
        }

        f.recommended_size += ident.to_string().len() + 5;
    }

    f
}

/// Creates the implementation of AsJson for structs using
/// the newtype pattern.
fn impl_newtype_pattern(input: &DeriveInput, unnamed: &FieldsUnnamed) -> TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if unnamed.unnamed.len() == 1 {
        TokenStream::from(quote! {
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
        })
    } else {
        compile_error!(unnamed, "Structs with multiple unnamed fields are not supported")
    }
}

fn as_json_fn(fields: &StructFields) -> TokenStream2 {
    let StructFields {
        fields,
        fields_names,
        options,
        options_names,
        nullables,
        nullables_names,
        options_nullables,
        options_nullables_names,
        recommended_size
    } = fields;

    quote! {
        #[cfg_attr(feature = "aggressive-inlining", inline)]
        fn as_json(&self) -> String {
            let mut json = String::with_capacity(#recommended_size);
            json.push('{');

            #(
             json.push_str(concat!("\"", #fields_names, "\":"));
             ::automate::encode::AsJson::concat_json(&self.#fields, &mut json);
             json.push(',');
            )*

            #(
             if let Some(optional) = &self.#options {
                 json.push_str(concat!("\"", #options_names, "\":"));
                 ::automate::encode::AsJson::concat_json(optional, &mut json);
                 json.push(',');
             }
            )*

            #(
             if let Some(nullable) = &self.#nullables {
                 json.push_str(concat!("\"", #nullables_names, "\":"));
                 ::automate::encode::AsJson::concat_json(nullable, &mut json);
                 json.push(',');
             } else {
                 json.push_str(concat!("\"", #nullables_names, "\":null,"));
             }
            )*

            #(
             if let Some(optional) = &self.#options_nullables {
                 if let Some(optional) = optional {
                     json.push_str(concat!("\"", #options_nullables_names, "\":"));
                     ::automate::encode::AsJson::concat_json(optional, &mut json);
                     json.push(',');
                 } else { // Some(None) means null
                     json.push_str(concat!("\"", #options_nullables_names, "\":null,"));
                 }
             }
            )*

            if json.len() > 1 {
                json.pop(); //remove last comma
            }

            json.push('}');

            json
        }
    }
}

fn concat_json_fn(fields: &StructFields) -> TokenStream2 {
    let StructFields {
        fields,
        fields_names,
        options,
        options_names,
        nullables,
        nullables_names,
        options_nullables,
        options_nullables_names,
        recommended_size
    } = fields;

    quote! {
        #[cfg_attr(feature = "aggressive-inlining", inline)]
        fn concat_json(&self, dest: &mut String) {
            dest.reserve(#recommended_size);
            let original_len = dest.len();
            dest.push('{');

            #(
             dest.push_str(concat!("\"", #fields_names, "\":"));
             ::automate::encode::AsJson::concat_json(&self.#fields, dest);
             dest.push(',');
            )*

            #(
             if let Some(optional) = &self.#options {
                 dest.push_str(concat!("\"", #options_names, "\":"));
                 ::automate::encode::AsJson::concat_json(optional, dest);
                 dest.push(',');
             }
            )*

            #(
             if let Some(nullable) = &self.#nullables {
                 dest.push_str(concat!("\"", #nullables_names, "\":"));
                 ::automate::encode::AsJson::concat_json(nullable, dest);
                 dest.push(',');
             } else {
                 dest.push_str(concat!("\"", #nullables_names, "\":null,"));
             }
            )*

            #(
             if let Some(optional) = &self.#options_nullables {
                 if let Some(optional) = optional {
                     dest.push_str(concat!("\"", #options_nullables_names, "\":"));
                     ::automate::encode::AsJson::concat_json(optional, dest);
                     dest.push(',');
                 } else { // Some(None) means null
                     dest.push_str(concat!("\"", #options_nullables_names, "\":null,"));
                 }
             }
            )*

            if dest.len() > original_len + 1 {
                dest.pop(); //remove last comma
            }

            dest.push('}');
        }
    }
}

pub fn as_json(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item);

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Unnamed(unnamed) = &data_struct.fields {
            return impl_newtype_pattern(&input, unnamed);
        }

        let name = &input.ident;
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        let fields = extract_fields(data_struct);
        let as_json = as_json_fn(&fields);
        let concat_json = concat_json_fn(&fields);

        TokenStream::from(quote! {
            impl #impl_generics ::automate::encode::AsJson for #name #ty_generics #where_clause {
                #as_json
                #concat_json
            }
        })
    } else {
        compile_error!(input, "AsJson can only be applied to structs")
    }
}