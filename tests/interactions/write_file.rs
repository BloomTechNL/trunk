use crate::abilities::{UseFileSystem, UseGit};
use screenplay::{Actor, Interaction};

pub struct WriteFile {
    pub name: &'static str,
    pub content: &'static str,
}

impl Interaction for WriteFile {
    fn perform_as(&self, actor: &Actor) {
        let fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        fs.write_file(&git.repo.borrow(), self.name, self.content);
    }
}
