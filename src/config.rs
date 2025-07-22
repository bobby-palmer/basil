use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    // TODO
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}
