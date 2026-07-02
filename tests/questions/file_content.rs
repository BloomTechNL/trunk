use crate::abilities::{AccessScenarioContext, UseFileSystem};
use screenplay::{Ability, Actor, Question};

pub struct FileContent {
    pub name: &'static str,
}

impl Question<String> for FileContent {
    fn answered_by(&self, actor: &Actor) -> String {
        let asc = AccessScenarioContext::by(actor);
        let fs = UseFileSystem::by(actor);
        fs.read_file(&asc.actor_context(actor).working_dir, self.name)
    }
}
