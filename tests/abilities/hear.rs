use std::cell::Cell;
use std::rc::Rc;

use screenplay::Ability;

pub struct Hear {
    pub played: Rc<Cell<bool>>,
}

impl Ability for Hear {}
