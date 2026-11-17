use std::{error::Error, fmt};

use crate::{attempt::Attempt, exercise::Exercise, grader::GradingError};

/// Represents a started exercise along with its associated attempt.
/// Contains the exercise and the attempt that has been started for it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Started<D, S, P> {
    pub exercise: Exercise<D>,
    pub attempt: Attempt<S, P>,
}

#[derive(Debug)]
pub enum StartError<E> {
    Source(E),
    Grading(GradingError),
}

impl<E: fmt::Display> fmt::Display for StartError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source(e) => write!(f, "question source: {e}"),
            Self::Grading(e) => write!(f, "grader: {e}"),
        }
    }
}
impl<E: Error + 'static> Error for StartError<E> {}
