use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Ability, Actor, Interaction};

pub struct Update;

impl Interaction for Update {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        let dir = asc.actor_context_mut(actor).working_dir.clone();
        trunk.app.update(&dir).expect("Update should succeed");
    }
}
