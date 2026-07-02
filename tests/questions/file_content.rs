use crate::abilities::{AccessScenarioContext, UseFileSystem};
use screenplay::{Actor, Question};

pub struct FileContent {
    pub name: &'static str,
}

impl Question<String> for FileContent {
    fn answered_by(&self, actor: &Actor) -> String {
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        let fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        fs.read_file(&asc.actor_context(actor).working_dir, self.name)
    }
}
