use crate::abilities::{AccessScenarioContext, UseFileSystem};
use screenplay::{Actor, Interaction};

pub struct CreateDir {
    pub name: &'static str,
}

impl Interaction for CreateDir {
    fn perform_as(&self, actor: &Actor) {
        let asc = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        let fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let working_dir = &asc.actor_context(actor).working_dir;
        fs.create_dir(working_dir, self.name);
    }
}
