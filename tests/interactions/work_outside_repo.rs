use crate::abilities::AccessScenarioContext;
use screenplay::{Ability, Actor, Interaction};

pub struct WorkOutsideRepo;

impl Interaction for WorkOutsideRepo {
    fn perform_as(&self, actor: &Actor) {
        let asc = AccessScenarioContext::by(actor);
        let base_dir = asc.base_dir();
        asc.actor_context_mut(actor).working_dir = base_dir;
    }
}
