use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Interaction};

pub struct Config {
    pub co_authors_required: Option<bool>,
    pub auto_update_period: Option<u64>,
    pub local: bool,
}

impl Interaction for Config {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        trunk
            .dispatch(
                Commands::Config {
                    co_authors_required: self.co_authors_required,
                    auto_update_period: self.auto_update_period,
                    local: self.local,
                },
                &asc.actor_context(actor).working_dir,
            )
            .expect("g config should succeed");
    }
}
