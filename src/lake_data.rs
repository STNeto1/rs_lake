use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub topic: String,
    pub timestamp: String,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    data_type: String,
    data: Value,
}

impl Data {
    pub fn new_from_input(data: &Value) -> Self {
        let data_type = match data {
            Value::Null => String::from("null"),
            Value::Bool(_) => String::from("boolean"),
            Value::String(_) => String::from("string"),
            Value::Number(_) => String::from("number"),
            Value::Object(_) => String::from("object"),
            Value::Array(_) => String::from("array"),
        };

        return Self {
            data_type,
            data: data.to_owned(),
        };
    }
}
