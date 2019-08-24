use std::collections::HashMap;
use std::str::FromStr;

/// A data structure that can be represented in a
/// JSON string.
///
/// Provides a significantly faster way to serialize
/// than serde, thus the `.as_json()` method should be
/// used when performance is required.
/// The code to derive this trait can be automatically
/// produced using `#[derive(AsJson)]` as the procedural
/// macro crate provides a derive macro for AsJson.
///
/// # Example
/// ```
/// #[macro_use] extern crate automate_proc;
/// use automate::AsJson;
///
/// #[derive(AsJson)]
/// struct File {
///     path: &'static str,
///     content: &'static str,
///     size: u32
/// }
///
/// let file = File {
///     path: "/path/to/file",
///     content: "Serialized with AsJson",
///     size: 22
/// };
///
/// assert_eq!(file.as_json(), r#"{"path":"/path/to/file","content":"Serialized with AsJson","size":22}"#)
/// ```
pub trait AsJson {
    fn as_json(&self) -> String;
    fn concat_json(&self, dest: &mut String);
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

                fn concat_json(&self, dest: &mut String) {
                    dest.push_str(&self.to_string());
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
                    let as_string = self.to_string();

                    let mut string = String::with_capacity(as_string.len() + 2);
                    string.push('"');
                    string.push_str(&as_string);
                    string.push('"');

                    string
                }

                fn concat_json(&self, dest: &mut String) {
                    let as_string = self.to_string();

                    dest.reserve(as_string.len() + 2);
                    dest.push('"');
                    dest.push_str(&as_string);
                    dest.push('"');
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
        let mut string = String::with_capacity(self.len() + 3);
        string.push('"');
        string.push_str(&self);
        string.push('"');

        string
    }

    fn concat_json(&self, dest: &mut String) {
        dest.reserve(self.len() + 3);
        dest.push('"');
        dest.push_str(&self);
        dest.push('"');
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
        let mut string = String::with_capacity(self.len() + 3);
        string.push('"');
        string.push_str(&self);
        string.push('"');

        string
    }

    fn concat_json(&self, dest: &mut String) {
        dest.reserve(self.len() + 3);
        dest.push('"');
        dest.push_str(&self);
        dest.push('"');
    }
}

impl<J> AsJson for Vec<J> where J: AsJson {
    fn as_json(&self) -> String {
        let mut json = String::from("[");

        for val in self {
            json.push_str(&val.as_json());
            json.push(',');
        }

        json.pop(); //remove last comma
        json.push(']');

        json
    }

    fn concat_json(&self, dest: &mut String) {
        dest.push('[');

        for val in self {
            val.concat_json(dest);
            dest.push(',');
        }

        dest.pop(); //remove last comma
        dest.push(']');
    }
}

impl<J, K> AsJson for HashMap<J, K> where J: AsJson, K: AsJson {
    fn as_json(&self) -> String {
        let mut json = String::from("{");

        for (key, val) in self {
            json.push_str(&key.as_json());
            json.push(':');
            json.push_str(&key.as_json());
            json.push(',');
        }

        json.pop(); //remove last comma
        json.push('}');

        json
    }

    fn concat_json(&self, dest: &mut String) {
        dest.push('{');

        for (key, val) in self {
            key.concat_json(dest);
            dest.push(':');
            val.concat_json(dest);
            dest.push(',');
        }

        dest.pop(); //remove last comma
        dest.push('}');
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use test::Bencher;

    #[derive(Serialize, AsJson)]
    struct Something {
        somewhere: String,
        somehow: i32,
        someway: Vec<String>,
        somewhat: u64,
        someday: HashMap<String, String>
    }

    impl Something {
        fn create() -> Something {
            let mut vec = Vec::new();
            vec.push(String::from("Hello"));
            vec.push(String::from("world"));
            vec.push(String::from("!"));

            let mut map = HashMap::new();
            map.insert(String::from("Hello"), String::from("olleH"));
            map.insert(String::from("world"), String::from("dlrow"));
            map.insert(String::from("!"), String::from("!"));

            Something {
                somewhere: String::from("Not here"),
                somehow: -37218,
                someway: vec,
                somewhat: 1936198231983251985,
                someday: map
            }
        }
    }

    #[bench]
    fn bench_serde(b: &mut Bencher) {
        let something = Something::create();

        b.iter(|| {
            serde_json::to_string(&something);
        });
    }

    #[bench]
    fn bench_automatea(b: &mut Bencher) {
        let something = Something::create();

        b.iter(|| {
            something.as_json();
        });
    }

}