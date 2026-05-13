use g_cli::CoAuthorAliases;
use std::collections::HashMap;
use std::path::PathBuf;

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
}
