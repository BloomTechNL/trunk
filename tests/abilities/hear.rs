use std::cell::Cell;
use std::rc::Rc;

use screenplay::Ability;

use crate::common::timeline::Timeline;

pub struct Hear {
    pub played: Rc<Cell<bool>>,
    pub music_started: Rc<Cell<bool>>,
    pub music_playing: Rc<Cell<bool>>,
    pub timeline: Timeline,
}

impl Ability for Hear {}
