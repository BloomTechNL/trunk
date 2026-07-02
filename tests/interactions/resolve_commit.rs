use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Actor, Interaction};

pub struct ResolveCommit;

impl Interaction for ResolveCommit {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        trunk
            .app
            .commit_resolve(&asc.actor_context(actor).working_dir)
            .expect("g c --resolve should succeed");
    }
}
