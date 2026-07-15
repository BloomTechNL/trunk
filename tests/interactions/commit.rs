use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Interaction};

/// Run `g c` in the actor's repo.
pub struct Commit {
    pub message: &'static str,
    pub co_authors: Vec<&'static str>,
}

impl Interaction for Commit {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .dispatch(
                Commands::Commit {
                    message: Some(self.message.to_string()),
                    co_authors: self.co_authors.iter().map(ToString::to_string).collect(),
                    resolve: false,
                    abort: false,
                },
                &asc.actor_context(actor).working_dir,
            )
            .expect("g c should succeed");
    }
}
