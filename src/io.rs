pub trait MessageReader {
    async fn read() -> String;
}

pub trait MessageWriter {
    async fn write(message: String);
}
