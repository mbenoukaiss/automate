use std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet};
use std::fmt::{Display, Formatter, Error, Debug, Write};
use std::hash::{BuildHasher};
use serde::{Deserialize, Deserializer};

/// A data structure that can be represented in a
/// JSON string.
///
/// The code to derive this trait can be automatically
/// produced using `#[derive(AsJson)]` as the procedural
/// macro crate provides a derive macro for AsJson.
///
/// # Example
/// ```
/// #[macro_use] extern crate automate_derive;
/// use automate::encode::AsJson;
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

/// Implements [AsJson](automate::AsJson) for simple
/// numeric and other types that should be represented
/// as is in the JSON string.
macro_rules! impl_for_num {
    ($($ty:ty),*) => (
        $(
            impl AsJson for $ty {
                #[cfg_attr(feature = "aggressive-inlining", inline)]
                fn as_json(&self) -> String {
                    self.to_string()
                }

                #[cfg_attr(feature = "aggressive-inlining", inline)]
                fn concat_json(&self, dest: &mut String) {
                    dest.write_fmt(format_args!("{}", self)).expect("A Display implementation returned an error unexpectedly");
                }
            }
        )*
    )
}

/// Implements [AsJson](automate::AsJson) for collections.
macro_rules! impl_for_single_collection {
    ($($ty:ident:$insert_method:ident <$($rq_trait:ident),*> ),*) => {
        $(
            impl<J> AsJson for $ty<J> where J: AsJson {
                #[cfg_attr(feature = "aggressive-inlining", inline)]
                fn as_json(&self) -> String {
                    let mut json = String::with_capacity(self.len() * 5 + 2);
                    json.push('[');

                    if !self.is_empty() {
                        for val in self {
                            val.concat_json(&mut json);
                            json.push(',');
                        }

                        json.pop(); //remove last comma
                    }

                    json.push(']');

                    json
                }

                #[cfg_attr(feature = "aggressive-inlining", inline)]
                fn concat_json(&self, dest: &mut String) {
                    dest.reserve(self.len() * 5 + 2);
                    dest.push('[');

                    if !self.is_empty() {
                        for val in self {
                            val.concat_json(dest);
                            dest.push(',');
                        }

                        dest.pop(); //remove last comma
                    }

                    dest.push(']');
                }
            }
        )*
    };
}

/// Implements [AsJson](automate::AsJson) for collections.
macro_rules! impl_for_associative_collection {
    ($($ty:ident <$($rq_first_trait:ident),*> ),*) => {
        $(
            impl<J> AsJson for $ty<String, J> where J: AsJson {
                #[cfg_attr(feature = "aggressive-inlining", inline)]
                fn as_json(&self) -> String {
                    let mut json = String::with_capacity(self.len() * 10 + 2);
                    json.push('{');

                    if !self.is_empty() {
                        for (key, val) in self {
                            key.concat_json(&mut json);
                            json.push(':');
                            val.concat_json(&mut json);
                            json.push(',');
                        }

                        json.pop(); //remove last comma
                    }

                    json.push('}');

                    json
                }

                #[cfg_attr(feature = "aggressive-inlining", inline)]
                fn concat_json(&self, dest: &mut String) {
                    dest.reserve(self.len() * 10 + 2);
                    dest.push('{');

                    if !self.is_empty() {
                        for (key, val) in self {
                            key.concat_json(dest);
                            dest.push(':');
                            val.concat_json(dest);
                            dest.push(',');
                        }

                        dest.pop(); //remove last comma
                    }

                    dest.push('}');
                }
            }
        )*
    };
}

macro_rules! impl_for_arrays {
    ($($size:tt),*) => (
        $(
            impl<T> AsJson for [T; $size] where T: AsJson {
                #[cfg_attr(feature = "aggressive-inlining", inline)]
                fn as_json(&self) -> String {
                    let mut json = String::with_capacity($size * 2 + 2);
                    json.push('[');

                    if !self.is_empty() {
                        for val in self {
                            val.concat_json(&mut json);
                            json.push(',');
                        }

                        json.pop();
                    }

                    json.push(']');

                    json
                }

                #[cfg_attr(feature = "aggressive-inlining", inline)]
                fn concat_json(&self, dest: &mut String) {
                    dest.reserve($size * 2 + 2);
                    dest.push('[');

                    if !self.is_empty() {
                        for val in self {
                            val.concat_json(dest);
                            dest.push(',');
                        }

                        dest.pop();
                    }

                    dest.push(']');
                }
            }
        )*
    )
}

impl_for_num! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    bool
}

impl_for_single_collection! {
    Vec:push <>, VecDeque:push_back <>,
    LinkedList:push_back <>, BTreeSet:insert <Ord>
}

impl_for_associative_collection! {
    BTreeMap<>
}

