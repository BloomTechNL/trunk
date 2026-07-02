use crate::abilities::{AccessScenarioContext, UseFileSystem};
use screenplay::{Actor, Interaction};

pub struct WriteFile {
    pub name: &'static str,
    pub content: &'static str,
}

impl Interaction for WriteFile {
    fn perform_as(&self, actor: &Actor) {
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        let fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        fs.write_file(
            &asc.actor_context(actor).working_dir,
            self.name,
            self.content,
        );
    }
}
