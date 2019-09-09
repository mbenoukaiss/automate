use proc_macro2::Ident;
use syn::{DeriveInput, Data, Type};

pub fn extract_fields(input: &DeriveInput) -> (Vec<&Ident>, Vec<&Ident>, usize) {
    if let Data::Struct(data_struct) = &input.data {
        let mut recommended_size = 0;
        let mut fields = Vec::new();
        let mut options = Vec::new();

        for field in &data_struct.fields {
            let ident = field.ident.as_ref().expect("Expected ident for field");

            if let Type::Path(path) = &field.ty {
                if path.path.segments.len() == 1 && path.path.segments.first().unwrap().ident == "Option" {
                    recommended_size += ident.to_string().len() / 2 + 5;
                    options.push(ident);
                    continue;
                }
            }

            recommended_size += ident.to_string().len() + 5;

            fields.push(ident);
        }

        return (fields, options, recommended_size);
    } else {
        panic!("AsJson can only be applied to structs"); //Expected struct for proc macros 'object' and 'payload'
    }
}