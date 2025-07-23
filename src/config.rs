use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let config_str = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&config_str)?)
    }
}
