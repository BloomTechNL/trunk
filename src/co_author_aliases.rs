use std::path::PathBuf;

pub struct RealCoAuthorAliases;

pub trait CoAuthorAliases {
    fn format_alias(&self, alias: &str) -> Option<String>;
}

impl CoAuthorAliases for RealCoAuthorAliases {
    fn format_alias(&self, alias: &str) -> Option<String> {
        let mut aliases = std::collections::HashMap::new();

        let alias_file = if let Ok(path) = std::env::var("TRUNK_ALIASES_PATH") {
            PathBuf::from(path)
        } else {
            let home = std::env::var("HOME").map(PathBuf::from).or_else(|_| {
                std::env::var("USERPROFILE").map(PathBuf::from) // Windows support just in case
            });

            if let Ok(home_path) = home {
                home_path.join(".config/trunk/aliases")
            } else {
                return aliases.get(alias).map(String::from);
            }
        };

        if alias_file.exists() {
            let content = std::fs::read_to_string(alias_file).expect("Could not read file");
            for line in content.lines() {
                if let Some((alias, full)) = line.split_once(':') {
                    aliases.insert(alias.trim().to_string(), full.trim().to_string());
                }
            }
        }
        aliases.get(alias).map(String::from)
    }
}
