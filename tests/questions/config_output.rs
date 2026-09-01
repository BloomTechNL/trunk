use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Question};

pub struct ConfigOutput;

impl Question<String> for ConfigOutput {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk.dispatch_and_capture(
            Commands::Config {
                co_authors_required: None,
                auto_update_period: None,
                local: false,
            },
            &asc.actor_context(actor).working_dir,
        )
    }
}
