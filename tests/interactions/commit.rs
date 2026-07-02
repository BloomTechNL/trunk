use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Actor, Interaction};

/// Run `g c` in the actor's repo.
pub struct Commit {
    pub message: &'static str,
    pub co_authors: Vec<&'static str>,
}

impl Interaction for Commit {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        trunk
            .app
            .commit(
                &asc.actor_context(actor).working_dir,
                self.message,
                self.co_authors.iter().map(|s| *s).collect(),
            )
            .expect("g c should succeed");
    }
}
