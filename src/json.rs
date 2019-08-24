use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Error, Debug};

/// A data structure that can be represented in a
/// JSON string.
///
/// Provides a faster way to serialize than serde,
/// thus the `.as_json()` method should be used when
/// performance is required.
/// To better exploit the performance of this trait, it
/// is recommended to use `.concat_json()` if the
/// destination string for the serialized data structure
/// already exists.
///
/// The code to derive this trait can be automatically
/// produced using `#[derive(AsJson)]` as the procedural
/// macro crate provides a derive macro for AsJson.
///
/// # Example
/// ```
/// #[macro_use] extern crate automatea_proc;
/// use automatea::AsJson;
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

/// A data structure that can be parsed from the value
/// of a JSON string.
///
/// Currently only implemented for primitive types such as
/// integers, floats and string.
///
/// # Example
/// ```
/// use automatea::FromJson;
///
/// assert_eq!(String::from_json("\"Hello\"").unwrap(), "Hello");
/// assert_eq!(u32::from_json("643789").unwrap(), 643789);
/// assert_eq!(i128::from_json("\"434233249862398463649324\"").unwrap(), 434233249862398463649324);
/// ```
pub trait FromJson {
    fn from_json(json: &str) -> Result<Self, JsonError> where Self: Sized;
}

/// Represents an error relative to a JSON string.
/// The error usually means that the provided JSON is
/// not a correctly formatted JSON string.
pub struct JsonError {
    pub msg: String,
}

impl JsonError {
    pub fn new<S>(msg: S) -> JsonError where S: Into<String> {
        JsonError { msg: msg.into() }
    }

    pub fn err<S, T>(msg: S) -> Result<T, JsonError> where S: Into<String> {
        Err(JsonError { msg: msg.into() })
    }
}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.msg)
    }
}

