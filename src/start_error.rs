//! Distinguish a failure to obtain content from a failure to initialize it.
//!
//! Handle [`StartError`] around [`crate::next`]. A source error can mean a storage
//! or selection failure; a grading error means supplied content cannot initialize
//! the grader. Neither is ordinary emptiness, and neither creates an attempt.

use crate::GradingError;
use std::{error::Error, fmt};

/// A typed failure at one of the two start boundaries.
#[derive(Debug)]
pub enum StartError<E> {
    /// The question-source adapter could not supply a result.
    Source(E),
    /// The grader rejected the selected definition.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Availability, Exercise, QuestionSource, next,
        test_support::{ChoiceGrader, OneExercise, choice_exercise},
    };

    struct UnavailableSource;
    impl QuestionSource for UnavailableSource {
        type Definition = char;
        type Error = &'static str;
        fn next(&mut self) -> Result<Availability<Exercise<char>>, Self::Error> {
            Err("cannot read selection")
        }
    }

    #[test]
    fn a_caller_can_distinguish_source_failure_from_bad_content() {
        let source_failure = next(
            &mut UnavailableSource,
            &ChoiceGrader,
            "d".into(),
            "a".into(),
        );
        assert!(matches!(
            source_failure,
            Err(StartError::Source("cannot read selection"))
        ));

        let mut malformed = choice_exercise();
        malformed.definition = 'Z';
        let invalid_content = next(
            &mut OneExercise(Some(malformed)),
            &ChoiceGrader,
            "d".into(),
            "a".into(),
        );
        assert!(matches!(
            invalid_content,
            Err(StartError::Grading(GradingError::InvalidDefinition(_)))
        ));
    }
}
