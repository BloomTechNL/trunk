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

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

// ---------------------------------------------------------------------------
// Ability — marker trait for capabilities the Actor can use
// ---------------------------------------------------------------------------

/// Marker trait for abilities that can be given to an [`Actor`].
///
/// Abilities must be `'static` so they can be stored in a type-erased map
/// and retrieved later with [`Actor::ability`].
///
/// # Example
///
/// ```rust
/// use screenplay::Ability;
///
/// struct WebBrowser { headless: bool }
/// impl Ability for WebBrowser {}
/// ```
pub trait Ability: 'static {}

// ---------------------------------------------------------------------------
// Actor — the central abstraction that performs interactions
// ---------------------------------------------------------------------------

/// The **Actor** is the heart of the Screenplay Pattern.
///
/// An actor holds a set of type-erased [`Ability`] values and can
/// [`attempts_to`](Actor::attempts_to) perform one or more
/// [`Interaction`]s. Use [`who_can`](Actor::who_can) in a builder style
/// to equip the actor with abilities before the test scenario begins.
///
/// # Example
///
/// ```rust
/// use screenplay::*;
///
/// struct Browsing;
/// impl Ability for Browsing {}
///
/// let user = Actor::new().who_can(Browsing);
/// assert!(user.ability::<Browsing>().is_some());
/// ```
pub struct Actor {
    abilities: HashMap<TypeId, Box<dyn Any>>,
}

impl Actor {
    /// Create a new actor with no abilities.
    ///
    /// Use [`who_can`](Actor::who_can) to add abilities in builder style.
    pub fn new() -> Self {
        Actor {
            abilities: HashMap::new(),
        }
    }

    /// Add an ability and return `Self` for chaining.
    ///
    /// If an ability of the same concrete type already exists it is silently
    /// replaced.
    ///
    /// ```rust
    /// use screenplay::*;
    ///
    /// struct Browsing;
    /// impl Ability for Browsing {}
    ///
    /// let actor = Actor::new()
    ///     .who_can(Browsing);
    /// ```
    pub fn who_can<A: Ability + 'static>(mut self, ability: A) -> Self {
        self.abilities.insert(TypeId::of::<A>(), Box::new(ability));
        self
    }

    /// Borrow an ability of type `A`, or `None` if the actor doesn't have it.
    ///
    /// ```rust
    /// use screenplay::*;
    ///
    /// struct Browsing;
    /// impl Ability for Browsing {}
    ///
    /// struct Logging;
    /// impl Ability for Logging {}
    ///
    /// let actor = Actor::new().who_can(Browsing);
    /// assert!(actor.ability::<Browsing>().is_some());
    /// assert!(actor.ability::<Logging>().is_none());
    /// ```
    pub fn ability<A: Ability + 'static>(&self) -> Option<&A> {
        self.abilities
            .get(&TypeId::of::<A>())
            .and_then(|boxed| boxed.downcast_ref::<A>())
    }

    /// Execute one or more [`Interaction`]s sequentially.
    ///
    /// Pass a **tuple** of interactions: `(a,)`, `(a, b)`, `(a, b, c)`, …
    /// (up to 10 elements). Each interaction's
    /// [`perform_as`](Interaction::perform_as) is called in order.
    ///
    /// ```rust
    /// use screenplay::*;
    ///
    /// struct SayHi;
    /// impl Interaction for SayHi {
    ///     fn perform_as(&self, _actor: &Actor) { /* … */ }
    /// }
    ///
    /// let actor = Actor::new();
    /// actor.attempts_to((SayHi,));
    /// actor.attempts_to((SayHi, SayHi));
    /// ```
    pub fn attempts_to<T: Interactions>(&self, interactions: T) {
        interactions.perform_all(self);
    }
}

