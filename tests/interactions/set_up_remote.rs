use crate::abilities::{AccessScenarioContext, UseFileSystem};
use crate::common::use_git::set_up_remote;
use screenplay::{Actor, Interaction};

/// Create a bare `origin.git` remote inside the shared base directory.
pub struct SetUpRemote;

impl Interaction for SetUpRemote {
    fn perform_as(&self, actor: &Actor) {
        let ctx = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        let _fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        set_up_remote(ctx.context.borrow().base_dir.path());
    }
}
