use crate::io::{MessageReader, MessageWriter};

mod error;

const BUF_LEN: usize = 10;

pub async fn run_server(
    mut reader: impl MessageReader, 
    mut writer: impl MessageWriter) {

    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(BUF_LEN);

    loop {
        let message = rx.recv().await.unwrap();
        writer.write(message).await.unwrap();
    }

}
