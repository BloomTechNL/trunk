use crate::abilities::{UseGit, UseTrunk};
use screenplay::{Actor, Question};

/// Ask for the output of `g l` in the actor's repo.
pub struct Log;

impl Question<String> for Log {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk.app.log(&git.repo.borrow())
    }
}
