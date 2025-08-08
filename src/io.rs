use anyhow::Result;
use tokio::io::{AsyncBufRead, AsyncReadExt, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};

const LENGTH_HEADER: &str = "Content-Length: ";

pub trait MessageReader {

    /// Read and return message with Content-Length header
    async fn read(&mut self) -> Result<String>;
}

impl<R> MessageReader for R
where
    R: AsyncBufRead + Unpin
{
    /// Not cancellation safe in tokio
    async fn read(&mut self) -> Result<String> {
        let mut length: Option<usize> = None;

        loop {
            let mut line = String::new();

            if self.read_line(&mut line).await? == 0 {
                anyhow::bail!("Reached EOF");
            }

            let line = line.trim();

            if line.is_empty() {
                break;
            }

            if let Some(len_str) = line.strip_prefix(LENGTH_HEADER) {
                length = Some(len_str.parse()?)
            }
        }

        let length = length.ok_or(anyhow::anyhow!("No content length specified"))?;
        let mut buf = vec![0u8; length];
        self.read_exact(&mut buf).await?;

        Ok(String::from_utf8(buf)?)
    }
}

pub trait MessageWriter {

    /// Write message with Content-Length header
    async fn write(&mut self, message: String) -> Result<()>;
}

impl<W> MessageWriter for W
where
    W: AsyncWrite + Unpin
{
    /// Not cancellation safe in tokio
    async fn write(&mut self, message: String) -> Result<()> {
        self.write_all(format!("{}{}\r\n\r\n", LENGTH_HEADER, message.len()).as_bytes()).await?;
        self.write_all(message.as_bytes()).await?;
        Ok(())
    }
}
