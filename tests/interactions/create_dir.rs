use crate::abilities::{AccessScenarioContext, UseFileSystem};
use screenplay::{Ability, Actor, Interaction};

pub struct CreateDir {
    pub name: &'static str,
}

impl Interaction for CreateDir {
    fn perform_as(&self, actor: &Actor) {
        let asc = AccessScenarioContext::by(actor);
        let fs = UseFileSystem::by(actor);
        let working_dir = &asc.actor_context(actor).working_dir;
        fs.create_dir(working_dir, self.name);
    }
}
