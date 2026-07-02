use crate::abilities::{UseFileSystem, UseGit};
use crate::common::use_git::commit_file;
use screenplay::{Actor, Interaction};

pub struct CommitUnpushed;

impl Interaction for CommitUnpushed {
    fn perform_as(&self, actor: &Actor) {
        let _fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        commit_file(&git.repo.borrow());
    }
}
