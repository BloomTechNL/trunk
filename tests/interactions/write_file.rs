use crate::abilities::{AccessScenarioContext, UseFileSystem};
use screenplay::{Ability, Actor, Interaction};

pub struct WriteFile {
    pub name: &'static str,
    pub content: &'static str,
}

impl Interaction for WriteFile {
    fn perform_as(&self, actor: &Actor) {
        let asc = AccessScenarioContext::by(actor);
        let fs = UseFileSystem::by(actor);
        fs.write_file(
            &asc.actor_context(actor).working_dir,
            self.name,
            self.content,
        );
    }
}
