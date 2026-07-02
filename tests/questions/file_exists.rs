use crate::abilities::{AccessScenarioContext, UseFileSystem};
use screenplay::{Ability, Actor, Question};

pub struct FileExists {
    pub name: &'static str,
}

impl Question<bool> for FileExists {
    fn answered_by(&self, actor: &Actor) -> bool {
        let asc = AccessScenarioContext::by(actor);
        let fs = UseFileSystem::by(actor);
        fs.file_exists(&asc.actor_context(actor).working_dir, self.name)
    }
}
