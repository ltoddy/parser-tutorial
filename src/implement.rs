use std::collections::HashMap;

use crate::value::Json;

macro_rules! impl_from_num_for_json {
    ($($t:ident)*) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    };
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64);

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}

impl From<Vec<Json>> for Json {
    fn from(v: Vec<Json>) -> Self {
        Json::Array(v)
    }
}

impl From<HashMap<String, Json>> for Json {
    fn from(mut map: HashMap<String, Json>) -> Self {
        let mut object = HashMap::new();

        for (key, value) in map.drain() {
            object.insert(key, value);
        }

        Json::Object(object)
    }
}
