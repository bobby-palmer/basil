use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
struct RequestMessage {
    jsonrpc: String,
    id: Value,
    method: String,
    params: Option<Value>,
}

#[derive(Deserialize, Serialize)]
struct NotificationMessage {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
}

#[derive(Deserialize)]
enum IncomingMessage {
    Request(RequestMessage),
    Notification(NotificationMessage),
}

#[derive(Serialize)]
struct ResponseError {
    code: i32,
    message: String,
}

pub enum BasilError {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    ServerNotInitialized,
    UnknownErrorCode,
    RequestFailed,
    ServerCancelled,
    ContentModified,
    RequestCancelled,
}
