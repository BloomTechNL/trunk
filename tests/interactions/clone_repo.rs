use crate::abilities::{AccessScenarioContext, UseFileSystem, UseGit};
use screenplay::{Ability, Actor, Interaction};

pub struct CloneRepo {
    pub name: &'static str,
}

impl Interaction for CloneRepo {
    fn perform_as(&self, actor: &Actor) {
        let asc = AccessScenarioContext::by(actor);
        let _fs = UseFileSystem::by(actor);
        let git = UseGit::by(actor);
        let path = UseGit::clone_repo(
            asc.context.borrow().base_dir.path(),
            self.name,
            "origin.git",
        );
        *git.repo.borrow_mut() = path.clone();
        asc.actor_context_mut(actor).working_dir = path;
    }
}