impl Default for Actor {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Interaction — something the actor can *do*
// ---------------------------------------------------------------------------

/// An **Interaction** is an action that an [`Actor`] can perform.
///
/// Implement this trait for each user-level action (e.g. `NavigateTo`,
/// `FillIn`, `Click`). The actor's abilities are accessible through the
/// `actor` parameter.
///
/// ```rust
/// use screenplay::*;
///
/// struct Increment;
/// impl Interaction for Increment {
///     fn perform_as(&self, _actor: &Actor) {
///         // actor.ability::<Counter>().unwrap().inc();
///     }
/// }
/// ```
pub trait Interaction {
    /// Execute this interaction with the given actor's abilities.
    fn perform_as(&self, actor: &Actor);
}

// ---------------------------------------------------------------------------
// Interactions — dispatch trait for tuples of interactions
// ---------------------------------------------------------------------------

/// Trait implemented for tuples of [`Interaction`]s (arity 1 through 10).
///
/// This is the internal dispatch mechanism behind
/// [`Actor::attempts_to`]. Library users do not need to implement
/// this trait directly.
pub trait Interactions {
    /// Execute every interaction in the tuple, in order.
    fn perform_all(&self, actor: &Actor);
}

/// Macro that generates an [`Interactions`] impl for a tuple of the given
/// arity. Each element is constrained to [`Interaction`] and its
/// [`perform_as`](Interaction::perform_as) is called in sequence.
macro_rules! impl_interactions {
    ($($idx:tt => $gen:ident),+) => {
        impl<$($gen: Interaction),+> Interactions for ($($gen,)+) {
            fn perform_all(&self, actor: &Actor) {
                $(self.$idx.perform_as(actor);)+
            }
        }
    };
}

// Generate impls for tuples of size 1 through 10.
impl_interactions!(0 => A0);
impl_interactions!(0 => A0, 1 => A1);
impl_interactions!(0 => A0, 1 => A1, 2 => A2);
impl_interactions!(0 => A0, 1 => A1, 2 => A2, 3 => A3);
impl_interactions!(0 => A0, 1 => A1, 2 => A2, 3 => A3, 4 => A4);
impl_interactions!(0 => A0, 1 => A1, 2 => A2, 3 => A3, 4 => A4, 5 => A5);
impl_interactions!(0 => A0, 1 => A1, 2 => A2, 3 => A3, 4 => A4, 5 => A5, 6 => A6);
impl_interactions!(0 => A0, 1 => A1, 2 => A2, 3 => A3, 4 => A4, 5 => A5, 6 => A6, 7 => A7);
impl_interactions!(0 => A0, 1 => A1, 2 => A2, 3 => A3, 4 => A4, 5 => A5, 6 => A6, 7 => A7, 8 => A8);
impl_interactions!(0 => A0, 1 => A1, 2 => A2, 3 => A3, 4 => A4, 5 => A5, 6 => A6, 7 => A7, 8 => A8, 9 => A9);

// ---------------------------------------------------------------------------
// Question — something the actor asks about system state
// ---------------------------------------------------------------------------

/// A **Question** retrieves information from the system through the actor's
/// abilities.
///
/// The type parameter `T` is the answer type — it can be any type.
///
/// ```rust
/// use screenplay::*;
///
/// struct PageTitle;
/// impl Question<String> for PageTitle {
///     fn answered_by(&self, actor: &Actor) -> String {
///         // actor.ability::<Browsing>().unwrap().title()
///         "Home".to_string()
///     }
/// }
/// ```
pub trait Question<T> {
    /// Answer the question using the actor's abilities.
    fn answered_by(&self, actor: &Actor) -> T;
}

// ---------------------------------------------------------------------------
// Expectation — a predicate over a value
// ---------------------------------------------------------------------------

/// An **Expectation** is a boolean predicate with a human-readable message.
///
/// Use the built-in constructors ([`is_true`], [`equals`], [`is_greater_than`])
/// or implement your own.
///
/// ```rust
/// use screenplay::*;
/// use std::fmt::Debug;
///
/// struct Contains<T: PartialEq + Debug + 'static> { substring: T }
///
/// impl<T: PartialEq + Debug + 'static> Expectation<T> for Contains<T> {
///     fn test(&self, value: &T) -> bool {
///         // simplified — real impl would iterate
///         value == &self.substring
///     }
///     fn message(&self, value: &T) -> String {
///         format!("Expected {:?} to contain {:?}", value, self.substring)
///     }
/// }
/// ```
pub trait Expectation<T> {
    /// Return `true` when `value` meets the expectation.
    fn test(&self, value: &T) -> bool;
    /// Produce a human-readable failure message for the given `value`.
    fn message(&self, value: &T) -> String;
}

// ---------------------------------------------------------------------------
// Built-in expectations
// ---------------------------------------------------------------------------

/// Expect a `bool` value to be `true`.
///
/// ```rust
/// use screenplay::*;
///
/// struct IsLoggedIn;
/// impl Question<bool> for IsLoggedIn {
///     fn answered_by(&self, _actor: &Actor) -> bool { true }
/// }
///
/// let actor = Actor::new();
/// actor.attempts_to((Ensure::that(IsLoggedIn, is_true()),));
/// ```
pub fn is_true() -> impl Expectation<bool> {
    IsTrue
}

struct IsTrue;

impl Expectation<bool> for IsTrue {
    fn test(&self, value: &bool) -> bool {
        *value
    }
    fn message(&self, _value: &bool) -> String {
        "Expected true, but got false".to_string()
    }
}

/// Expect a value to equal `expected` (uses [`PartialEq`]).
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
/// actor.attempts_to((Ensure::that(TheAnswer, equals(42)),));
/// ```
pub fn equals<T: PartialEq + Debug + 'static>(expected: T) -> impl Expectation<T> {
    Equals { expected }
}

struct Equals<T> {
    expected: T,
}

impl<T: PartialEq + Debug + 'static> Expectation<T> for Equals<T> {
    fn test(&self, value: &T) -> bool {
        *value == self.expected
    }
    fn message(&self, value: &T) -> String {
        format!("Expected {:?} to equal {:?}", value, self.expected)
    }
}

/// Expect a value to be strictly greater than `expected` (uses [`PartialOrd`]).
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
/// actor.attempts_to((Ensure::that(TheAnswer, is_greater_than(0)),));
/// ```
pub fn is_greater_than<T: PartialOrd + Debug + 'static>(expected: T) -> impl Expectation<T> {
    IsGreaterThan { expected }
}

struct IsGreaterThan<T> {
    expected: T,
}

impl<T: PartialOrd + Debug + 'static> Expectation<T> for IsGreaterThan<T> {
    fn test(&self, value: &T) -> bool {
        *value > self.expected
    }
    fn message(&self, value: &T) -> String {
        format!(
            "Expected {:?} to be greater than {:?}",
            value, self.expected
        )
    }
}

// ---------------------------------------------------------------------------
// Ensure — combine a Question with an Expectation into an Interaction
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Dummy ability for tests
    struct Logging;
    impl Ability for Logging {}

    // Dummy interaction that records it was called
    use std::cell::Cell;

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
