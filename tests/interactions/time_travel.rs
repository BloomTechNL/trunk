use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Ability, Actor, Interaction};

pub struct TimeTravel {
    pub target: &'static str,
}

impl Interaction for TimeTravel {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .time_travel(&asc.actor_context(actor).working_dir, self.target)
            .expect("g tt should succeed");
    }
}
