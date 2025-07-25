use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub allowed_targets: Option<Vec<String>>,
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let config_str = std::fs::read_to_string(path)?;
        Self::from_str(&config_str)
    }

    pub fn from_str(str: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(str)?)
    }
}
