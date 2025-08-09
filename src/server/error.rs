pub struct BspError {
    code: BspErrorKind,
    message: String,
}

pub enum BspErrorKind {
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
