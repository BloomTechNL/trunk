use crate::abilities::{UseFileSystem, UseGit};
use crate::common::use_git::initial_commit;
use screenplay::{Actor, Interaction};

/// Make an initial commit (README) and push to origin so other actors can
/// see it when they clone.
pub struct InitialCommit;

impl Interaction for InitialCommit {
    fn perform_as(&self, actor: &Actor) {
        let _fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        initial_commit(&git.repo.borrow());
    }
}
