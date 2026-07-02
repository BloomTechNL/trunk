use crate::abilities::{AccessScenarioContext, UseTrunk};
use screenplay::{Actor, Interaction};

/// Configure trunk settings via `g config`.
pub struct Config {
    pub co_authors_required: Option<bool>,
}

impl Interaction for Config {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        trunk
            .app
            .config(
                &asc.actor_context(actor).working_dir,
                self.co_authors_required,
            )
            .expect("g config should succeed");
    }
}
