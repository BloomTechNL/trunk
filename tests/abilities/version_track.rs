use std::cell::Cell;
use std::rc::Rc;

use screenplay::Ability;

pub struct VersionTrack {
    pub count: Rc<Cell<u32>>,
}

impl Ability for VersionTrack {}
