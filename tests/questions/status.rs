use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Ability, Actor, Question};

/// Ask for the output of `g s` in the actor's repo.
pub struct Status;

impl Question<String> for Status {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk.app.status(&asc.actor_context(actor).working_dir)
    }
}