impl_for_arrays! {
     1,  2,  3,  4,  5,  6,  7,  8,
     9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32
}

impl AsJson for () {
    #[cfg_attr(feature = "aggressive-inlining", inline)]
    fn as_json(&self) -> String {
        String::from("")
    }

    #[cfg_attr(feature = "aggressive-inlining", inline)]
    fn concat_json(&self, _: &mut String) {

    }
}

impl<T> AsJson for Option<T> where T: AsJson {
    fn as_json(&self) -> String {
        match self {
            Some(val) => val.as_json(),
            None => String::from("null")
        }
    }

    fn concat_json(&self, dest: &mut String) {
        match self {
            Some(val) => val.concat_json(dest),
            None => dest.push_str("null")
        }
    }
}

impl AsJson for String {
    #[cfg_attr(feature = "aggressive-inlining", inline)]
    fn as_json(&self) -> String {
        let mut string = String::with_capacity(self.len() + 2);
        string.push('"');
        string.push_str(&self);
        string.push('"');

        string
    }

    #[cfg_attr(feature = "aggressive-inlining", inline)]
    fn concat_json(&self, dest: &mut String) {
        dest.reserve(self.len() + 2);
        dest.push('"');
        dest.push_str(&self);
        dest.push('"');
    }
}

impl AsJson for &str {
    #[cfg_attr(feature = "aggressive-inlining", inline)]
    fn as_json(&self) -> String {
        let mut string = String::with_capacity(self.len() + 3);
        string.push('"');
        string.push_str(&self);
        string.push('"');

        string
    }

    #[cfg_attr(feature = "aggressive-inlining", inline)]
    fn concat_json(&self, dest: &mut String) {
        dest.reserve(self.len() + 3);
        dest.push('"');
        dest.push_str(&self);
        dest.push('"');
    }
}

impl<J, S: BuildHasher> AsJson for HashSet<J, S> where J: AsJson {
    #[cfg_attr(feature = "aggressive-inlining", inline)]
    fn as_json(&self) -> String {
        let mut json = String::with_capacity(self.len() * 5 + 2);
        json.push('[');

        if !self.is_empty() {
            for val in self {
                val.concat_json(&mut json);
                json.push(',');
            }

            json.pop(); //remove last comma
        }

        json.push(']');

        json
    }

    #[cfg_attr(feature = "aggressive-inlining", inline)]
    fn concat_json(&self, dest: &mut String) {
        dest.reserve(self.len() * 5 + 2);
        dest.push('[');

        if !self.is_empty() {
            for val in self {
                val.concat_json(dest);
                dest.push(',');
            }

            dest.pop(); //remove last comma
        }

        dest.push(']');
    }
}

impl<J, S: BuildHasher> AsJson for HashMap<String, J, S> where J: AsJson {
    #[cfg_attr(feature = "aggressive-inlining", inline)]
    fn as_json(&self) -> String {
        let mut json = String::with_capacity(self.len() * 10 + 2);
        json.push('{');

        if !self.is_empty() {
            for (key, val) in self {
                key.concat_json(&mut json);
                json.push(':');
                val.concat_json(&mut json);
                json.push(',');
            }

            json.pop(); //remove last comma
        }

        json.push('}');

        json
    }

