#[macro_export]
macro_rules! deserialize {
    ($data:ident) => {
        ::automate::json::FromJson::from_json(&$data)
    };

    ($data:ident as $type:ty) => {
        <$type as ::automate::json::FromJson>::from_json(&$data)
    }
}

#[macro_export]
macro_rules! map {
    {$($key:expr => $val:expr),*} => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key.to_owned(), $val.to_owned());)*

        map
    }}
}

#[macro_export]
macro_rules! json {
    {} => {{
        String::from("{}")
    }};
    {$($key:expr => $val:expr),*} => {{
        let mut json = String::with_capacity(10);
        json.push('{');
        $(json.push('"');json.push_str($key); json.push_str("\":");$val.concat_json(&mut json);json.push(',');)*
        json.push('}');
        json
    }}
}
