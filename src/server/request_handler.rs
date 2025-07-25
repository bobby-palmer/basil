mod initialize_build;
use initialize_build::InitializeBuildHandler;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::server::{context::Context, error::BasilError};

pub struct RequestHandler;

impl RequestHandler {
    pub async fn call(
        method: &str,
        ctx: &Context,
        params: Option<Value>,
    ) -> Result<Value, BasilError> {
        match method {
            "build/initialize" => InitializeBuildHandler::call(ctx, params).await,
            other_method => Err(BasilError::MethodNotFound(other_method.to_owned())),
        }
    }
}

trait WithParamsHandler {
    type Input: DeserializeOwned;
    type Output: Serialize;

    async fn handle(ctx: &Context, params: Self::Input) -> Result<Self::Output, BasilError>;

    async fn call(ctx: &Context, params: Option<Value>) -> Result<Value, BasilError> {
        let params = match params {
            Some(params) => Ok(params),
            None => Err(BasilError::InvalidParams("Missing params".to_owned())),
        }?;

        let params: Self::Input = serde_json::from_value(params)
            .map_err(|err| BasilError::InvalidParams(err.to_string()))?;
        Self::handle(ctx, params)
            .await
            .map(|result| serde_json::to_value(result).unwrap())
    }
}

trait NoParamsHandler {
    type Output: Serialize;

    async fn handle(ctx: &Context) -> Result<Self::Output, BasilError>;

    async fn call(ctx: &Context, _params: Option<Value>) -> Result<Value, BasilError> {
        Self::handle(ctx)
            .await
            .map(|result| serde_json::to_value(result).unwrap())
    }
}
