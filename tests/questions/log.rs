use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Question};

/// Ask for the output of `g l` in the actor's repo.
pub struct Log;

impl Question<String> for Log {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk.dispatch_and_capture(Commands::Log, &asc.actor_context(actor).working_dir)
    }
}
