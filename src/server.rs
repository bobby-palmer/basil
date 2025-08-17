mod context;
mod error;
mod rpc;

use std::sync::Arc;

use context::Context;
use serde_json::Value;

use crate::{bazel::BazelClient, io::{MessageReader, MessageWriter}, server::{error::{BspError, BspErrorKind}, rpc::{RequestMessage, ResponseMessage}}};


pub async fn run<R, W>(mut reader: R, _writer: W, bazel_client: BazelClient) 
where
    R: MessageReader,
    W: MessageWriter,
{
    let (tx, rx) = tokio::sync::mpsc::channel::<Vec<u8>>(64);

    // recv and send response
    tokio::spawn(async move {
        loop {
            todo!()
        }
    });

    let ctx = Arc::new(Context::new(tx, bazel_client));

    loop {
        let request = reader.read_message().await.unwrap();
        let ctx = ctx.clone();

        tokio::spawn(async move {
            handle(request, &ctx).await;
        });
    }
}

async fn handle(message: Vec<u8>, ctx: &Context) {
    match serde_json::from_slice::<RequestMessage>(&message) {
        Ok(message) => {
            if message.is_notification() {
                handle_notification(&message.method, message.params, ctx).await;
            } else {
                let result = 
                    handle_request(&message.method, message.params, ctx).await;
                ctx.send_response(
                    ResponseMessage::new(
                        message.id,
                        result
                    )
                ).await;
            }
        },
        Err(err) => {
            ctx.send_response(
                ResponseMessage::new(
                    Value::Null, 
                    Err((BspErrorKind::ParseError, err.to_string()).into())
                )
            ).await;
        },
    }
}

async fn handle_request(
    method: &str, 
    _params: Option<Value>, 
    _ctx: &Context) -> Result<Value, BspError> {
    match method {
        other => Err((BspErrorKind::MethodNotFound, other.to_string()).into())
    }
}

async fn handle_notification(method: &str, _params: Option<Value>, _ctx: &Context) {
    match method {
        _ => {}
    }
}
