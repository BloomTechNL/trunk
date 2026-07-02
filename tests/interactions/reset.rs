use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Ability, Actor, Interaction};

pub struct Reset;

impl Interaction for Reset {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .app
            .reset(&asc.actor_context(actor).working_dir)
            .expect("g r should succeed");
    }
}
