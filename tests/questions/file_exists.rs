use crate::abilities::{AccessScenarioContext, UseFileSystem};
use screenplay::{Actor, Question};

pub struct FileExists {
    pub name: &'static str,
}

impl Question<bool> for FileExists {
    fn answered_by(&self, actor: &Actor) -> bool {
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        let fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        fs.file_exists(&asc.actor_context(actor).working_dir, self.name)
    }
}
