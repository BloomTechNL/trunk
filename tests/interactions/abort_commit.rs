use crate::abilities::{UseGit, UseTrunk};
use screenplay::{Actor, Interaction};

pub struct AbortCommit;

impl Interaction for AbortCommit {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk
            .app
            .commit_abort(&git.repo.borrow())
            .expect("g c --abort should succeed");
    }
}
