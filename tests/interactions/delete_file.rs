use std::fs;

use crate::abilities::{UseFileSystem, UseGit};
use screenplay::{Actor, Interaction};

pub struct DeleteFile {
    pub name: &'static str,
}

impl Interaction for DeleteFile {
    fn perform_as(&self, actor: &Actor) {
        let _fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        fs::remove_file(git.repo.borrow().join(self.name)).expect("remove file");
    }
}
