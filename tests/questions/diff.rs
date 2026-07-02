use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Actor, Question};

pub struct Diff;

impl Question<String> for Diff {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        trunk.app.diff(&asc.actor_context(actor).working_dir)
    }
}
