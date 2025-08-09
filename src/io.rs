use anyhow::Result;

pub trait MessageWriter {
    /// Write a message with content length header
    async fn write(&mut self, message: String) -> Result<()>;
}

pub trait MessageReader {
    /// read a message with content length header
    async fn read(&mut self) -> Result<String>;
}
