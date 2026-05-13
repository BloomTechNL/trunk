use g_cli::CoAuthorAliases;
use std::collections::HashMap;

pub struct InMemoryCoAuthorAliases {
    pub aliases: HashMap<String, String>,
}

impl InMemoryCoAuthorAliases {
    pub fn new() -> Self {
        InMemoryCoAuthorAliases {
            aliases: HashMap::new(),
        }
    }
}

impl CoAuthorAliases for InMemoryCoAuthorAliases {
    fn format_alias(&self, alias: &str) -> Option<String> {
        self.aliases
            .get(alias)
            .and_then(|x| x.split_once(':').map(|(_, after)| after.to_string()))
    }

    fn add_alias(&mut self, alias: &str, name: &str, email: &str) -> anyhow::Result<()> {
        let content = format!("{}:{} <{}>", alias, name, email);
        self.aliases.insert(alias.to_string(), content);
        Ok(())
    }
}
