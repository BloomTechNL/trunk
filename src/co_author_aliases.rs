use anyhow::Result;
use std::path::PathBuf;

pub struct RealCoAuthorAliases {
    path: PathBuf,
}

impl RealCoAuthorAliases {
    #[must_use]
    pub const fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

pub trait CoAuthorAliases {
    fn format_alias(&self, alias: &str) -> Option<String>;

    fn add_alias(&self, alias: &str, name: &str, email: &str) -> Result<()>;
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

    fn add_alias(&self, alias: &str, name: &str, email: &str) -> Result<()> {
        use std::fs::OpenOptions;
        use std::io::Write;
        let content = format!("{alias}:{name} <{email}>\n");
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