impl Debug for JsonError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{{ msg: {} }}", self.msg)
    }
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
                fn from_json(json: &str) -> Result<$ty, JsonError> {
                    <$ty>::from_str(json)
                        .map_err(|_| JsonError::new(format!("Failed to parse {} into {}", json, stringify!($ty))))
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
                fn from_json(json: &str) -> Result<$ty, JsonError> {
                    if json.len() >= 2 && json.chars().next().unwrap() == '"' && json.chars().last().unwrap() == '"' {
                        <$ty>::from_str(&json[1..json.len()-1])
                            .map_err(|_| JsonError::new(format!("Failed to parse {} into {}", json, stringify!($ty))))
                    } else {
                        JsonError::err("Incorrect JSON large number received")
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
    fn from_json(json: &str) -> Result<String, JsonError> {
        if json.len() >= 2 && json.chars().next().unwrap() == '"' && json.chars().last().unwrap() == '"' {
            Ok(String::from(&json[1..json.len() - 1]))
        } else {
            JsonError::err("Incorrect JSON string value received")
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
            json.push_str(&val.as_json());
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

/// Searches for a key through the root JSON object of the
/// candidate string and returns a parsed value.
///
/// The searched value can only be a string or a float/integer,
/// objects, vectors and maps won't work. Furthermore, only keys
/// in the root JSON object will be compared to your search. If,
/// for example, you search for a key "a" which is only present
/// in a nested JSON object, the function will fail.
///
/// Returns a JsonError if the candidate string is not a correct
/// JSON string or if the function failed to find the key and
/// its associated value in the string.
pub fn json_root_search<T>(key: &str, candidate: &str) -> Result<T, JsonError> where T: FromJson {
    if key.len() == 0 {
        return JsonError::err("The searched key can't be empty");
    }

    //get candidate slice starting at the first character of the value
    let value_begin = {
        let mut quoted_key = String::with_capacity(key.len() + 2);
        quoted_key.push('"');
        quoted_key.push_str(key);
        quoted_key.push('"');

        let mut iter = candidate.chars();
        let mut key_iter = quoted_key.chars();
        let mut prev_index = 0;
        let mut nesting_level = 0;
        let mut key_end = None;

        while key_end.is_none() {
            if let Some(next) = iter.next() {
                if next == '{' || next == '[' {
                    nesting_level += 1
                } else if next == '}' || next == ']' {
                    nesting_level -= 1
                }

                if nesting_level == 1 {
                    if let Some(key_next) = key_iter.next() {
                        if next != key_next {
                            key_iter = quoted_key.chars();
                        }
                    } else {
                        key_end = Some(prev_index + 1);
                        break;
                    }
                }

                prev_index += 1;
            } else {
                return JsonError::err("Could not find key in candidate");
            }
        }

        if key_end.is_none() {
            return JsonError::err("Could not find key in candidate");
        }

        match candidate[key_end.unwrap()..].find(|c: char| c.is_numeric() || c == '"') {
            Some(i) => &candidate[key_end.unwrap() + i..],
            None => return JsonError::err("Could not find value in candidate")
        }
    };

    let mut iter = value_begin.chars();
    let mut prev_index = 0;
    let mut nesting_level = 0;
    let mut value = None;

    while value.is_none() {
        if let Some(next) = iter.next() {
            if next == '{' || next == '[' {
                nesting_level += 1
            } else if next == '}' || next == ']' {
                nesting_level -= 1
            }

            if next == ',' || (nesting_level == -1 && next == '}') { //reached the end of the json value/string
                value = Some(&value_begin[..prev_index]);
            }

            prev_index += 1;
        } else {
            return JsonError::err("Unexpected end of string");
        }
    }

    if let Some(value) = value {
        return match T::from_json(value) {
            Ok(value) => Ok(value),
            Err(e) => JsonError::err(format!("Failed to parse JSON: {}", e))
        };
    }

    JsonError::err("An error occurred while searching for key")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_search() {
        let simple_str = r#"{"key":"value"}"#;
        let simple_int = r#"{"key":432}"#;
        let simple_float = r#"{"key":5.56}"#;

        assert_eq!(json_root_search::<String>("key", simple_str).unwrap(), "value");
        assert_eq!(json_root_search::<u32>("key", simple_int).unwrap(), 432);
        assert_eq!(json_root_search::<f32>("key", simple_float).unwrap(), 5.56);
    }

    #[test]
    fn test_root_search_vec() {
        let contains_array = r#"{"array":["a","b","c"],"b":40}"#;

        assert_eq!(json_root_search::<u8>("b", contains_array).unwrap(), 40);
    }

    #[test]
    fn test_root_search_fail() {
        let simple = r#"{"key":"value"}"#;

        assert_eq!(json_root_search::<String>("ke", simple).err().unwrap().msg, "Could not find key in candidate");
        assert_eq!(json_root_search::<String>("ey", simple).err().unwrap().msg, "Could not find key in candidate");
        assert_eq!(json_root_search::<String>("", simple).err().unwrap().msg, "The searched key can't be empty");
        assert_eq!(json_root_search::<String>("aaa", simple).err().unwrap().msg, "Could not find key in candidate");
        assert_eq!(json_root_search::<String>("value", simple).err().unwrap().msg, "Could not find key in candidate");
    }

    #[test]
    fn test_root_search_nested() {
        let nested_safe_order = r#"{"a":700,"nested":{"a":700}}"#;
        let nested_risky_order = r#"{"nested":{"a":1000},"a":700}"#;

        assert_eq!(json_root_search::<u16>("a", nested_safe_order).unwrap(), 700);
        assert_eq!(json_root_search::<u16>("a", nested_risky_order).unwrap(), 700);
    }

    #[test]
    fn test_search_invalid() {
        let no_final_brace = r#"{"key":"value""#;
        let no_comma = r#"{"key":"value""other":5}"#;
        let wrong_type = r#"{"int":"value"}"#;

        assert_eq!(json_root_search::<String>("key", no_final_brace).err().unwrap().msg,
                   "Unexpected end of string");
        assert_eq!(json_root_search::<String>("key", no_comma).err().unwrap().msg,
                   "Failed to parse JSON: Incorrect JSON string value received");
        assert_eq!(json_root_search::<u32>("int", wrong_type).err().unwrap().msg,
                   "Failed to parse JSON: Failed to parse \"value\" into u32");
    }
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use serde::Serialize;
    use test::Bencher;

    #[derive(Serialize, AsJson)]
    struct Something {
        somewhere: String,
        somehow: i32,
        someway: Vec<String>,
        somewhat: u64,
        someday: HashMap<String, String>,
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
                someday: map,
            }
        }
    }

    #[bench]
    fn bench_serde(b: &mut Bencher) {
        let something = Something::create();

        b.iter(|| {
            serde_json::to_string(&something).unwrap();
        });
    }

    #[bench]
    fn bench_automatea(b: &mut Bencher) {
        let something = Something::create();

        b.iter(|| {
            something.as_json();
        });
    }

    #[bench]
    fn bench_json_root_search(b: &mut Bencher) {
        b.iter(|| {
            json_root_search::<String>("second", r#"{"first":123,"second":"Hello","third":-78}"#).unwrap();
        });
    }
}