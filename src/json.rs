use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[derive(Deserialize)]
struct RequestMessage {
    jsonrpc: String,
    id: Value,
    method: String,
    params: Option<Value>,
}

#[derive(Deserialize, Serialize)]
#[skip_serializing_none]
struct NotificationMessage {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
}

#[derive(Serialize)]
#[skip_serializing_none]
struct ResponseMessage {
    id: Value,
    result: Option<Value>,
    error: Option<ResponseError>
}

#[derive(Serialize)]
struct ResponseError {
    code: i32,
    message: String,
}

pub enum BasilError {

    /// Invalid JSON was received by the server.
    /// An error occurred on the server while parsing the JSON text.
    ParseError(String),

    /// The JSON sent is not a valid Request object.
    InvalidRequest(String),

    /// The method does not exist / is not available.
    MethodNotFound(String),

    /// Invalid method parameter(s).
    InvalidParams(String),

    /// Internal JSON-RPC error.
    InternalError(String),

    /// Error code indicating that a server received a notification or
	  /// request before the server received the `initialize` request
    ServerNotInitialized(String),
    UnknownErrorCode(String),
    
    /// A request failed but it was syntactically correct, e.g the
    /// method name was known and the parameters were valid. The error
    /// message should contain human readable information about why
    /// the request failed.
    RequestFailed(String),

    /// The server cancelled the request. This error code should
	  /// only be used for requests that explicitly support being
	  /// server cancellable.
    ServerCancelled(String),

    /// The server detected that the content of a document got
    /// modified outside normal conditions. A server should
    /// NOT send this error code if it detects a content change
    /// in its unprocessed messages. The result even computed
    /// on an older state might still be useful for the client.
    ///
    /// If a client decides that a result is not of any use anymore
    /// the client should cancel the request.
    ContentModified(String),
    
    ///The client has canceled a request and a server has detected
	  /// the cancel.
    RequestCancelled(String),
}

impl Into<ResponseError> for BasilError {

    fn into(self) -> ResponseError {
        match self {
            BasilError::ParseError(message) => ResponseError { code: -32700, message },
            BasilError::InvalidRequest(message) => ResponseError { code: -32600, message },
            BasilError::MethodNotFound(message) => ResponseError { code: -32601, message },
            BasilError::InvalidParams(message) => ResponseError { code: -32602, message },
            BasilError::InternalError(message) => ResponseError { code: -32603, message },
            BasilError::ServerNotInitialized(message) => ResponseError { code: -32002, message },
            BasilError::UnknownErrorCode(message) => ResponseError { code: -32001, message },
            BasilError::RequestFailed(message) => ResponseError { code: -32803, message },
            BasilError::ServerCancelled(message) => ResponseError { code: -32802, message },
            BasilError::ContentModified(message) => ResponseError { code: -32801, message },
            BasilError::RequestCancelled(message) => ResponseError { code: -32800, message },
        }
    }
}
