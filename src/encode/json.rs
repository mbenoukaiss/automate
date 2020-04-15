use std::fmt::{Display, Formatter, Error, Debug};
use serde::{Deserialize, Deserializer};

/// A value that has to be included in the JSON
/// string but can contain the value null.
///
/// Some values in discord can be omitted from
/// the JSON string, these are represented as
/// Option<T>. Some other values must be in the
/// JSON string but may contain the null value
/// and should be represented using Option<Option<T>>.
pub fn double_option<'de, T, D>(de: D) -> Result<Option<Option<T>>, D::Error>
    where T: Deserialize<'de>,
          D: Deserializer<'de>
{
    Deserialize::deserialize(de).map(Some)
}

/// Represents an error relative to a JSON string.
/// The error usually means that the provided JSON is
/// not a correctly formatted JSON string.
pub struct JsonError {
    pub msg: String,

    #[cfg(feature = "backtrace")]
    pub backtrace: String,
}

impl JsonError {
    pub fn new<S>(msg: S) -> JsonError where S: Into<String> {
        JsonError {
            msg: msg.into(),

            #[cfg(feature = "backtrace")]
            backtrace: format!("{:#?}", backtrace::Backtrace::new()),
        }
    }

    pub fn err<S, T>(msg: S) -> Result<T, JsonError> where S: Into<String> {
        Err(JsonError {
            msg: msg.into(),

            #[cfg(feature = "backtrace")]
            backtrace: format!("{:#?}", backtrace::Backtrace::new()),
        })
    }
}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        #[cfg(not(feature = "backtrace"))] return write!(f, "{}", self.msg);
        #[cfg(feature = "backtrace")] return write!(f, "{}\n{}", self.msg, self.backtrace);
    }
}

impl Debug for JsonError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        #[cfg(not(feature = "backtrace"))] return write!(f, "{{ msg: {} }}", self.msg);
        #[cfg(feature = "backtrace")] return write!(f, "{{ msg: {}, backtrace: {} }}", self.msg, self.backtrace);
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
pub fn root_search<'de, T>(key: &str, candidate: &'de str) -> Result<T, JsonError> where T: Deserialize<'de> {
    if key.is_empty() {
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
            if nesting_level < 0 {
                return JsonError::err("Incorrectly formatted JSON string");
            }

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
        return match serde_json::from_str(value) {
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

        assert_eq!(root_search::<String>("key", simple_str).unwrap(), "value");
        assert_eq!(root_search::<u32>("key", simple_int).unwrap(), 432);
        assert_eq!(root_search::<f32>("key", simple_float).unwrap(), 5.56);
    }

    #[test]
    fn test_root_search_key_not_found() {
        let simple = r#"{"key":"value"}"#;

        assert_eq!(root_search::<String>("ke", simple).err().unwrap().msg, "Could not find key in candidate");
        assert_eq!(root_search::<String>("ey", simple).err().unwrap().msg, "Could not find key in candidate");
        assert_eq!(root_search::<String>("aaa", simple).err().unwrap().msg, "Could not find key in candidate");
        assert_eq!(root_search::<String>("value", simple).err().unwrap().msg, "Could not find key in candidate");
    }

    #[test]
    fn test_root_search_empty_key() {
        let simple = r#"{"key":"value"}"#;

        assert_eq!(root_search::<String>("", simple).err().unwrap().msg, "The searched key can't be empty");
    }

    #[test]
    fn test_root_search_nested_vec() {
        let contains_array = r#"{"array":["a","b","c"],"b":40}"#;

        assert_eq!(root_search::<u8>("b", contains_array).unwrap(), 40);
        assert_eq!(root_search::<String>("c", contains_array).err().unwrap().msg, "Could not find key in candidate");
    }

    #[test]
    fn test_root_search_nested_object() {
        let nested_safe_order = r#"{"a":700,"nested":{"a":700}}"#;
        let nested_risky_order = r#"{"nested":{"a":1000},"a":700}"#;

        assert_eq!(root_search::<u16>("a", nested_safe_order).unwrap(), 700);
        assert_eq!(root_search::<u16>("a", nested_risky_order).unwrap(), 700);
    }

    #[test]
    fn test_search_invalid_json() {
        let no_first_brace = r#""key":"value"}"#;
        let no_final_brace = r#"{"key":"value""#;
        let no_comma = r#"{"key":"value""other":5}"#;
        let no_key_quote = r#"{"key:"value","other":5}"#;
        let no_val_quote = r#"{"key":value","other":5}"#;
        let wrong_type = r#"{"int":"value"}"#;

        assert!(root_search::<String>("key", no_first_brace).is_err());
        assert!(root_search::<String>("key", no_final_brace).is_err());
        assert!(root_search::<String>("key", no_comma).is_err());
        assert!(root_search::<String>("key", no_key_quote).is_err());
        assert!(root_search::<String>("key", no_val_quote).is_err());
        assert!(root_search::<u32>("int", wrong_type).is_err());
    }

}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use serde::Serialize;
    use test::Bencher;

    #[derive(Serialize)]
    struct Something {
        somewhere: String,
        somehow: i32,
        someway: Vec<String>,
        somewhat: String,
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
                somewhat: String::from("1936198231983251985"),
                someday: map,
            }
        }
    }

    #[derive(Serialize)]
    struct LongStr {
        first: String,
        second: String,
    }

    impl LongStr {
        fn create() -> LongStr {
            let mut first = String::new();
            let mut second = String::new();

            for _ in 0..500 {
                first.push_str("abc");
                second.push_str("d");
            }

            LongStr {
                first,
                second,
            }
        }
    }

    #[bench]
    fn bench_root_search_average(b: &mut Bencher) {
        let something = serde_json::to_string(&Something::create()).unwrap();

        b.iter(|| {
            root_search::<String>("somewhere", &something).unwrap();
        });
    }

    #[bench]
    fn bench_root_search_long_str(b: &mut Bencher) {
        let long_str = serde_json::to_string(&LongStr::create()).unwrap();

        b.iter(|| {
            root_search::<String>("second", &long_str).unwrap();
        });
    }
}