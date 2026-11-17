use tempo_types::Revision;

use crate::{
    attempt::{Attempt, AttemptStatus},
    study::StudyError,
};

/// Checks if the given attempt is open and matches the expected revision.
/// Returns the next revision if successful, or an appropriate `StudyError` otherwise.
/// 
/// # Errors
/// Returns `StudyError::RevisionConflict` if the attempt's revision does not match the expected revision.
/// Returns `StudyError::ClosedAttempt` if the attempt is not active.
/// Returns `StudyError::RevisionExhausted` if the next revision cannot be computed.
/// 
/// # Parameters
/// - `attempt`: The attempt to check.
/// - `expected`: The expected revision of the attempt.
/// 
/// # Returns
/// Returns the next revision if the attempt is open and matches the expected revision.
/// 
/// # Examples
/// ```rust
/// let attempt = Attempt { revision: Revision(1), status: AttemptStatus::Active, ... };
/// let next_revision = check_open(&attempt, Revision(1))?;
/// assert_eq!(next_revision, Revision(2));
/// ```
pub fn check_open<S, P>(attempt: &Attempt<S, P>, expected: Revision) -> Result<Revision, StudyError> {
    if attempt.revision != expected {
        return Err(StudyError::RevisionConflict {
            expected,
            actual: attempt.revision,
        });
    }
    if attempt.status != AttemptStatus::Active {
        return Err(StudyError::ClosedAttempt);
    }
    attempt
        .revision
        .0
        .checked_add(1)
        .map(Revision)
        .ok_or(StudyError::RevisionExhausted)
}
