use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Actor, Interaction};

pub struct TimeTravel {
    pub target: &'static str,
}

impl Interaction for TimeTravel {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        trunk
            .app
            .time_travel(&asc.actor_context(actor).working_dir, self.target)
            .expect("g tt should succeed");
    }
}
