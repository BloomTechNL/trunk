use std::cell::Cell;
use std::rc::Rc;

use g_cli::Updater;

#[derive(Clone)]
pub struct MockUpdater {
    update_count: Rc<Cell<u32>>,
}

impl MockUpdater {
    pub fn new() -> Self {
        Self {
            update_count: Rc::new(Cell::new(0)),
        }
    }

    pub fn inner(&self) -> Rc<Cell<u32>> {
        self.update_count.clone()
    }

    pub fn update_count(&self) -> u32 {
        self.update_count.get()
    }
}

impl Updater for MockUpdater {
    fn update(&self) -> anyhow::Result<()> {
        self.update_count.set(self.update_count.get() + 1);
        Ok(())
    }
}
