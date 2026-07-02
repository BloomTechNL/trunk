use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Actor, Interaction};

pub struct AbortCommit;

impl Interaction for AbortCommit {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        trunk
            .app
            .commit_abort(&asc.actor_context(actor).working_dir)
            .expect("g c --abort should succeed");
    }
}
