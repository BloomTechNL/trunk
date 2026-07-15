use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Ability, Actor, Interaction};

pub struct ResolveCommit;

impl Interaction for ResolveCommit {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .commit_resolve(&asc.actor_context(actor).working_dir)
            .expect("g c --resolve should succeed");
    }
}
