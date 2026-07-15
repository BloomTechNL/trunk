use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Ability, Actor, Interaction};

/// Run `g p` in the actor's repo.
pub struct Pull;

impl Interaction for Pull {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .pull(&asc.actor_context(actor).working_dir)
            .expect("g p should succeed");
    }
}
