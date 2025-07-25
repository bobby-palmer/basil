use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::server::context::Context;

pub struct NotificationHandler;

impl NotificationHandler {
    pub async fn call(method: String, ctx: &Context, params: Option<Value>) {
        match method {
            other_method => {}
        }
    }
}

trait WithParamsHandler<P>
where
    P: DeserializeOwned,
{
    async fn handle(ctx: &Context, params: P);

    async fn call(ctx: &Context, params: Option<Value>) {
        let params = params.and_then(|p| serde_json::from_value::<P>(p).ok());

        if let Some(params) = params {
            Self::handle(ctx, params).await;
        }
    }
}

trait NoParamsHandler {
    async fn handle(ctx: &Context);

    async fn call(ctx: &Context, _params: Option<Value>) {
        Self::handle(ctx).await;
    }
}
