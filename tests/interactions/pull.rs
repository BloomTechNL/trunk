use crate::abilities::{UseGit, UseTrunk};
use screenplay::{Actor, Interaction};

/// Run `g p` in the actor's repo.
pub struct Pull;

impl Interaction for Pull {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk
            .app
            .pull(&git.repo.borrow())
            .expect("g p should succeed");
    }
}
