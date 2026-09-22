#![allow(dead_code)]
use std::cell::Cell;
use std::rc::Rc;

use g_cli::{HoldMusic, Playback};

use super::timeline::Timeline;

#[derive(Clone)]
pub struct MockHoldMusic {
    started: Rc<Cell<bool>>,
    playing: Rc<Cell<bool>>,
    timeline: Timeline,
}

impl MockHoldMusic {
    pub fn new(timeline: Timeline) -> Self {
        Self {
            started: Rc::new(Cell::new(false)),
            playing: Rc::new(Cell::new(false)),
            timeline,
        }
    }

    pub fn started_flag(&self) -> Rc<Cell<bool>> {
        self.started.clone()
    }

    pub fn playing_flag(&self) -> Rc<Cell<bool>> {
        self.playing.clone()
    }
}

struct MockPlayback {
    playing: Rc<Cell<bool>>,
    timeline: Timeline,
}

impl Playback for MockPlayback {}

impl Drop for MockPlayback {
    fn drop(&mut self) {
        self.playing.set(false);
        self.timeline.record("music stop");
    }
}

impl HoldMusic for MockHoldMusic {
    fn start(&self) -> anyhow::Result<Box<dyn Playback>> {
        self.started.set(true);
        self.playing.set(true);
        self.timeline.record("music start");
        Ok(Box::new(MockPlayback {
            playing: self.playing.clone(),
            timeline: self.timeline.clone(),
        }))
    }
}
