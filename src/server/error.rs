use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct BspError {
    #[serde(serialize_with = "serialize_error_code")]
    code: BspErrorKind,
    message: String,
}

impl From<(BspErrorKind, String)> for BspError {
    fn from(value: (BspErrorKind, String)) -> Self {
        Self {
            code: value.0,
            message: value.1,
        }
    }
}

#[derive(Clone, Copy)]
pub enum BspErrorKind {
    // Defined by JSON-RPC
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,

    /// Error code indicating that a server received a notification or
    /// request before the server received the `initialize` request
    ServerNotInitialized,
    UnknownErrorCode,

    /// A request failed but it was syntactically correct, e.g the method name
    /// was known and the parameters were valid. The error message should
    /// contain human readable information about why the request failed.
    RequestFailed,

    /// The server cancelled the request. This error code should only be used
    /// for requests that explicitly support being server cancellable.
    ServerCancelled,

    /// The server detected that the content of a document got modified
    /// outside normal conditions. A server should NOT send this error code
    /// if it detects a content change in its unprocessed messages. The
    /// result even computed on an older state might still be useful for the
    /// client. If a client decides that a result is not of any use anymore
    /// the client should cancel the request.
    ContentModified,

    /// The client has canceled a request and a server has detected the cancel.
    RequestCancelled,
}

impl From<BspErrorKind> for i32 {
    fn from(value: BspErrorKind) -> Self {
        match value {
            BspErrorKind::ParseError => -32700,
            BspErrorKind::InvalidRequest => -32600,
            BspErrorKind::MethodNotFound => -32601,
            BspErrorKind::InvalidParams => -32602,
            BspErrorKind::InternalError => -32603,
            BspErrorKind::ServerNotInitialized => -32002,
            BspErrorKind::UnknownErrorCode => -32001,
            BspErrorKind::RequestFailed => -32803,
            BspErrorKind::ServerCancelled => -32802,
            BspErrorKind::ContentModified => -32801,
            BspErrorKind::RequestCancelled => -32800,
        }
    }
}

/// Serialize error as integer value
fn serialize_error_code<S>(code: &BspErrorKind, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let code_value: i32 = (*code).into();
    serializer.serialize_i32(code_value)
}
