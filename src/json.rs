use std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet};
use std::str::FromStr;
use std::fmt::{Display, Formatter, Error, Debug, Write};
use std::hash::{Hash, BuildHasher};
use std::mem::MaybeUninit;
use backtrace::Backtrace;
use std::collections::hash_map::RandomState;

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

/// A data structure that can be parsed from the value
/// of a JSON string.
///
/// # Example
/// ```
/// use automate::FromJson;
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
    pub backtrace: String,
}

impl JsonError {
    pub fn new<S>(msg: S) -> JsonError where S: Into<String> {
        JsonError { msg: msg.into(), backtrace: format!("{:#?}", Backtrace::new()) }
    }

    pub fn err<S, T>(msg: S) -> Result<T, JsonError> where S: Into<String> {
        Err(JsonError { msg: msg.into(), backtrace: format!("{:#?}", Backtrace::new()) })
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

/// A value that has to be included in the JSON
/// string but can contain the value null.
///
/// Some values in discord can be omitted from
/// the JSON string, these are represented as
/// Option<T>. Some other values must be in the
/// JSON string but may contain the null value
/// and should be represented using Nullable<T>.
#[derive(Debug, Clone, Copy)]
pub enum Nullable<T> {
    Value(T),
    Null,
}

/// Implements [AsJson](automate::AsJson) and
/// [FromJson](automate::FromJson) for simple numeric and
/// other types that should be represented as is in the
/// JSON string.
macro_rules! impl_for_num {
    ($($ty:ty),*) => (
        $(
            impl AsJson for $ty {
                #[inline]
                fn as_json(&self) -> String {
                    self.to_string()
                }

                #[allow(unused_must_use)]
                #[inline]
                fn concat_json(&self, dest: &mut String) {
                    dest.write_fmt(format_args!("{}", self)).expect("A Display implementation returned an error unexpectedly");
                }
            }

            impl FromJson for $ty {
                #[inline]
                fn from_json(json: &str) -> Result<$ty, JsonError> {
                    <$ty>::from_str(json)
                        .map_err(|_| JsonError::new(format!("Failed to parse {} into {}", json, stringify!($ty))))
                }
            }
        )*
    )
}

/// Implements [AsJson](automate::AsJson) and
/// [FromJson](automate::FromJson) for larger numeric
/// that should be represented as strings in the
/// JSON string.
macro_rules! impl_for_large_num {
    ($($ty:ty),*) => (
        $(
            impl AsJson for $ty {
                #[allow(unused_must_use)]
                #[inline]
                fn as_json(&self) -> String {
                    let mut string = String::new();
                    string.write_fmt(format_args!("\"{}\"", self)).expect("A Display implementation returned an error unexpectedly");
                    string
                }

                #[allow(unused_must_use)]
                #[inline]
                fn concat_json(&self, dest: &mut String) {
                    dest.write_fmt(format_args!("\"{}\"", self)).expect("A Display implementation returned an error unexpectedly");
                }
            }

            impl FromJson for $ty {
                #[inline]
                fn from_json(json: &str) -> Result<$ty, JsonError> {
                    if json.len() >= 2 && json.starts_with('"') && json.ends_with('"') {
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

/// Implements [AsJson](automate::AsJson) and
/// [FromJson](automate::FromJson) for collections.
macro_rules! impl_for_single_collection {
    ($($ty:ident:$insert_method:ident <$($rq_trait:ident),*> ),*) => {
        $(
            impl<J> AsJson for $ty<J> where J: AsJson {
                #[inline]
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

                #[inline]
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

            impl<J> FromJson for $ty<J> where J: FromJson $(+ $rq_trait)* {
                #[inline]
                fn from_json(json: &str) -> Result<$ty<J>, JsonError> {
                    if json.len() >= 2 && json.starts_with('[') && json.ends_with(']') {
                        let mut col: $ty<J> = $ty::new();
                        if (&json[1..json.len()-1]).trim().is_empty() {
                            return Ok(col);
                        }

                        let mut nesting_level = 0;
                        let mut val_begin: usize = 0;

                        for (i, c) in json.char_indices() {
                            if c == '{' || c == '[' {
                                nesting_level += 1;

                                //we enter the root object
                                if nesting_level == 1 {
                                    val_begin = i + 1;
                                }

                                continue;
                            } else if c == '}' || c == ']' {
                                nesting_level -= 1;

                                //we hit end of json, but because there isn't a final comma, there is still 1 value
                                //waiting to be added to the collection
                                if nesting_level == 0 && val_begin != 0 {
                                    col.$insert_method(J::from_json((&json[val_begin..i]).trim())?);
                                    return Ok(col);
                                }

                                continue;
                            }

                            if nesting_level == 1 {
                                if c == ',' {
                                    col.$insert_method(J::from_json((&json[val_begin..i]).trim())?);

                                    val_begin = i + 1;
                                }
                            }
                        }

                        return Ok(col);
                    }

                    JsonError::err("Invalid array format given")
                }
            }
        )*
    };
}

/// Implements [AsJson](automate::AsJson) and
/// [FromJson](automate::FromJson) for collections.
macro_rules! impl_for_associative_collection {
    ($($ty:ident <$($rq_first_trait:ident),*> ),*) => {
        $(
            impl<J> AsJson for $ty<String, J> where J: AsJson {
                #[inline]
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

                #[inline]
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

            impl<J> FromJson for $ty<String, J> where J: FromJson $(+ $rq_first_trait)* {
                #[inline]
                fn from_json(json: &str) -> Result<$ty<String, J>, JsonError> {
                    if json.len() >= 2 && json.starts_with('{') && json.ends_with('}') {
                        return json_object_to_map(json)?
                            .iter()
                            .map(|(&k, &v)| {
                                match J::from_json(v.trim()) {
                                    Ok(v) => Ok((String::from(k), v)),
                                    Err(err) => Err(err)
                                }
                            })
                            .collect::<Result<$ty<String, J>, JsonError>>()
                    }

                    JsonError::err("Invalid object format given")
                }
            }
        )*
    };
}

macro_rules! impl_for_arrays {
    ($($size:tt),*) => (
        $(
            impl<T> AsJson for [T; $size] where T: AsJson {
                #[inline]
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

                #[allow(unused_must_use)]
                #[inline]
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

            impl<T> FromJson for [T; $size] where T: FromJson  {
                #[inline]
                fn from_json(json: &str) -> Result<[T; $size], JsonError> {
                    if json.len() >= 2 && json.starts_with('[') && json.ends_with(']') {
                        let split = json.split(',');
                        let mut count = 0;

                        let array: [T; $size] = unsafe {
                            let mut arr: MaybeUninit<[T; $size]> = MaybeUninit::uninit();
                            let arr_ptr = arr.as_mut_ptr() as *mut T; // pointer to 1st element

                            for val in split {
                                arr_ptr.add(count).write(T::from_json(val.trim())?);
                                count += 1;
                            }

                            if count != $size {
                                return JsonError::err(format!("Expected an array of size {}, got {}", $size, count));
                            }

                            arr.assume_init()
                        };


                        return Ok(array);
                    }

                    JsonError::err("Invalid array format given")
                }
            }
        )*
    )
}

impl_for_num! {
    i8, i16, i32, isize,
    u8, u16, u32, usize,
    f32,
    bool
}

impl_for_large_num! {
    i64, i128,
    u64, u128,
    f64
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

impl AsJson for String {
    #[inline]
    fn as_json(&self) -> String {
        let mut string = String::with_capacity(self.len() + 2);
        string.push('"');
        string.push_str(&self);
        string.push('"');

        string
    }

    #[inline]
    fn concat_json(&self, dest: &mut String) {
        dest.reserve(self.len() + 2);
        dest.push('"');
        dest.push_str(&self);
        dest.push('"');
    }
}

impl FromJson for String {
    #[inline]
    fn from_json(json: &str) -> Result<String, JsonError> {
        if json.len() >= 2 && json.starts_with('"') && json.ends_with('"') {
            Ok(String::from(&json[1..json.len() - 1]))
        } else {
            JsonError::err("Incorrect JSON string value received")
        }
    }
}

impl AsJson for &str {
    #[inline]
    fn as_json(&self) -> String {
        let mut string = String::with_capacity(self.len() + 3);
        string.push('"');
        string.push_str(&self);
        string.push('"');

        string
    }

    #[inline]
    fn concat_json(&self, dest: &mut String) {
        dest.reserve(self.len() + 3);
        dest.push('"');
        dest.push_str(&self);
        dest.push('"');
    }
}

impl<T> Display for Nullable<T> where T: Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Nullable::Value(v) => f.write_fmt(format_args!("{}", v)),
            Nullable::Null => f.write_str("null")
        }
    }
}

impl<T> AsJson for Nullable<T> where T: AsJson {
    fn as_json(&self) -> String {
        if let Nullable::Value(val) = self {
            val.as_json()
        } else {
            "null".to_owned()
        }
    }

    fn concat_json(&self, dest: &mut String) {
        if let Nullable::Value(val) = self {
            val.concat_json(dest);
        } else {
            dest.push_str("null");
        }
    }
}

impl<T> FromJson for Nullable<T> where T: FromJson {
    fn from_json(json: &str) -> Result<Nullable<T>, JsonError> where Self: Sized {
        Ok(match json {
            "null" => Nullable::Null,
            val => Nullable::Value(T::from_json(val)?)
        })
    }
}

impl<T> Default for Nullable<T> {
    fn default() -> Self {
        Nullable::Null
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    fn from(opt: Option<T>) -> Self {
        if let Some(opt) = opt {
            Nullable::Value(opt)
        } else {
            Nullable::Null
        }
    }
}

impl<J, S: BuildHasher> AsJson for HashSet<J, S> where J: AsJson {
    #[inline]
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

    #[inline]
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

impl<J> FromJson for HashSet<J, RandomState> where J: FromJson + Hash + Eq {
    #[inline]
    fn from_json(json: &str) -> Result<HashSet<J, RandomState>, JsonError> {
        if json.len() >= 2 && json.starts_with('[') && json.ends_with(']') {
            let mut col: HashSet<J, RandomState> = HashSet::new();
            if (&json[1..json.len() - 1]).trim().is_empty() {
                return Ok(col);
            }

            let mut nesting_level = 0;
            let mut val_begin: usize = 0;

            for (i, c) in json.char_indices() {
                if c == '[' {
                    nesting_level += 1;

                    //we enter the root object
                    if nesting_level == 1 {
                        val_begin = i + 1;
                    }

                    continue;
                } else if c == ']' {
                    nesting_level -= 1;

                    //we hit end of json, but because there isn't a final comma, there is still 1 value
                    //waiting to be added to the collection
                    if nesting_level == 0 && val_begin != 0 {
                        col.insert(J::from_json((&json[val_begin..i]).trim())?);
                        return Ok(col);
                    }

                    continue;
                }

                if c == ',' {
                    col.insert(J::from_json((&json[val_begin..i]).trim())?);

                    val_begin = i + 1;
                }
            }

            return Ok(col);
        }

        JsonError::err("Invalid array format given")
    }
}

impl<J, S: BuildHasher> AsJson for HashMap<String, J, S> where J: AsJson {
    #[inline]
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

    #[inline]
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

impl<J> FromJson for HashMap<String, J, RandomState> where J: FromJson {
    #[inline]
    fn from_json(json: &str) -> Result<HashMap<String, J, RandomState>, JsonError> {
        if json.len() >= 2 && json.starts_with('{') && json.ends_with('}') {
            return json_object_to_map(json)?
                .iter()
                .map(|(&sk, &sv)| {
                    match J::from_json(sv.trim()) {
                        Ok(v) => Ok((String::from(sk), v)),
                        Err(err) => Err(err)
                    }
                })
                .collect();
        }

        JsonError::err("Invalid object format given")
    }
}

//fn tmp(json: &str) {
//    let mut nesting_level = 0;
//    let mut key_idxs: [usize; 2] = [0; 2];
//    let mut val_idxs: [usize; 2] = [0; 2];
//
//    for (i, c) in input.char_indices() {
//        if c == '{' || c == '[' {
//            nesting_level += 1;
//        } else if c == '}' || c == ']' {
//            nesting_level -= 1;
//
//            //we hit end of json, but because there isn't a final comma, there is still 1 key/value
//            //pair waiting to be added to the map
//            if nesting_level == 0 && val_idxs[0] != 0 {
//                val_idxs[1] = i;
//
//                map.insert(
//                    &input[key_idxs[0]..key_idxs[1]],
//                    (&input[val_idxs[0]..val_idxs[1]]).trim(),
//                );
//
//                return Ok(map);
//            }
//        } else if nesting_level == 1 {
//            if c == '"' {
//                if key_idxs[0] == 0 {
//                    key_idxs[0] = i + 1;
//                } else if key_idxs[1] == 0 {
//                    key_idxs[1] = i;
//                }
//            } else if val_idxs[0] == 0 && c == ':' {
//                val_idxs[0] = i + 1;
//            } else if val_idxs[1] == 0 && c == ',' {
//                match &input[key_idxs[0]..key_idxs[1]] {
//                    #(
//                     #fns => #fn.replace(::automate::FromJson::from_json((&input[val_idxs[0]..val_idxs[1]]).trim())?)
//                    ),*
//                }
//                val_idxs[1] = i;
//
//                map.insert(
//                    &input[key_idxs[0]..key_idxs[1]],
//                    (&input[val_idxs[0]..val_idxs[1]]).trim(),
//                );
//
//                key_idxs = [0; 2];
//                val_idxs = [0; 2];
//            }
//        }
//    }
//}

pub fn json_object_to_map(input: &str) -> Result<HashMap<&str, &str>, JsonError> {
    let mut map = HashMap::new();
    let mut nesting_level = 0;
    let mut key_idxs: [usize; 2] = [0; 2];
    let mut val_idxs: [usize; 2] = [0; 2];

    for (i, c) in input.char_indices() {
        if c == '{' || c == '[' {
            nesting_level += 1;
        } else if c == '}' || c == ']' {
            nesting_level -= 1;

            //we hit end of json, but because there isn't a final comma, there is still 1 key/value
            //pair waiting to be added to the map
            if nesting_level == 0 && val_idxs[0] != 0 {
                val_idxs[1] = i;

                map.insert(
                    &input[key_idxs[0]..key_idxs[1]],
                    (&input[val_idxs[0]..val_idxs[1]]).trim(),
                );

                return Ok(map);
            }
        } else if nesting_level == 1 {
            if c == '"' {
                if key_idxs[0] == 0 {
                    key_idxs[0] = i + 1;
                } else if key_idxs[1] == 0 {
                    key_idxs[1] = i;
                }
            } else if val_idxs[0] == 0 && c == ':' {
                val_idxs[0] = i + 1;
            } else if val_idxs[1] == 0 && c == ',' {
                val_idxs[1] = i;

                map.insert(
                    &input[key_idxs[0]..key_idxs[1]],
                    (&input[val_idxs[0]..val_idxs[1]]).trim(),
                );

                key_idxs = [0; 2];
                val_idxs = [0; 2];
            }
        }
    }

    if nesting_level == 0 {
        Ok(map)
    } else {
        JsonError::err(format!("Given string is not a valid JSON string: {}", nesting_level))
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
    use crate::map;

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
    fn test_root_search_key_not_found() {
        let simple = r#"{"key":"value"}"#;

        assert_eq!(json_root_search::<String>("ke", simple).err().unwrap().msg, "Could not find key in candidate");
        assert_eq!(json_root_search::<String>("ey", simple).err().unwrap().msg, "Could not find key in candidate");
        assert_eq!(json_root_search::<String>("aaa", simple).err().unwrap().msg, "Could not find key in candidate");
        assert_eq!(json_root_search::<String>("value", simple).err().unwrap().msg, "Could not find key in candidate");
    }

    #[test]
    fn test_root_search_empty_key() {
        let simple = r#"{"key":"value"}"#;

        assert_eq!(json_root_search::<String>("", simple).err().unwrap().msg, "The searched key can't be empty");
    }

    #[test]
    fn test_root_search_nested_vec() {
        let contains_array = r#"{"array":["a","b","c"],"b":40}"#;

        assert_eq!(json_root_search::<u8>("b", contains_array).unwrap(), 40);
        assert_eq!(json_root_search::<String>("c", contains_array).err().unwrap().msg, "Could not find key in candidate");
    }

    #[test]
    fn test_root_search_nested_object() {
        let nested_safe_order = r#"{"a":700,"nested":{"a":700}}"#;
        let nested_risky_order = r#"{"nested":{"a":1000},"a":700}"#;

        assert_eq!(json_root_search::<u16>("a", nested_safe_order).unwrap(), 700);
        assert_eq!(json_root_search::<u16>("a", nested_risky_order).unwrap(), 700);
    }

    #[test]
    fn test_search_invalid_json() {
        let no_first_brace = r#""key":"value"}"#;
        let no_final_brace = r#"{"key":"value""#;
        let no_comma = r#"{"key":"value""other":5}"#;
        let no_key_quote = r#"{"key:"value","other":5}"#;
        let no_val_quote = r#"{"key":value","other":5}"#;
        let wrong_type = r#"{"int":"value"}"#;

        assert_eq!(json_root_search::<String>("key", no_first_brace).err().unwrap().msg,
                   "Incorrectly formatted JSON string");
        assert_eq!(json_root_search::<String>("key", no_final_brace).err().unwrap().msg,
                   "Unexpected end of string");
        assert_eq!(json_root_search::<String>("key", no_comma).err().unwrap().msg,
                   "Failed to parse JSON: Incorrect JSON string value received");
        assert_eq!(json_root_search::<String>("key", no_key_quote).err().unwrap().msg,
                   "Could not find key in candidate"); //the function can hardly know that the string is not correctly formatted
        assert_eq!(json_root_search::<String>("key", no_val_quote).err().unwrap().msg,
                   "Failed to parse JSON: Incorrect JSON string value received");
        assert_eq!(json_root_search::<u32>("int", wrong_type).err().unwrap().msg,
                   "Failed to parse JSON: Failed to parse \"value\" into u32");
    }

    #[test]
    fn test_object_to_map_basic() {
        let no_final_brace = r#"{"key":"value"}"#;
        let no_final_brace_result = json_object_to_map(no_final_brace).unwrap();

        assert!(no_final_brace_result.contains_key("key"));
        assert_eq!(no_final_brace_result.get("key").unwrap(), &"\"value\"");
    }

    #[test]
    fn test_serialize_map() {
        let str_map_of_vec = r#"{"key":["vec","of","strings"]}"#;
        let str_map_of_empty_vec = r#"{"key":[]}"#;

        let map_of_vec: HashMap<String, Vec<String>> = map! {
            "key".to_owned() => vec!["vec".to_owned(), "of".to_owned(), "strings".to_owned()]
        };

        let map_of_empty_vec: HashMap<String, Vec<String>> = map! {
            "key".to_owned() => vec![]
        };

        assert_eq!(HashMap::from_json(str_map_of_vec).unwrap(), map_of_vec);
        assert_eq!(HashMap::from_json(str_map_of_empty_vec).unwrap(), map_of_empty_vec);
    }
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use serde::{Serialize, Deserialize};
    use test::Bencher;

    #[derive(Serialize, Deserialize, AsJson, FromJson)]
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

    #[derive(Serialize, Deserialize, AsJson, FromJson)]
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

    #[derive(Serialize, Deserialize, AsJson, FromJson, Clone)]
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

    #[derive(Serialize, Deserialize, AsJson, FromJson)]
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
    fn bench_json_object_to_map_average(b: &mut Bencher) {
        let something = Something::create().as_json();

        b.iter(|| {
            json_object_to_map(&something).unwrap();
        });
    }

    #[bench]
    fn bench_json_object_to_map_long_str(b: &mut Bencher) {
        let long_str = LongStr::create().as_json();

        b.iter(|| {
            json_object_to_map(&long_str).unwrap();
        });
    }

    #[bench]
    fn bench_json_root_search_average(b: &mut Bencher) {
        let something = Something::create().as_json();

        b.iter(|| {
            json_root_search::<String>("somewhere", &something).unwrap();
        });
    }

    #[bench]
    fn bench_json_root_search_long_str(b: &mut Bencher) {
        let long_str = LongStr::create().as_json();

        b.iter(|| {
            json_root_search::<String>("second", &long_str).unwrap();
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

    #[bench]
    fn bench_deserializer_average_automate(b: &mut Bencher) {
        let something_json = Something::create().as_json();

        b.iter(|| {
            Something::from_json(&something_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_average_serde(b: &mut Bencher) {
        let something_json = Something::create().as_json();

        b.iter(|| {
            serde_json::from_str::<Something>(&something_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_short_automate(b: &mut Bencher) {
        let short_json = Short::create().as_json();

        b.iter(|| {
            Short::from_json(&short_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_short_serde(b: &mut Bencher) {
        let short_json = Short::create().as_json();

        b.iter(|| {
            serde_json::from_str::<Short>(&short_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_long_string_automate(b: &mut Bencher) {
        let long_str_json = LongStr::create().as_json();

        b.iter(|| {
            LongStr::from_json(&long_str_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_long_string_serde(b: &mut Bencher) {
        let long_str_json = LongStr::create().as_json();

        b.iter(|| {
            serde_json::from_str::<LongStr>(&long_str_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_empty_hashmap_automate(b: &mut Bencher) {
        let map_json = HashMap::<String, String>::new().as_json();
        println!("{}", map_json);

        b.iter(|| {
            HashMap::<String, String>::from_json(&map_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_empty_hashmap_serde(b: &mut Bencher) {
        let map_json = HashMap::<String, String>::new().as_json();

        b.iter(|| {
            serde_json::from_str::<HashMap<String, String>>(&map_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_full_hashmap_automate(b: &mut Bencher) {
        let mut map: HashMap<String, String> = HashMap::new();
        for _ in 0..500 {
            map.insert(String::from("hello"), String::from("world"));
        }

        let map_json = map.as_json();

        b.iter(|| {
            HashMap::<String, String>::from_json(&map_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_full_hashmap_serde(b: &mut Bencher) {
        let mut map: HashMap<String, String> = HashMap::new();
        for _ in 0..500 {
            map.insert(String::from("hello"), String::from("world"));
        }

        let map_json = map.as_json();

        b.iter(|| {
            serde_json::from_str::<HashMap<String, String>>(&map_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_empty_vec_automate(b: &mut Bencher) {
        let vec_json = Vec::<String>::new().as_json();
        println!("{}", vec_json);

        b.iter(|| {
            Vec::<String>::from_json(&vec_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_empty_vec_serde(b: &mut Bencher) {
        let vec_json = Vec::<String>::new().as_json();

        b.iter(|| {
            serde_json::from_str::<Vec<String>>(&vec_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_full_vec_automate(b: &mut Bencher) {
        let mut vec: Vec<String> = Vec::new();
        for _ in 0..500 {
            vec.push(String::from("hello world"));
        }

        let vec_json = vec.as_json();

        b.iter(|| {
            Vec::<String>::from_json(&vec_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_full_vec_serde(b: &mut Bencher) {
        let mut vec: Vec<String> = Vec::new();
        for _ in 0..500 {
            vec.push(String::from("hello world"));
        }

        let vec_json = vec.as_json();

        b.iter(|| {
            serde_json::from_str::<Vec<String>>(&vec_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_options_all_some_automate(b: &mut Bencher) {
        let options_json = Options::all_some().as_json();

        b.iter(|| {
            Options::from_json(&options_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_options_all_some_serde(b: &mut Bencher) {
        let options_json = Options::all_some().as_json();

        b.iter(|| {
            serde_json::from_str::<Options>(&options_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_options_half_some_automate(b: &mut Bencher) {
        let options_json = Options::half_some().as_json();

        b.iter(|| {
            Options::from_json(&options_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_options_half_some_serde(b: &mut Bencher) {
        let options_json = Options::half_some().as_json();

        b.iter(|| {
            serde_json::from_str::<Options>(&options_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_options_no_some_automate(b: &mut Bencher) {
        let options_json = Options::no_some().as_json();

        b.iter(|| {
            Options::from_json(&options_json).unwrap();
        });
    }

    #[bench]
    fn bench_deserializer_options_no_some_serde(b: &mut Bencher) {
        let options_json = Options::no_some().as_json();
println!("{}", options_json);
        b.iter(|| {
            serde_json::from_str::<Options>(&options_json).unwrap();
        });
    }
}