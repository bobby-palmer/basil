use tokio::io::{stdin, stdout, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, Stdin, Stdout};

pub trait MessageReader {
    async fn read(&mut self) -> String;
}

pub struct StdinReader(BufReader<Stdin>);

impl StdinReader {
    pub fn new() -> Self {
        Self(BufReader::new(stdin()))
    }
}

impl MessageReader for StdinReader {
    async fn read(&mut self) -> String {

        let mut content_length: Option<usize> = None;

        loop {
            let mut line = String::new();

            if self.0.read_line(&mut line).await.unwrap() == 0 {
                panic!("stdin closed");
            }

            let line = line.trim();

            if line.is_empty() {
                break;
            }

            if let Some(len) = line.strip_prefix("Content-Length: ") {
                content_length = Some(len.parse().unwrap());
            }
        }

        let content_length = content_length.unwrap();
        let mut buf = vec![0u8; content_length];
        self.0.read_exact(&mut buf).await.unwrap();

        String::from_utf8_lossy(&buf).into()
    }
}

pub trait MessageWriter {
    async fn write(&mut self, message: String);
}

pub struct StdoutWriter(Stdout);

impl StdoutWriter {
    pub fn new() -> Self {
        Self(stdout())
    }
}

impl MessageWriter for StdoutWriter {
    async fn write(&mut self, message: String) {
        let message = message.as_bytes();
        let header = format!("Content-Length: {}\r\n\r\n", message.len());
        self.0.write_all(header.as_bytes()).await.unwrap();
        self.0.write_all(message).await.unwrap();
        self.0.flush().await.unwrap();
    }
}
