use crate::abilities::{UseGit, UseTrunk};
use screenplay::{Actor, Interaction};

/// Run `g c` in the actor's repo, capturing any error instead of panicking.
///
/// Use this when the test *expects* the commit to fail.  The error message is
/// stored in [`UseTrunk::last_error`] and can be inspected with the
/// [`CommitError`](crate::questions::CommitError) question.
pub struct AttemptCommit {
    pub message: &'static str,
    pub co_authors: Vec<&'static str>,
}

impl Interaction for AttemptCommit {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        let result = trunk.app.commit(
            &git.repo.borrow(),
            self.message,
            self.co_authors.iter().map(|s| *s).collect(),
        );
        trunk
            .last_error
            .replace(result.err().map(|e| e.to_string()));
    }
}
