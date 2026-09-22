use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Interaction};

pub struct Unmute;

impl Interaction for Unmute {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .dispatch(Commands::Unmute, &asc.actor_context(actor).working_dir)
            .expect("g unmute should succeed");
    }
}
