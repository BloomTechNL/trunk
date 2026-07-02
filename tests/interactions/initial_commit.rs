use crate::abilities::{UseFileSystem, UseGit};
use screenplay::{Ability, Actor, Interaction};

pub struct InitialCommit;

impl Interaction for InitialCommit {
    fn perform_as(&self, actor: &Actor) {
        let _fs = UseFileSystem::by(actor);
        let git = UseGit::by(actor);
        git.initial_commit();
    }
}
