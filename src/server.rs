use crate::{io, config};

pub async fn run<R, W>(mut reader: R, mut writer: W, config: config::Config) -> anyhow::Result<()> 
where
    R: io::MessageReader,
    W: io::MessageWriter
{
    Ok(())
}
