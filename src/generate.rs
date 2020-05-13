use std::collections::HashMap;

use crate::value::Json;

pub struct CodeGenerator {
    value: String,
    indentation: u16,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            indentation: 0,
        }
    }

    pub fn gather(&mut self, json: &Json) {
        match *json {
            Json::Null => self.write("null"),
            Json::Boolean(ref b) => self.write(if *b { "true" } else { "false" }),
            Json::Number(ref n) => self.write(&n.to_string()),
            Json::String(ref s) => self.write(&format!("{:?}", s)),
            Json::Array(ref a) => self.write_array(a),
            Json::Object(ref o) => self.write_object(o),
        }
    }

    pub fn product(self) -> String {
        self.value
    }

    fn write(&mut self, slice: &str) {
        self.value.push_str(slice);
    }

    fn write_char(&mut self, ch: char) {
        self.value.push(ch);
    }

    fn write_array(&mut self, array: &[Json]) {
        self.write_char('[');
        self.write_char(']');
    }

    fn write_object(&mut self, object: &HashMap<String, Json>) {
        self.write_char('{');
        self.write_char('}');
    }
}
