use chrono::Utc;
use scylla::{
    cql_to_rust::{FromCqlVal, FromCqlValError},
    frame::{
        response::result::CqlValue,
        value::{Timestamp, Value, ValueTooBig},
    },
    FromRow, ValueList,
};
pub fn get_data_type(data: &serde_json::Value) -> String {
    return match data {
        serde_json::Value::Null => String::from("null"),
        serde_json::Value::Bool(_) => String::from("boolean"),
        serde_json::Value::String(_) => String::from("string"),
        serde_json::Value::Number(_) => String::from("number"),
        serde_json::Value::Object(_) => String::from("object"),
        serde_json::Value::Array(_) => String::from("array"),
    };
}

#[derive(Debug, FromRow, ValueList)]
pub struct Entry {
    pub topic: String,
    pub timestamp: Duration,
    pub data_type: String,
    pub data: String,
}

#[derive(Debug)]
pub struct Duration(chrono::Duration);

impl Duration {
    pub fn now() -> Self {
        let dt = Utc::now();
        let timestamp: i64 = dt.timestamp();
        Self(chrono::Duration::seconds(timestamp))
    }
}

impl Value for Duration {
    fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), ValueTooBig> {
        Timestamp(self.0).serialize(buf)
    }
}

impl FromCqlVal<Option<CqlValue>> for Duration {
    fn from_cql(cql_val: Option<CqlValue>) -> Result<Self, FromCqlValError> {
        chrono::Duration::from_cql(cql_val).map(Self)
    }
}
