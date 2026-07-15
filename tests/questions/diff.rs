use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Question};

pub struct Diff;

impl Question<String> for Diff {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk.dispatch_and_capture(Commands::Diff, &asc.actor_context(actor).working_dir)
    }
}
