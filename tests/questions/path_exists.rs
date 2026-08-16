use crate::abilities::{AccessScenarioContext, UseFileSystem};
use screenplay::{Ability, Actor, Question};

pub struct PathExists {
    pub name: &'static str,
}

impl Question<bool> for PathExists {
    fn answered_by(&self, actor: &Actor) -> bool {
        let asc = AccessScenarioContext::by(actor);
        let fs = UseFileSystem::by(actor);
        fs.path_exists(&asc.actor_context(actor).working_dir, self.name)
    }
}
