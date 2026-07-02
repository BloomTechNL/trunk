use crate::abilities::{AccessScenarioContext, UseFileSystem, UseGit};
use screenplay::{Ability, Actor, Interaction};

pub struct SetUpRemote;

impl Interaction for SetUpRemote {
    fn perform_as(&self, actor: &Actor) {
        let ctx = AccessScenarioContext::by(actor);
        let _fs = UseFileSystem::by(actor);
        UseGit::set_up_remote(ctx.context.borrow().base_dir.path());
    }
}
