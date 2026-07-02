use crate::abilities::{AccessScenarioContext, UseFileSystem};
use screenplay::{Ability, Actor, Interaction};

pub struct DeleteFile {
    pub name: &'static str,
}

impl Interaction for DeleteFile {
    fn perform_as(&self, actor: &Actor) {
        let asc = AccessScenarioContext::by(actor);
        let fs = UseFileSystem::by(actor);
        fs.remove_file(&asc.actor_context(actor).working_dir, self.name);
    }
}
