#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Timeline {
    events: Rc<RefCell<Vec<String>>>,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            events: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn record(&self, event: impl Into<String>) {
        self.events.borrow_mut().push(event.into());
    }

    pub fn joined(&self) -> String {
        self.events.borrow().join("|")
    }
}
