#[macro_export]
macro_rules! deserialize {
    ($data:ident) => {
        ::automatea::json::FromJson::from_json(&$data)
    };

    ($data:ident as $type:ty) => {
        <$type as ::automatea::json::FromJson>::from_json(&$data)
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
