use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Interaction};

pub struct ResolveRevert;

impl Interaction for ResolveRevert {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .dispatch(
                Commands::Revert {
                    resolve: true,
                    abort: false,
                    noninteractive: true,
                    hash: None,
                },
                &asc.actor_context(actor).working_dir,
            )
            .expect("g rv --resolve should succeed");
    }
}
