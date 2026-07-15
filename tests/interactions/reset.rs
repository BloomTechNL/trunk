use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Interaction};

pub struct Reset;

impl Interaction for Reset {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .dispatch(Commands::Reset, &asc.actor_context(actor).working_dir)
            .expect("g r should succeed");
    }
}
