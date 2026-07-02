use crate::abilities::{AccessScenarioContext, UseFileSystem, UseGit};
use screenplay::{Actor, Interaction};

pub struct SetUpRemote;

impl Interaction for SetUpRemote {
    fn perform_as(&self, actor: &Actor) {
        let ctx = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        let _fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        UseGit::set_up_remote(ctx.context.borrow().base_dir.path());
    }
}
