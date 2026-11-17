//! A grader cannot process malformed content, state, or an answer representation.
//!
//! Return [`GradingError`] for an input the grading protocol cannot evaluate.
//! A well-formed but wrong answer is a successful `GradeStep` with feedback and
//! possibly `Again`, not an error. Study wraps these errors without scheduling or
//! persisting a learning outcome.

use std::{error::Error, fmt};

/// Why the grading operation could not produce a valid step.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GradingError {
    /// The exercise definition cannot be used by this grader.
    InvalidDefinition(String),
    /// Stored/resumed state cannot be used with the definition.
    InvalidState(String),
    /// The answer representation cannot be evaluated by this grader.
    InvalidAnswer(String),
}

impl fmt::Display for GradingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl Error for GradingError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Grader, test_support::ChoiceGrader};
    use tempo_types::Grade;

    #[test]
    fn an_unrecognized_answer_is_an_error_but_a_valid_wrong_choice_is_a_grade() {
        assert!(matches!(
            ChoiceGrader.answer(&'B', &(), &'Z'),
            Err(GradingError::InvalidAnswer(_))
        ));
        let wrong = ChoiceGrader.answer(&'B', &(), &'A').unwrap();
        assert_eq!(wrong.assessment, Some(Grade::Again));
    }
}
