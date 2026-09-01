use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::git::repo_root;
use crate::handler::Handler;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(rename = "coAuthorsRequired")]
    #[serde(default = "default_co_authors_required")]
    pub co_authors_required: bool,
    #[serde(rename = "autoUpdatePeriod")]
    #[serde(default = "default_auto_update_period")]
    pub auto_update_period: u64,
}

const fn default_co_authors_required() -> bool {
    true
}

const fn default_auto_update_period() -> u64 {
    604_800
}

impl Default for Config {
    fn default() -> Self {
        Self {
            co_authors_required: true,
            auto_update_period: 604_800,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct PartialConfig {
    #[serde(rename = "coAuthorsRequired")]
    co_authors_required: Option<bool>,
    #[serde(rename = "autoUpdatePeriod")]
    auto_update_period: Option<u64>,
}

#[must_use]
pub fn merge_repo_override(mut config: Config, repo: &Path) -> Config {
    let Some(root) = repo_root(repo) else {
        return config;
    };
    let path = root.join(".trunk.json");
    if !path.exists() {
        return config;
    }
    let Ok(content) = std::fs::read_to_string(&path) else {
        return config;
    };
    match serde_json::from_str::<PartialConfig>(&content) {
        Ok(partial) => {
            if let Some(required) = partial.co_authors_required {
                config.co_authors_required = required;
            }
            if let Some(period) = partial.auto_update_period {
                config.auto_update_period = period;
            }
            config
        }
        Err(e) => {
            eprintln!(
                "Warning: Failed to parse {}: {}. Ignoring.",
                path.display(),
                e
            );
            config
        }
    }
}

pub trait TrunkConfig {
    fn load(&self) -> Config;

    fn set_co_authors_required(&self, required: bool) -> Result<()>;

    fn set_auto_update_period(&self, period: u64) -> Result<()>;
}

pub struct RepoAwareTrunkConfig<TC: TrunkConfig> {
    inner: TC,
    repo: PathBuf,
}

impl<TC: TrunkConfig> RepoAwareTrunkConfig<TC> {
    pub const fn new(inner: TC, repo: PathBuf) -> Self {
        Self { inner, repo }
    }
}

impl<TC: TrunkConfig> TrunkConfig for RepoAwareTrunkConfig<TC> {
    fn load(&self) -> Config {
        merge_repo_override(self.inner.load(), &self.repo)
    }

    fn set_co_authors_required(&self, required: bool) -> Result<()> {
        self.inner.set_co_authors_required(required)
    }

    fn set_auto_update_period(&self, period: u64) -> Result<()> {
        self.inner.set_auto_update_period(period)
    }
}

pub struct RealTrunkConfig {
    path: PathBuf,
}

impl RealTrunkConfig {
    #[must_use]
    pub const fn new(path: PathBuf) -> Self {
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
        self.write_config(|c| c.co_authors_required = required)
    }

    fn set_auto_update_period(&self, period: u64) -> Result<()> {
        self.write_config(|c| c.auto_update_period = period)
    }
}

impl RealTrunkConfig {
    fn write_config(&self, mutate: impl FnOnce(&mut Config)) -> Result<()> {
        let mut config = self.load();
        mutate(&mut config);

        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(&config)?;
        std::fs::write(&self.path, json)?;
        Ok(())
    }
}

pub struct ConfigHandler<'a, TC: TrunkConfig> {
    config: &'a TC,
}

impl<'a, TC: TrunkConfig> ConfigHandler<'a, TC> {
    pub const fn new(config: &'a TC) -> Self {
        Self { config }
    }
}

impl<TC: TrunkConfig> Handler<(Option<bool>, Option<u64>)> for ConfigHandler<'_, TC> {
    fn handle(
        &self,
        (co_authors_required, auto_update_period): (Option<bool>, Option<u64>),
    ) -> Result<()> {
        if let Some(required) = co_authors_required {
            self.config.set_co_authors_required(required)?;
        }
        if let Some(period) = auto_update_period {
            self.config.set_auto_update_period(period)?;
        }
        Ok(())
    }
}
