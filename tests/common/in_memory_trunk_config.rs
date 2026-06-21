use g_cli::config::{Config, TrunkConfig};
use std::cell::RefCell;

pub struct InMemoryTrunkConfig {
    config: RefCell<Config>,
}

impl InMemoryTrunkConfig {
    pub fn new() -> Self {
        Self {
            config: RefCell::new(Config::default()),
        }
    }
}

impl TrunkConfig for InMemoryTrunkConfig {
    fn load(&self) -> Config {
        self.config.borrow().clone()
    }

    fn set_co_authors_required(&self, required: bool) -> anyhow::Result<()> {
        self.config.borrow_mut().co_authors_required = required;
        Ok(())
    }
}
