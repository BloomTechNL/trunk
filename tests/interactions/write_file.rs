use crate::abilities::{UseFileSystem, UseGit};
use crate::common::write_file::write_file;
use screenplay::{Actor, Interaction};

/// Write a file into the actor's repo.
pub struct WriteFile {
    pub name: &'static str,
    pub content: &'static str,
}

impl Interaction for WriteFile {
    fn perform_as(&self, actor: &Actor) {
        let _fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        write_file(&git.repo.borrow(), self.name, self.content);
    }
}
