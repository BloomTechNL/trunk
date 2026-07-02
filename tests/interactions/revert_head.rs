use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Actor, Interaction};

pub struct RevertHead;

impl Interaction for RevertHead {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        let hash = &trunk
            .app
            .commit_hashes(&asc.actor_context(actor).working_dir)[0];
        trunk
            .app
            .revert(&asc.actor_context(actor).working_dir, hash)
            .expect("g rv should succeed");
    }
}
