use crate::abilities::{UseFileSystem, UseGit};
use screenplay::{Ability, Actor, Interaction};

pub struct PutInStash;

impl Interaction for PutInStash {
    fn perform_as(&self, actor: &Actor) {
        let _fs = UseFileSystem::by(actor);
        let git = UseGit::by(actor);
        git.put_something_in_stash();
    }
}
