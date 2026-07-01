use crate::abilities::{UseGit, UseTrunk};
use screenplay::{Actor, Interaction};

/// Run `g c` in the actor's repo.
pub struct Commit {
    pub message: &'static str,
    pub co_authors: Vec<&'static str>,
}

impl Interaction for Commit {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk
            .app
            .commit(
                &git.repo.borrow(),
                self.message,
                self.co_authors.iter().map(|s| *s).collect(),
            )
            .expect("g c should succeed");
    }
}
