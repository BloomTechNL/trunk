use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Actor, Question};

/// Ask for the output of `g l` in the actor's repo.
pub struct Log;

impl Question<String> for Log {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        trunk.app.log(&asc.actor_context(actor).working_dir)
    }
}
