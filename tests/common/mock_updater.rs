use std::cell::Cell;
use std::rc::Rc;

use g_cli::{Clock, LastUpdateStore, Updater};

use crate::common::mock_clock::MockClock;
use crate::common::mock_last_update_store::MockLastUpdateStore;

#[derive(Clone)]
pub struct MockUpdater {
    update_count: Rc<Cell<u32>>,
    clock: MockClock,
    last_update_store: MockLastUpdateStore,
}

impl MockUpdater {
    pub fn new() -> Self {
        Self {
            update_count: Rc::new(Cell::new(0)),
            clock: MockClock::new(),
            last_update_store: MockLastUpdateStore::new(),
        }
    }

    pub fn inner(&self) -> Rc<Cell<u32>> {
        self.update_count.clone()
    }

    pub fn update_count(&self) -> u32 {
        self.update_count.get()
    }

    pub fn clock_inner(&self) -> Rc<Cell<u64>> {
        self.clock.inner()
    }
}

impl Updater for MockUpdater {
    fn update(&self) -> anyhow::Result<()> {
        self.update_count.set(self.update_count.get() + 1);
        Ok(())
    }

    fn clock(&self) -> &dyn Clock {
        &self.clock
    }

    fn last_update_store(&self) -> &dyn LastUpdateStore {
        &self.last_update_store
    }
}
