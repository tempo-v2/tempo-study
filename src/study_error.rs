//! Failures when answering or abandoning an existing attempt.
//!
//! Handle [`StudyError`] at the application boundary. A stale revision asks the
//! caller to reload current state; a closed attempt cannot accept more answers;
//! a grading error means input/state was not evaluable. A wrong, valid answer
//! returns progress with feedback and an optional newly emitted assessment. It is
//! not an operational error; an earlier terminal grade prevents another emission.

use crate::GradingError;
use std::{error::Error, fmt};
use tempo_types::Revision;

/// Why Study could not propose the requested transition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StudyError {
    /// The command was made against a different attempt revision.
    RevisionConflict {
        expected: Revision,
        actual: Revision,
    },
    /// Presentation is complete or abandoned.
    ClosedAttempt,
    /// Incrementing the response revision would overflow.
    RevisionExhausted,
    /// The grader's prompt, completion, and assessment contradict the contract.
    InvalidGraderTransition,
    /// The grader could not process the supplied content, state, or answer.
    Grading(GradingError),
}

impl fmt::Display for StudyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl Error for StudyError {}
impl From<GradingError> for StudyError {
    fn from(value: GradingError) -> Self {
        Self::Grading(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        submit,
        test_support::{ChoiceGrader, issued_choice},
    };

    #[test]
    fn a_stale_view_reports_both_revisions_to_the_caller() {
        let mut current = issued_choice();
        current.revision = Revision(3);
        let error = submit(&ChoiceGrader, &'B', &current, Revision(2), &'B').unwrap_err();
        match error {
            StudyError::RevisionConflict { expected, actual } => {
                assert_eq!(expected, Revision(2));
                assert_eq!(actual, Revision(3));
            }
            other => panic!("expected stale-view error, got {other}"),
        }
    }
}
