use crate::abilities::{AccessScenarioContext, UseFileSystem, UseGit, UseTrunk};
use screenplay::{Actor, Interaction};

/// Clone from `origin.git` into `base_dir/<name>` using `g clone` and record
/// the path as this actor's repo.
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
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");

        let base = ctx.context.borrow().base_dir.path().to_path_buf();
        let source = base.join("origin.git");

        trunk
            .app
            .clone(
                &base,
                source.to_str().expect("source path must be valid UTF-8"),
                self.name,
            )
            .expect("g clone should succeed");

        *git.repo.borrow_mut() = base.join(self.name);
    }
}
