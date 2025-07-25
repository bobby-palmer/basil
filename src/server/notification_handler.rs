mod build_exit;

use serde::de::DeserializeOwned;
use serde_json::Value;
use tracing::debug;

use crate::server::{context::Context, notification_handler::build_exit::BuildExitHandler};

pub struct NotificationHandler;

impl NotificationHandler {
    pub async fn call(method: &str, ctx: &Context, params: Option<Value>) {
        match method {
            "build/exit" => BuildExitHandler::call(ctx, params).await,
            other_method => debug!(
                "Dropping notification with unknown method: {}",
                other_method
            ),
        }
    }
}

trait WithParamsHandler {
    type Input: DeserializeOwned;

    async fn handle(ctx: &Context, params: Self::Input);

    async fn call(ctx: &Context, params: Option<Value>) {
        let params = params.and_then(|p| serde_json::from_value::<Self::Input>(p).ok());

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
