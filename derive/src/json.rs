use proc_macro2::Ident;
use syn::{DataStruct, Type};

pub type StructFields<'a> = ((Vec<&'a Ident>, Vec<String>), (Vec<&'a Ident>, Vec<String>), usize);

pub fn rename_field(field: &Ident) -> String {
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
pub fn extract_fields(data_struct: &DataStruct) -> StructFields {
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
