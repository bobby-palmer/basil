use crate::io;

pub struct Server {
    recv: tokio::sync::mpsc::Receiver<String>,
    send: tokio::sync::mpsc::Sender<String>,
}

impl Server {
    pub fn new() -> Self {
        let (send, recv) = tokio::sync::mpsc::channel(10);
        Self { recv, send }
    }

    pub fn run<R, W>(reader: R, writer: W) -> anyhow::Result<()>
    where
        R: io::MessageReader,
        W: io::MessageWriter,
    {

        // tokio select
        Ok(())
    }
}
