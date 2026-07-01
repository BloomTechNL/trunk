use crate::abilities::{UseGit, UseTrunk};
use screenplay::{Actor, Question};

/// Ask for the output of `g s` in the actor's repo.
pub struct Status;

impl Question<String> for Status {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk.app.status(&git.repo.borrow())
    }
}
