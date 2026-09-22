use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Interaction};

pub struct Mute;

impl Interaction for Mute {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .dispatch(Commands::Mute, &asc.actor_context(actor).working_dir)
            .expect("g mute should succeed");
    }
}
