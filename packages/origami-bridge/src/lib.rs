use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};

#[derive(Deserialize)]
pub struct Request {
    pub id: String,
    pub data: Value,
}

#[derive(Serialize)]
pub struct Response {
    pub id: usize,
    pub data: serde_json::Value,
}

impl Request {
    pub fn from_str(data: &str) -> Value {
        from_str(data).unwrap()
    }
}
