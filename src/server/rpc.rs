use serde_json::Value;
use serde_with::skip_serializing_none;

use crate::server::error::BspError;

#[skip_serializing_none]
#[derive(serde::Deserialize)]
pub struct RequestMessage {
    jsonrpc: String,
    pub id: Value,
    pub method: String,
    pub params: Option<Value>,
}

impl RequestMessage {
    pub fn is_notification(&self) -> bool {
        self.id.is_null()
    }
}

#[skip_serializing_none]
#[derive(serde::Serialize)]
pub struct ResponseMessage {
    jsonrpc: String,
    id: Value,
    result: Option<Value>,
    error: Option<BspError>,
}

impl ResponseMessage {
    pub fn new(id: Value, result: Result<Value, BspError>) -> Self {
        let (result, error) = match result {
            Ok(result) => (Some(result), None),
            Err(error) => (None, Some(error))
        };

        Self {
            jsonrpc: "2.0".into(),
            id,
            result,
            error,
        }
    }
}
