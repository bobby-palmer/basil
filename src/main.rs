use std::env;

mod io;
mod server;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_file = env::args().next();

    let config = match config_file {
        Some(file) => config::Config::from_file(&file),
        None => Err(anyhow::anyhow!("config file not specified"))
    }?;

    let stdin = io::StdinReader::new();
    let stdout = io::StdoutWriter::new();

    server::run(stdin, stdout, config).await
}
