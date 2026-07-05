use crate::abilities::ClockControl;
use screenplay::{Ability, Actor, Interaction};

pub struct AdvanceTime {
    pub secs: u64,
}

impl Interaction for AdvanceTime {
    fn perform_as(&self, actor: &Actor) {
        let clock = ClockControl::by(actor);
        clock.time.set(clock.time.get() + self.secs);
    }
}
