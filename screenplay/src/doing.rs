use crate::{Actor, Interaction, Outcome, Question};
use std::panic::{catch_unwind, AssertUnwindSafe};

struct Doing<I> {
    interaction: I,
}

/// Execute an interaction and capture its outcome.
///
/// Returns a [`Question`] that answers [`Outcome::Success`] if the interaction
/// completes without panicking, or [`Outcome::Failure`] with the panic message
/// if it panics.
pub fn doing<I: Interaction>(interaction: I) -> impl Question<Outcome> {
    Doing { interaction }
}

impl<I: Interaction> Question<Outcome> for Doing<I> {
    fn answered_by(&self, actor: &Actor) -> Outcome {
        let result = catch_unwind(AssertUnwindSafe(|| {
            self.interaction.perform_as(actor);
        }));
        match result {
            Ok(()) => Outcome::Success,
            Err(e) => {
                let msg = if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else {
                    "unknown error".to_string()
                };
                Outcome::Failure(msg)
            }
        }
    }
}
