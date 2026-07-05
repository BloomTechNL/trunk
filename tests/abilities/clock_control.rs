use std::cell::Cell;
use std::rc::Rc;

use screenplay::Ability;

pub struct ClockControl {
    pub time: Rc<Cell<u64>>,
}

impl Ability for ClockControl {}
