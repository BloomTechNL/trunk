use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Interaction};

pub struct ResolveCommit;

impl Interaction for ResolveCommit {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .dispatch(
                Commands::Commit {
                    message: None,
                    co_authors: vec![],
                    resolve: true,
                    abort: false,
                },
                &asc.actor_context(actor).working_dir,
            )
            .expect("g c --resolve should succeed");
    }
}
