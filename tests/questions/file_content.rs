use crate::abilities::{UseFileSystem, UseGit};
use screenplay::{Actor, Question};

pub struct FileContent {
    pub name: &'static str,
}

impl Question<String> for FileContent {
    fn answered_by(&self, actor: &Actor) -> String {
        let fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        fs.read_file(&git.repo.borrow(), self.name)
    }
}
