use std::path::{Path, PathBuf};

pub struct RealCoAuthorAliases {
    path: PathBuf,
}

impl RealCoAuthorAliases {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

pub trait CoAuthorAliases {
    fn format_alias(&self, alias: &str) -> Option<String>;
    fn path(&self) -> &Path;
}

impl CoAuthorAliases for RealCoAuthorAliases {
    fn format_alias(&self, alias: &str) -> Option<String> {
        let mut aliases = std::collections::HashMap::new();

        if self.path.exists() {
            let content = std::fs::read_to_string(&self.path).expect("Could not read file");
            for line in content.lines() {
                if let Some((alias, full)) = line.split_once(':') {
                    aliases.insert(alias.trim().to_string(), full.trim().to_string());
                }
            }
        }
        aliases.get(alias).map(String::from)
    }

    fn path(&self) -> &Path {
        &self.path
    }
}
