use std::collections::HashMap;

pub trait AsJson {
    fn as_json(&self) -> String;
}

trait Zero {
    const ZERO: Self;
}

macro_rules! impl_as_json {
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

impl_as_json! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    bool
}

impl AsJson for String {
    fn as_json(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl<J> AsJson for HashMap<J, J> where J: AsJson {
    fn as_json(&self) -> String {
        let mut json = String::from("{");

        for (key, val) in self {
            json.push_str(&format!("{}:{}", key.as_json(), val.as_json()));
            json.push(',')
        }

        json.pop(); //remove last comma
        json.push('}');

        json
    }
}