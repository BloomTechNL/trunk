use crate::abilities::{UseFileSystem, UseGit};
use screenplay::{Actor, Question};

pub struct FileExists {
    pub name: &'static str,
}

impl Question<bool> for FileExists {
    fn answered_by(&self, actor: &Actor) -> bool {
        let fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        fs.file_exists(&git.repo.borrow(), self.name)
    }
}
