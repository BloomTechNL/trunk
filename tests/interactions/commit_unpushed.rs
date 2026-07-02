use crate::abilities::{UseFileSystem, UseGit};
use screenplay::{Ability, Actor, Interaction};

pub struct CommitUnpushed;

impl Interaction for CommitUnpushed {
    fn perform_as(&self, actor: &Actor) {
        let _fs = UseFileSystem::by(actor);
        let git = UseGit::by(actor);
        git.commit_file();
    }
}
