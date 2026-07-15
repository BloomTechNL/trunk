#![allow(dead_code)]
use std::cell::Cell;
use std::rc::Rc;

use g_cli::LastUpdateStore;

#[derive(Clone)]
pub struct MockLastUpdateStore {
    value: Rc<Cell<Option<u64>>>,
}

impl MockLastUpdateStore {
    pub fn new() -> Self {
        Self {
            value: Rc::new(Cell::new(None)),
        }
    }
}

impl LastUpdateStore for MockLastUpdateStore {
    fn read(&self) -> Option<u64> {
        self.value.get()
    }

    fn write(&self, timestamp: u64) -> anyhow::Result<()> {
        self.value.set(Some(timestamp));
        Ok(())
    }
}
