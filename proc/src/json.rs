use proc_macro2::Ident;
use syn::{DataStruct, Data, Type};

pub fn rename_field(field: &Ident) -> String {
    let name = field.to_string();

    if name.chars().next().unwrap() == '_' {
        (&name[1..]).to_owned()
    } else {
        name
    }
}

pub fn extract_fields(data_struct: &DataStruct) -> (Vec<&Ident>, Vec<String>, Vec<&Ident>, Vec<String>, usize) {
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

    return (fields, fields_names, options, options_names, recommended_size);
}