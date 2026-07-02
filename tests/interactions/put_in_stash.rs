use crate::abilities::{UseFileSystem, UseGit};
use screenplay::{Actor, Interaction};

pub struct PutInStash;

impl Interaction for PutInStash {
    fn perform_as(&self, actor: &Actor) {
        let _fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        git.put_something_in_stash();
    }
}
