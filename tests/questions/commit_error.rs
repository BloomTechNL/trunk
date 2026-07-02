use crate::abilities::UseTrunk;
use screenplay::{Actor, Question};

/// Returns the error message from the last [`AttemptCommit`].
///
/// [`AttemptCommit`]: crate::interactions::AttemptCommit
pub struct CommitError;

impl Question<String> for CommitError {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        trunk
            .last_error
            .borrow()
            .clone()
            .expect("expected a commit error but commit succeeded")
    }
}