    #[cfg_attr(feature = "aggressive-inlining", inline)]
    fn concat_json(&self, dest: &mut String) {
        dest.reserve(self.len() * 10 + 2);
        dest.push('{');

        if !self.is_empty() {
            for (key, val) in self {
                key.concat_json(dest);
                dest.push(':');
                val.concat_json(dest);
                dest.push(',');
            }

            dest.pop(); //remove last comma
        }

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

    #[derive(Serialize, AsJson)]
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

    #[derive(Serialize, AsJson)]
    struct Short {
        a: u8,
        b: u16,
        c: u32,
    }

    impl Short {
        fn create() -> Short {
            Short {
                a: 0,
                b: 1,
                c: 2,
            }
        }
    }

    #[derive(Serialize, AsJson)]
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

    #[derive(Serialize, AsJson)]
    struct Options {
        first: Option<String>,
        second: Option<u32>,
        third: Option<String>,
        fourth: Option<u8>,
    }

    impl Options {
        fn all_some() -> Options {
            Options {
                first: Some(String::from("I am some!")),
                second: Some(37),
                third: Some(String::from("Me too...")),
                fourth: Some(250),
            }
        }

        fn half_some() -> Options {
            Options {
                first: Some(String::from("I am some!")),
                second: Some(37),
                third: None,
                fourth: None,
            }
        }

        fn no_some() -> Options {
            Options {
                first: None,
                second: None,
                third: None,
                fourth: None,
            }
        }
    }

    #[bench]
    fn bench_root_search_average(b: &mut Bencher) {
        let something = Something::create().as_json();

        b.iter(|| {
            root_search::<String>("somewhere", &something).unwrap();
        });
    }

    #[bench]
    fn bench_root_search_long_str(b: &mut Bencher) {
        let long_str = LongStr::create().as_json();

        b.iter(|| {
            root_search::<String>("second", &long_str).unwrap();
        });
    }

    #[bench]
    fn bench_serializer_average_automate(b: &mut Bencher) {
        let something = Something::create();

        b.iter(|| {
            something.as_json();
        });
    }

    #[bench]
    fn bench_serializer_average_serde(b: &mut Bencher) {
        let something = Something::create();

        b.iter(|| {
            serde_json::to_string(&something).unwrap();
        });
    }

    #[bench]
    fn bench_serializer_short_automate(b: &mut Bencher) {
        let short = Short::create();

        b.iter(|| {
            short.as_json();
        });
    }

    #[bench]
    fn bench_serializer_short_serde(b: &mut Bencher) {
        let short = Short::create();

        b.iter(|| {
            serde_json::to_string(&short).unwrap();
        });
    }

    #[bench]
    fn bench_serializer_long_string_automate(b: &mut Bencher) {
        let long_str = LongStr::create();

        b.iter(|| {
            long_str.as_json();
        });
    }

    #[bench]
    fn bench_serializer_long_string_serde(b: &mut Bencher) {
        let long_str = LongStr::create();

        b.iter(|| {
            serde_json::to_string(&long_str).unwrap();
        });
    }

    #[bench]
    fn bench_serializer_empty_hashmap_automate(b: &mut Bencher) {
        let map: HashMap<String, String> = HashMap::new();

        b.iter(|| {
            map.as_json();
        });
    }

    #[bench]
    fn bench_serializer_empty_hashmap_serde(b: &mut Bencher) {
        let map: HashMap<String, String> = HashMap::new();

        b.iter(|| {
            serde_json::to_string(&map).unwrap();
        });
    }

    #[bench]
    fn bench_serializer_full_hashmap_automate(b: &mut Bencher) {
        let mut map: HashMap<String, String> = HashMap::new();
        for _ in 0..500 {
            map.insert(String::from("hello"), String::from("world"));
        }

        b.iter(|| {
            map.as_json();
        });
    }

    #[bench]
    fn bench_serializer_full_hashmap_serde(b: &mut Bencher) {
        let mut map: HashMap<String, String> = HashMap::new();
        for _ in 0..500 {
            map.insert(String::from("hello"), String::from("world"));
        }

        b.iter(|| {
            serde_json::to_string(&map).unwrap();
        });
    }

    #[bench]
    fn bench_serializer_empty_vec_automate(b: &mut Bencher) {
        let vec: Vec<String> = Vec::new();

        b.iter(|| {
            vec.as_json();
        });
    }

    #[bench]
    fn bench_serializer_empty_vec_serde(b: &mut Bencher) {
        let vec: Vec<String> = Vec::new();

        b.iter(|| {
            serde_json::to_string(&vec).unwrap();
        });
    }

    #[bench]
    fn bench_serializer_full_vec_automate(b: &mut Bencher) {
        let mut vec: Vec<String> = Vec::new();
        for _ in 0..500 {
            vec.push(String::from("hello world"));
        }

        b.iter(|| {
            vec.as_json();
        });
    }

    #[bench]
    fn bench_serializer_full_vec_serde(b: &mut Bencher) {
        let mut vec: Vec<String> = Vec::new();
        for _ in 0..500 {
            vec.push(String::from("hello world"));
        }

        b.iter(|| {
            serde_json::to_string(&vec).unwrap();
        });
    }

    #[bench]
    fn bench_serializer_options_all_some_automate(b: &mut Bencher) {
        let options = Options::all_some();

        b.iter(|| {
            options.as_json();
        });
    }

    #[bench]
    fn bench_serializer_options_all_some_serde(b: &mut Bencher) {
        let options = Options::all_some();

        b.iter(|| {
            serde_json::to_string(&options).unwrap();
        });
    }

    #[bench]
    fn bench_serializer_options_half_some_automate(b: &mut Bencher) {
        let options = Options::half_some();

        b.iter(|| {
            options.as_json();
        });
    }

    #[bench]
    fn bench_serializer_options_half_some_serde(b: &mut Bencher) {
        let options = Options::half_some();

        b.iter(|| {
            serde_json::to_string(&options).unwrap();
        });
    }

    #[bench]
    fn bench_serializer_options_no_some_automate(b: &mut Bencher) {
        let options = Options::no_some();

        b.iter(|| {
            options.as_json();
        });
    }

    #[bench]
    fn bench_serializer_options_no_some_serde(b: &mut Bencher) {
        let options = Options::no_some();

        b.iter(|| {
            serde_json::to_string(&options).unwrap();
        });
    }
}