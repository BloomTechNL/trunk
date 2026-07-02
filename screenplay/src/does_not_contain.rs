//! Negative substring expectation — check that a `String` does *not* contain a
//! given substring.

use crate::expectation::Expectation;

/// Expect a `String` **not** to contain the given substring.
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
/// actor.attempts_to((Ensure::that(Greeting, does_not_contain("goodbye")),));
/// ```
pub fn does_not_contain(expected: impl Into<String>) -> impl Expectation<String> {
    DoesNotContain {
        expected: expected.into(),
    }
}

struct DoesNotContain {
    expected: String,
}

impl Expectation<String> for DoesNotContain {
    fn test(&self, value: &String) -> bool {
        !value.contains(&self.expected)
    }

    fn message(&self, value: &String) -> String {
        format!(
            "Expected output NOT to contain {:?}, but got:\n{}",
            self.expected, value
        )
    }
}
