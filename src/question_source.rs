use crate::{Availability, exercise::Exercise};

/// A source of exercises that can be presented to the user.
/// Implemented by the application adapter; Study never imports Queue or Store.
/// The `next` method returns the next available exercise or indicates if there are no more exercises.
/// It returns a `Result` that wraps an `Availability` enum, which can be either `Ready` with an exercise or `Empty` if no exercises are available.
pub trait QuestionSource {
    /// The type of the exercise definition.
    type Definition;
    /// The type of error that can occur when fetching the next exercise.
    type Error;
    /// Returns the next available exercise or indicates if there are no more exercises.
    fn next(&mut self) -> Result<Availability<Exercise<Self::Definition>>, Self::Error>;
}
