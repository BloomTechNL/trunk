use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Actor, Interaction};

/// Run `g p` in the actor's repo.
pub struct Pull;

impl Interaction for Pull {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        trunk
            .app
            .pull(&asc.actor_context(actor).working_dir)
            .expect("g p should succeed");
    }
}
