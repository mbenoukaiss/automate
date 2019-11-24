/// Creates a hashmap associating the
/// given keys to the given values.
#[macro_export]
macro_rules! map {
    {$($key:expr => $val:expr),*} => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key.to_owned(), $val.to_owned());)*

        map
    }}
}

/// Creates a JSON string associating
/// the given keys with the given values.
#[macro_export]
macro_rules! json {
    {} => {{
        String::from("{}")
    }};
    {$($fkey:expr => $fval:expr, $($key:expr => $val:expr),*)?} => {{
        let mut json = String::with_capacity(10);

        json.push('{');
        $(
            json.push('"');json.push_str($fkey); json.push_str("\":");$fval.concat_json(&mut json);
            $(json.push(',');json.push('"');json.push_str($key); json.push_str("\":");$val.concat_json(&mut json);)*
        )?
        json.push('}');

        json
    }}
}
