use tracing::error;

mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file_appender = tracing_appender::rolling::never(".basil", "logs");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();
    
    run_basil().await.inspect_err(|err| {
        error!("{}", err.to_string())        
    })
}

async fn run_basil() -> anyhow::Result<()> {
    Ok(())
}
