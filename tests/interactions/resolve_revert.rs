use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Ability, Actor, Interaction};

pub struct ResolveRevert;

impl Interaction for ResolveRevert {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .app
            .revert_resolve(&asc.actor_context(actor).working_dir)
            .expect("g rv --resolve should succeed");
    }
}
