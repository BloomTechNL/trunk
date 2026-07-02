//! # Screenplay Pattern — minimal framework for Rust
//!
//! A lightweight implementation of the [Screenplay Pattern](https://serenity-js.org/handbook/design/screenplay-pattern.html),
//! inspired by Serenity/JS. Provides the core abstractions (Actor, Ability,
//! Interaction, Question, Expectation, Ensure) with zero dependencies.
//!
//! The framework ships **no concrete abilities** — library users define their own
//! (e.g. `WebBrowser`, `UseCli`, `Database`).
//!
//! ## Quick example
//!
//! ```rust
//! use screenplay::*;
//!
//! // ── User-defined ability ──
//! struct Counter(usize);
//! impl Ability for Counter {}
//!
//! // ── User-defined interaction ──
//! struct Increment;
//! impl Interaction for Increment {
//!     fn perform_as(&self, actor: &Actor) {
//!         // In real code you would fetch the ability and mutate state:
//!         // actor.ability::<Counter>().unwrap().0 += 1;
//!     }
//! }
//!
//! // ── User-defined question ──
//! struct TheAnswer;
//! impl Question<i32> for TheAnswer {
//!     fn answered_by(&self, _actor: &Actor) -> i32 {
//!         42
//!     }
//! }
//!
//! // ── Wire everything together ──
//! let user = Actor::new().who_can(Counter(0));
//!
//! // Single interaction
//! user.attempts_to((Increment,));
//!
//! // Multiple interactions + assertions
//! user.attempts_to((
//!     Increment,
//!     Ensure::that(TheAnswer, equals(42)),
//!     Ensure::that(TheAnswer, is_greater_than(0)),
//! ));
//! ```

// -- Modules ---------------------------------------------------------------

mod ability;
mod actor;
mod contains;
mod does_not_contain;
mod doing;
mod ensure;
mod expectation;
mod fails;
mod interaction;
mod outcome;
mod question;

// -- Public API re-exports -------------------------------------------------

pub use ability::Ability;
pub use actor::Actor;
pub use ensure::Ensure;
pub use contains::contains;
pub use does_not_contain::does_not_contain;
pub use doing::doing;
pub use expectation::{equals, is_false, is_greater_than, is_true, Expectation};
pub use fails::fails;
pub use interaction::{Interaction, Interactions};
pub use outcome::Outcome;
pub use question::Question;

