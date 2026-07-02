use crate::abilities::UseTrunk;
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
        trunk
            .app
            .add_alias(self.alias, self.name, self.email)
            .expect("add_alias should succeed");
    }
}
