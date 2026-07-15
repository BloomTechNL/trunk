#![allow(dead_code)]
use std::cell::Cell;
use std::rc::Rc;

use g_cli::Clock;

#[derive(Clone)]
pub struct MockClock {
    time: Rc<Cell<u64>>,
}

impl MockClock {
    pub fn new() -> Self {
        Self {
            time: Rc::new(Cell::new(0)),
        }
    }

    pub fn inner(&self) -> Rc<Cell<u64>> {
        self.time.clone()
    }
}

impl Clock for MockClock {
    fn now_secs(&self) -> u64 {
        self.time.get()
    }
}
