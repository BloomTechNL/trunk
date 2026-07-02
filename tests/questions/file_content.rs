use std::fs;

use crate::abilities::UseGit;
use screenplay::{Actor, Question};

pub struct FileContent {
    pub name: &'static str,
}

impl Question<String> for FileContent {
    fn answered_by(&self, actor: &Actor) -> String {
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        fs::read_to_string(git.repo.borrow().join(self.name)).expect("read file")
    }
}
