use crate::abilities::{UseGit, UseTrunk};
use screenplay::{Actor, Question};

pub struct Diff;

impl Question<String> for Diff {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk.app.diff(&git.repo.borrow())
    }
}
