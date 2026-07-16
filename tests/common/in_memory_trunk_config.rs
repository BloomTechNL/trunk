#![allow(dead_code)]
use g_cli::config::{Config, TrunkConfig};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct InMemoryTrunkConfig {
    config: Rc<RefCell<Config>>,
}

impl InMemoryTrunkConfig {
    pub fn new() -> Self {
        Self {
            config: Rc::new(RefCell::new(Config::default())),
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

    fn set_auto_update_period(&self, period: u64) -> anyhow::Result<()> {
        self.config.borrow_mut().auto_update_period = period;
        Ok(())
    }
}
