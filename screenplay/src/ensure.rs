//! [`Ensure`] — combine a [`Question`] with an [`Expectation`] into an [`Interaction`].

use std::marker::PhantomData;

use crate::actor::Actor;
use crate::expectation::Expectation;
use crate::interaction::Interaction;
use crate::question::Question;

/// **Ensure** combines a [`Question`] with an [`Expectation`] and turns the
/// pair into an [`Interaction`]. When performed, it calls
/// [`answered_by`](Question::answered_by) on the question, then
/// [`test`](Expectation::test) on the expectation. If the test fails,
/// [`message`](Expectation::message) is used in a panic message.
///
/// Construct via [`Ensure::that`]:
///
/// ```rust
/// use screenplay::*;
///
/// struct TheAnswer;
/// impl Question<i32> for TheAnswer {
///     fn answered_by(&self, _actor: &Actor) -> i32 { 42 }
/// }
///
/// let actor = Actor::new();
///
/// // This passes (42 == 42):
/// actor.attempts_to((Ensure::that(TheAnswer, equals(42)),));
///
/// // This would panic with a descriptive message:
/// // actor.attempts_to((Ensure::that(TheAnswer, equals(99)),));
/// ```
pub struct Ensure<Q, E, T> {
    question: Q,
    expectation: E,
    _marker: PhantomData<T>,
}

impl<Q, E, T> Ensure<Q, E, T> {
    /// Create an `Ensure` interaction from a question and an expectation.
    ///
    /// The type `T` is inferred from the question and expectation — both
    /// must agree on the same `T`.
    pub fn that(question: Q, expectation: E) -> Self
    where
        Q: Question<T>,
        E: Expectation<T>,
    {
        Ensure {
            question,
            expectation,
            _marker: PhantomData,
        }
    }
}

impl<Q: Question<T>, E: Expectation<T>, T> Interaction for Ensure<Q, E, T> {
    fn perform_as(&self, actor: &Actor) {
        let answer = self.question.answered_by(actor);
        if !self.expectation.test(&answer) {
            panic!("{}", self.expectation.message(&answer));
        }
    }
}
