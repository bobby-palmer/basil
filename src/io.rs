use anyhow::Result;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWrite, AsyncWriteExt};

const LENGTH_HEADER: &str = "Content-Length: ";

pub trait MessageWriter {

    /// Write a message with content length header.
    /// Not cancellation safe: Do not use in tokio select!
    async fn write_message(&mut self, message: &[u8]) -> Result<()>;
}

impl<W> MessageWriter for W
where
    W: AsyncWrite + Unpin
{
    async fn write_message(&mut self, message: &[u8]) -> Result<()> {
        self.write_all(format!("{}{}\r\n\r\n", LENGTH_HEADER, message.len()).as_bytes()).await?;
        self.write_all(message).await?;
        Ok(())
    }
}

pub trait MessageReader {

    /// Read a message with content length header.
    /// Not cancellation safe: Do not use in tokio select!
    async fn read_message(&mut self) -> Result<Vec<u8>>;
}

impl<R> MessageReader for R
where
    R: AsyncBufRead + Unpin
{
    async fn read_message(&mut self) -> Result<Vec<u8>> {
        let mut content_length: Option<usize> = None;

        loop {
            let mut line = String::new();

            if self.read_line(&mut line).await? == 0 {
                anyhow::bail!("Empty header line");
            }

            let line = line.trim_end();

            if line.is_empty() {
                break;
            }

            if let Some(len) = line.strip_prefix(LENGTH_HEADER) {
                content_length = Some(len.parse()?);
            }
        }

        let len = content_length.ok_or(anyhow::anyhow!("Content length not specified"))?;
        let mut buffer = vec![0u8; len];
        self.read_exact(&mut buffer).await?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod test {
    use tokio::io::BufReader;
    use super::*;

    #[tokio::test]
    async fn test_matches_input_and_output() {
        let (mut d1, d2) = tokio::io::duplex(64);
        let mut reader = BufReader::new(d2);

        let message = "Hello this is a test: adofbsjdkfjbn";
        d1.write_message(message.as_bytes()).await.unwrap();

        assert_eq!(reader.read_message().await.unwrap(), message.as_bytes());
    }
}
