/// The result of attempting an interaction via [`doing`].
pub enum Outcome {
    Success,
    Failure(String),
}
