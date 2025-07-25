mod bazel;
mod config;
mod io;
mod server;

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::never(".basil", "logs");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}
