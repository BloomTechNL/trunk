use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Interaction};

/// Add a co-author alias via `g add-alias`.
pub struct AddAlias {
    pub alias: &'static str,
    pub name: &'static str,
    pub email: &'static str,
}

impl Interaction for AddAlias {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        let path = asc.base_dir();
        trunk
            .dispatch(
                Commands::AddAlias {
                    alias: self.alias.to_string(),
                    name: self.name.to_string(),
                    email: self.email.to_string(),
                },
                &path,
            )
            .expect("add_alias should succeed");
    }
}
