use crate::{Expectation, Outcome};

/// Expect an [`Outcome::Failure`], optionally with a specific error substring.
///
/// `fails()` — matches any failure.
/// `fails().with_error("...")` — matches only failures whose message contains the
/// given substring.
pub fn fails() -> Fails {
    Fails { expected: None }
}

pub struct Fails {
    expected: Option<String>,
}

impl Fails {
    pub fn with_error(mut self, msg: impl Into<String>) -> Self {
        self.expected = Some(msg.into());
        self
    }
}

impl Expectation<Outcome> for Fails {
    fn test(&self, value: &Outcome) -> bool {
        match (value, &self.expected) {
            (Outcome::Failure(_), None) => true,
            (Outcome::Failure(actual), Some(expected)) => actual.contains(expected),
            _ => false,
        }
    }

    fn message(&self, value: &Outcome) -> String {
        match value {
            Outcome::Success => "expected a failure but got success".to_string(),
            Outcome::Failure(actual) => match &self.expected {
                None => format!("expected failure, got: {}", actual),
                Some(expected) => format!(
                    "expected failure containing {:?}, got: {}",
                    expected, actual
                ),
            },
        }
    }
}
