use crate::abilities::{AccessScenarioContext, UseFileSystem};
use screenplay::{Ability, Actor, Interaction};

pub struct CreateSymlink {
    pub name: &'static str,
    pub target: &'static str,
}

impl Interaction for CreateSymlink {
    fn perform_as(&self, actor: &Actor) {
        let asc = AccessScenarioContext::by(actor);
        let fs = UseFileSystem::by(actor);
        fs.create_symlink(
            &asc.actor_context(actor).working_dir,
            self.name,
            self.target,
        );
    }
}
