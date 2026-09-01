use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Interaction};

pub struct Commit {
    pub message: Option<&'static str>,
    pub co_authors: Vec<&'static str>,
}

impl Interaction for Commit {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        let result = trunk.dispatch(
            Commands::Commit {
                message: self.message.map(str::to_string),
                co_authors: self.co_authors.iter().map(ToString::to_string).collect(),
                resolve: false,
                abort: false,
            },
            &asc.actor_context(actor).working_dir,
        );
        if self.message.is_some() {
            result.expect("g c should succeed");
        } else {
            result.expect("g c without message should fail");
        }
    }
}
