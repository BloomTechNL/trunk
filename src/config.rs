use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(rename = "coAuthorsRequired")]
    #[serde(default = "default_co_authors_required")]
    pub co_authors_required: bool,
}

fn default_co_authors_required() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            co_authors_required: true,
        }
    }
}

pub trait TrunkConfig {
    fn load(&self) -> Config;

    fn set_co_authors_required(&self, required: bool) -> Result<()>;
}

pub struct RealTrunkConfig {
    path: PathBuf,
}

impl RealTrunkConfig {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl TrunkConfig for RealTrunkConfig {
    fn load(&self) -> Config {
        if !self.path.exists() {
            return Config::default();
        }
        match std::fs::read_to_string(&self.path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|e| {
                eprintln!(
                    "Warning: Failed to parse {}: {}. Using defaults.",
                    self.path.display(),
                    e
                );
                Config::default()
            }),
            Err(e) => {
                eprintln!(
                    "Warning: Failed to read {}: {}. Using defaults.",
                    self.path.display(),
                    e
                );
                Config::default()
            }
        }
    }

    fn set_co_authors_required(&self, required: bool) -> Result<()> {
        let mut config = self.load();
        config.co_authors_required = required;

        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(&config)?;
        std::fs::write(&self.path, json)?;
        Ok(())
    }
}

pub fn cmd_config(co_authors_required: Option<bool>, config: &impl TrunkConfig) -> Result<()> {
    if let Some(required) = co_authors_required {
        config.set_co_authors_required(required)?;
    }
    Ok(())
}
