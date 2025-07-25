mod initialize_build;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::server::{context::Context, error::BasilError};

pub struct RequestHandler;

impl RequestHandler {
    pub async fn call(
        method: String,
        ctx: &Context,
        params: Option<Value>,
    ) -> Result<Value, BasilError> {
        match method {
            other_method => Err(BasilError::MethodNotFound(other_method)),
        }
    }
}

trait WithParamsHandler<P, R>
where
    P: DeserializeOwned,
    R: Serialize,
{
    async fn handle(ctx: &Context, params: P) -> Result<R, BasilError>;

    async fn call(ctx: &Context, params: Option<Value>) -> Result<Value, BasilError> {
        let params = match params {
            Some(params) => Ok(params),
            None => Err(BasilError::InvalidParams("Missing params".to_owned())),
        }?;

        let params: P = serde_json::from_value(params)
            .map_err(|err| BasilError::InvalidParams(err.to_string()))?;
        Self::handle(ctx, params)
            .await
            .map(|result| serde_json::to_value(result).unwrap())
    }
}

trait NoParamsHandler<R>
where
    R: Serialize,
{
    async fn handle(ctx: &Context) -> Result<R, BasilError>;

    async fn call(ctx: &Context, _params: Option<Value>) -> Result<Value, BasilError> {
        Self::handle(ctx)
            .await
            .map(|result| serde_json::to_value(result).unwrap())
    }
}
