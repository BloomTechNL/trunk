use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Actor, Question};

/// Ask for the output of `g s` in the actor's repo.
pub struct Status;

impl Question<String> for Status {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        trunk.app.status(&asc.actor_context(actor).working_dir)
    }
}
