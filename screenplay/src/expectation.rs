//! Expectations — boolean predicates with human-readable messages.

use std::fmt::Debug;

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

/// Expect a `bool` value to be `false`.
pub fn is_false() -> impl Expectation<bool> {
    IsFalse
}

struct IsFalse;

impl Expectation<bool> for IsFalse {
    fn test(&self, value: &bool) -> bool {
        !*value
    }
    fn message(&self, _value: &bool) -> String {
        "Expected false, but got true".to_string()
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

