#[derive(Debug)]
pub enum BasilError {
    ParseError(String),
    InvalidRequest(String),
    MethodNotFound(String),
    InvalidParams(String),
    InternalError(String),
    ServerNotInitialized(String),
    UnknownErrorCode(String),
    RequestFailed(String),
    ServerCancelled(String),
    ContentModified(String),
    RequestCancelled(String),
}

impl std::fmt::Display for BasilError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}
