use crate::abilities::{UseFileSystem, UseGit};
use screenplay::{Actor, Interaction};

pub struct InitialCommit;

impl Interaction for InitialCommit {
    fn perform_as(&self, actor: &Actor) {
        let _fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        git.initial_commit();
    }
}
