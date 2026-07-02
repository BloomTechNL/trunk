use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Ability, Actor, Interaction};

pub struct RevertHead;

impl Interaction for RevertHead {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        let hash = &trunk
            .app
            .commit_hashes(&asc.actor_context(actor).working_dir)[0];
        trunk
            .app
            .revert(&asc.actor_context(actor).working_dir, hash)
            .expect("g rv should succeed");
    }
}
