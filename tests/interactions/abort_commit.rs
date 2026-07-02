use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Ability, Actor, Interaction};

pub struct AbortCommit;

impl Interaction for AbortCommit {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .app
            .commit_abort(&asc.actor_context(actor).working_dir)
            .expect("g c --abort should succeed");
    }
}
