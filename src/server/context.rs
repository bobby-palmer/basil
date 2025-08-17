use tokio::sync::mpsc::Sender;

use crate::{bazel::BazelClient, server::rpc::ResponseMessage};

pub struct Context {}

impl Context {
    pub(super) fn new(_sender: Sender<Vec<u8>>, _client: BazelClient) 
        -> Self {
        todo!()
    }
    pub(super) async fn send_response(&self, _message: ResponseMessage) {
        todo!()
    }
}
