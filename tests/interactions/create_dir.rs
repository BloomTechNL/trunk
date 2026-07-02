use std::fs;

use crate::abilities::{UseFileSystem, UseGit};
use screenplay::{Actor, Interaction};

pub struct CreateDir {
    pub name: &'static str,
}

impl Interaction for CreateDir {
    fn perform_as(&self, actor: &Actor) {
        let _fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        fs::create_dir(git.repo.borrow().join(self.name)).expect("create_dir");
    }
}
