use crate::abilities::UseGit;
use screenplay::{Actor, Question};

pub struct FileExists {
    pub name: &'static str,
}

impl Question<bool> for FileExists {
    fn answered_by(&self, actor: &Actor) -> bool {
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        git.repo.borrow().join(self.name).exists()
    }
}
