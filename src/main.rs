use crate::config::Config;

mod server;
mod bazel;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file_appender = tracing_appender::rolling::never(".basil", "logs");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();
    
    run_basil().await.inspect_err(|err| {
        tracing::error!("{}", err.to_string())        
    })
}

async fn run_basil() -> anyhow::Result<()> {
    let config_path = std::env::args().next();

    let config = match config_path {
        Some(path) => Config::from_file(&path),
        None => Err(anyhow::anyhow!("Missing basil config file arg"))
    }?;

    Ok(())
}
