use std::collections::HashMap;

pub trait AsJson {
    fn as_json(&self) -> String;
}

trait Zero {
    const ZERO: Self;
}

macro_rules! impl_num_as_json {
    ( $($ty:ty),* ) => (
        $(
            impl AsJson for $ty {
                fn as_json(&self) -> String {
                    self.to_string()
                }
            }
        )*
    )
}

macro_rules! impl_str_as_json {
    ( $($ty:ty),* ) => (
        $(
            impl AsJson for $ty {
                fn as_json(&self) -> String {
                    format!("\"{}\"", self)
                }
            }
        )*
    )
}

impl_num_as_json! {
    i8, i16, i32, isize,
    u8, u16, u32, usize,
    f32,
    bool
}

impl_str_as_json! {
    String, &str,
    i64, i128,
    u64, u128,
    f64
}

impl<J> AsJson for Vec<J> where J: AsJson {
    fn as_json(&self) -> String {
        let mut json = String::from("[");

        for val in self {
            json.push_str(&format!("{},", val.as_json()));
        }

        json.pop(); //remove last comma
        json.push(']');

        json
    }
}

impl<J, K> AsJson for HashMap<J, K> where J: AsJson, K: AsJson {
    fn as_json(&self) -> String {
        let mut json = String::from("{");

        for (key, val) in self {
            json.push_str(&format!("{}:{},", key.as_json(), val.as_json()));
        }

        json.pop(); //remove last comma
        json.push('}');

        json
    }
}