use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Interaction};

pub struct Fart;

impl Interaction for Fart {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .dispatch(Commands::Fart, &asc.actor_context(actor).working_dir)
            .expect("Fart should succeed");
    }
}
