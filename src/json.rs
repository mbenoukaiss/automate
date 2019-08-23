use std::collections::HashMap;
use std::str::FromStr;

pub trait AsJson {
    fn as_json(&self) -> String;
}

pub trait FromJson {
    fn from_json(json: &str) -> Result<Self, String> where Self: Sized;
}

macro_rules! impl_json_for_num {
    ( $($ty:ty),* ) => (
        $(
            impl AsJson for $ty {
                fn as_json(&self) -> String {
                    self.to_string()
                }
            }

            impl FromJson for $ty {
                fn from_json(json: &str) -> Result<$ty, String> {
                    <$ty>::from_str(json)
                        .map_err(|_| format!("Failed to parse {} into {}", json, stringify!($ty)))
                }
            }
        )*
    )
}

macro_rules! impl_json_for_large_num {
    ( $($ty:ty),* ) => (
        $(
            impl AsJson for $ty {
                fn as_json(&self) -> String {
                    format!("\"{}\"", self)
                }
            }

            impl FromJson for $ty {
                fn from_json(json: &str) -> Result<$ty, String> {
                    if json.len() >= 2 && json.chars().next().unwrap() == '"' && json.chars().last().unwrap() == '"' {
                        <$ty>::from_str(&json[1..json.len()-1])
                            .map_err(|_| format!("Failed to parse {} into {}", json, stringify!($ty)))
                    } else {
                        Err("Incorrect JSON large number received".to_owned())
                    }
                }
            }
        )*
    )
}

impl_json_for_num! {
    i8, i16, i32, isize,
    u8, u16, u32, usize,
    f32,
    bool
}

impl_json_for_large_num! {
    i64, i128,
    u64, u128,
    f64
}

impl AsJson for String {
    fn as_json(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl FromJson for String {
    fn from_json(json: &str) -> Result<String, String> {
        if json.len() >= 2 && json.chars().next().unwrap() == '"' && json.chars().last().unwrap() == '"' {
            Ok(String::from(&json[1..json.len() - 1]))
        } else {
            Err("Incorrect JSON string value received".to_owned())
        }
    }
}

impl AsJson for &str {
    fn as_json(&self) -> String {
        format!("\"{}\"", self)
    }
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