// -- Tests -----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    // Dummy ability for tests
    struct Logging;
    impl Ability for Logging {}

    // Dummy interaction that records it was called
    struct FlagInteraction<'a> {
        flag: &'a Cell<bool>,
    }
    impl Interaction for FlagInteraction<'_> {
        fn perform_as(&self, _actor: &Actor) {
            self.flag.set(true);
        }
    }

    struct ConstantQuestion<T> {
        value: T,
    }
    impl<T: Clone> Question<T> for ConstantQuestion<T> {
        fn answered_by(&self, _actor: &Actor) -> T {
            self.value.clone()
        }
    }

    // ── Actor tests ──

    #[test]
    fn actor_starts_with_no_abilities() {
        let actor = Actor::new();
        assert!(actor.ability::<Logging>().is_none());
    }

    #[test]
    fn who_can_adds_and_retrieves_ability() {
        let actor = Actor::new().who_can(Logging);
        assert!(actor.ability::<Logging>().is_some());
    }

    #[test]
    fn who_can_replaces_same_type_ability() {
        struct Counter(i32);
        impl Ability for Counter {}

        let actor = Actor::new().who_can(Counter(1)).who_can(Counter(99));

        let counter = actor.ability::<Counter>().unwrap();
        assert_eq!(counter.0, 99);
    }

    #[test]
    fn actor_default_is_empty() {
        let actor: Actor = Default::default();
        assert!(actor.ability::<Logging>().is_none());
    }

    // ── Interaction execution ──

    #[test]
    fn attempts_to_single_interaction() {
        let called = Cell::new(false);
        let actor = Actor::new();
        actor.attempts_to((FlagInteraction { flag: &called },));
        assert!(called.get());
    }

    #[test]
    fn attempts_to_executes_in_order() {
        let log = std::cell::RefCell::new(Vec::new());

        struct A<'a>(&'a std::cell::RefCell<Vec<i32>>, i32);
        impl Interaction for A<'_> {
            fn perform_as(&self, _actor: &Actor) {
                self.0.borrow_mut().push(self.1);
            }
        }

        let actor = Actor::new();
        actor.attempts_to((A(&log, 1), A(&log, 2), A(&log, 3)));

        assert_eq!(*log.borrow(), vec![1, 2, 3]);
    }

    #[test]
    fn attempts_to_10_interactions() {
        let count = Cell::new(0);

        struct Bump<'a>(&'a Cell<i32>);
        impl Interaction for Bump<'_> {
            fn perform_as(&self, _actor: &Actor) {
                self.0.set(self.0.get() + 1);
            }
        }

        let actor = Actor::new();
        actor.attempts_to((
            Bump(&count),
            Bump(&count),
            Bump(&count),
            Bump(&count),
            Bump(&count),
            Bump(&count),
            Bump(&count),
            Bump(&count),
            Bump(&count),
            Bump(&count),
        ));

        assert_eq!(count.get(), 10);
    }

    // ── Expectation tests ──

    #[test]
    fn is_true_passes() {
        let e = is_true();
        assert!(e.test(&true));
        assert!(!e.test(&false));
        assert!(e.message(&false).contains("false"));
    }

    #[test]
    fn equals_passes() {
        let e = equals(42);
        assert!(e.test(&42));
        assert!(!e.test(&99));
        assert!(e.message(&99).contains("99"));
        assert!(e.message(&99).contains("42"));
    }

    #[test]
    fn is_greater_than_passes() {
        let e = is_greater_than(10);
        assert!(e.test(&42));
        assert!(!e.test(&5));
        assert!(!e.test(&10)); // not strictly greater
        let msg = e.message(&5);
        assert!(msg.contains("5"));
        assert!(msg.contains("10"));
    }

    // ── Ensure integration ──

    #[test]
    fn ensure_passes_when_expectation_met() {
        let actor = Actor::new();
        // Should not panic
        actor.attempts_to((Ensure::that(ConstantQuestion { value: 42 }, equals(42)),));
    }

    #[test]
    #[should_panic(expected = "Expected 99 to equal 42")]
    fn ensure_panics_when_expectation_not_met() {
        let actor = Actor::new();
        actor.attempts_to((Ensure::that(ConstantQuestion { value: 99 }, equals(42)),));
    }

    #[test]
    #[should_panic(expected = "Expected true, but got false")]
    fn ensure_is_true_panics_with_message() {
        let actor = Actor::new();
        actor.attempts_to((Ensure::that(ConstantQuestion { value: false }, is_true()),));
    }

    #[test]
    #[should_panic(expected = "Expected 5 to be greater than 10")]
    fn ensure_is_greater_than_panics_with_message() {
        let actor = Actor::new();
        actor.attempts_to((Ensure::that(
            ConstantQuestion { value: 5 },
            is_greater_than(10),
        ),));
    }

    // ── Ability access for interactions ──

    #[test]
    fn interaction_can_access_actor_abilities() {
        struct Counter(i32);
        impl Ability for Counter {}

        use std::cell::RefCell;
        let log = RefCell::new(Vec::new());

        struct InspectAbility<'a>(&'a RefCell<Vec<i32>>);
        impl Interaction for InspectAbility<'_> {
            fn perform_as(&self, actor: &Actor) {
                let counter = actor.ability::<Counter>().unwrap();
                self.0.borrow_mut().push(counter.0);
            }
        }

        let actor = Actor::new().who_can(Counter(77));
        actor.attempts_to((InspectAbility(&log),));

        assert_eq!(*log.borrow(), vec![77]);
    }
}
