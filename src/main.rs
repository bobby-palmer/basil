mod io;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stdin = io::StdinReader::new();
    let stdout = io::StdoutWriter::new();
    Ok(())
}
