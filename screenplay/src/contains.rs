//! Substring expectation — check that a `String` contains a given substring.

use crate::expectation::Expectation;

/// Expect a `String` to contain the given substring.
///
/// ```rust
/// use screenplay::*;
///
/// struct Greeting;
/// impl Question<String> for Greeting {
///     fn answered_by(&self, _actor: &Actor) -> String {
///         "hello world".to_string()
///     }
/// }
///
/// let actor = Actor::new();
/// actor.attempts_to((Ensure::that(Greeting, contains("hello")),));
/// ```
pub fn contains(expected: impl Into<String>) -> impl Expectation<String> {
    Contains {
        expected: expected.into(),
    }
}

struct Contains {
    expected: String,
}

impl Expectation<String> for Contains {
    fn test(&self, value: &String) -> bool {
        value.contains(&self.expected)
    }

    fn message(&self, value: &String) -> String {
        format!(
            "Expected output to contain {:?}, but got:\n{}",
            self.expected, value
        )
    }
}
