use crate::abilities::{AccessScenarioContext, UseFileSystem, UseGit};
use crate::common::use_git::clone_repo;
use screenplay::{Actor, Interaction};

/// Clone from `origin.git` into `base_dir/<name>` and record the path as
/// this actor's repo.
pub struct CloneRepo {
    pub name: &'static str,
}

impl Interaction for CloneRepo {
    fn perform_as(&self, actor: &Actor) {
        let ctx = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        let _fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        let path = clone_repo(
            ctx.context.borrow().base_dir.path(),
            self.name,
            "origin.git",
        );
        *git.repo.borrow_mut() = path;
    }
}
