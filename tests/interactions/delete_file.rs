use crate::abilities::{AccessScenarioContext, UseFileSystem};
use screenplay::{Actor, Interaction};

pub struct DeleteFile {
    pub name: &'static str,
}

impl Interaction for DeleteFile {
    fn perform_as(&self, actor: &Actor) {
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        let fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        fs.remove_file(&asc.actor_context(actor).working_dir, self.name);
    }
}
