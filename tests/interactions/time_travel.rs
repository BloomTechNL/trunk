use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Interaction};

pub struct TimeTravel {
    pub target: &'static str,
}

impl Interaction for TimeTravel {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .dispatch(
                Commands::TimeTravel {
                    target: self.target.to_string(),
                },
                &asc.actor_context(actor).working_dir,
            )
            .expect("g tt should succeed");
    }
}
