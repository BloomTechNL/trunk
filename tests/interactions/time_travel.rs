use crate::abilities::{UseGit, UseTrunk};
use screenplay::{Actor, Interaction};

pub struct TimeTravel {
    pub target: &'static str,
}

impl Interaction for TimeTravel {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk
            .app
            .time_travel(&git.repo.borrow(), self.target)
            .expect("g tt should succeed");
    }
}
