use crate::abilities::{UseGit, UseTrunk};
use screenplay::{Actor, Interaction};

/// Configure trunk settings via `g config`.
pub struct Config {
    pub co_authors_required: Option<bool>,
}

impl Interaction for Config {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk
            .app
            .config(&git.repo.borrow(), self.co_authors_required)
            .expect("g config should succeed");
    }
}
