use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Ability, Actor, Question};

pub struct Diff;

impl Question<String> for Diff {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk.app.diff(&asc.actor_context(actor).working_dir)
    }
}
