use anyhow::Result;
use std::path::PathBuf;

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
        let content = format!("{}:{} <{}>\n", alias, name, email);
        use std::fs::OpenOptions;
        use std::io::Write;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
