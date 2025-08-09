use crate::io::{MessageReader, MessageWriter};

mod error;

pub async fn run_server(
    reader: impl MessageReader, 
    writer: impl MessageWriter) {

}
