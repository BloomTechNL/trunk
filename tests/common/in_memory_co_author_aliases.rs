#![allow(dead_code)]
use g_cli::CoAuthorAliases;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct InMemoryCoAuthorAliases {
    pub aliases: Rc<RefCell<HashMap<String, String>>>,
}

impl InMemoryCoAuthorAliases {
    pub fn new() -> Self {
        Self {
            aliases: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl CoAuthorAliases for InMemoryCoAuthorAliases {
    fn format_alias(&self, alias: &str) -> Option<String> {
        self.aliases
            .borrow()
            .get(alias)
            .and_then(|x| x.split_once(':').map(|(_, after)| after.to_string()))
    }

    fn add_alias(&self, alias: &str, name: &str, email: &str) -> anyhow::Result<()> {
        let content = format!("{alias}:{name} <{email}>");
        self.aliases.borrow_mut().insert(alias.to_string(), content);
        Ok(())
    }
}
