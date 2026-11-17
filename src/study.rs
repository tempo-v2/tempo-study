use std::error::Error;
use std::fmt;

use tempo_types::Revision;

use crate::grader::GradingError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StudyError {
    RevisionConflict {
        expected: Revision,
        actual: Revision,
    },
    ClosedAttempt,
    RevisionExhausted,
    InvalidGraderTransition,
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
