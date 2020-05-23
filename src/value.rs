use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Json {
    Null,
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

impl Json {
    pub fn is_null(&self) -> bool {
        match *self {
            Json::Null => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match *self {
            Json::String(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match *self {
            Json::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match *self {
            Json::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match *self {
            Json::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match *self {
            Json::Object(_) => true,
            _ => false,
        }
    }
}
