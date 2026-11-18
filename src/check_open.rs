//! The shared revision/lifecycle guard used by answer and abandon operations.
//!
//! Reach for [`crate::submit()`] or [`crate::abandon()`] for normal application work.
//! This helper exists so both operations check the same preconditions before
//! grading or changing state. It computes the next revision without mutation.

use crate::{Attempt, AttemptStatus, StudyError};
use tempo_types::Revision;

/// Return the next revision only when the expected revision matches an active run.
///
/// # Errors
/// Returns a revision conflict before checking lifecycle, then a closed-attempt
/// error for a finished/abandoned run, or exhaustion if increment would overflow.
pub fn check_open<S, P>(
    attempt: &Attempt<S, P>,
    expected: Revision,
) -> Result<Revision, StudyError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::issued_choice;

    #[test]
    fn a_matching_active_view_can_advance_without_mutating_the_original() {
        let attempt = issued_choice();
        assert_eq!(check_open(&attempt, Revision(0)), Ok(Revision(1)));
        assert_eq!(attempt.revision, Revision(0));
    }

    #[test]
    fn an_old_view_is_rejected_before_a_closed_lifecycle_is_considered() {
        let mut attempt = issued_choice();
        attempt.revision = Revision(1);
        attempt.status = AttemptStatus::Complete;
        assert_eq!(
            check_open(&attempt, Revision(0)),
            Err(StudyError::RevisionConflict {
                expected: Revision(0),
                actual: Revision(1),
            })
        );
        assert_eq!(
            check_open(&attempt, Revision(1)),
            Err(StudyError::ClosedAttempt)
        );
    }

    #[test]
    fn revision_exhaustion_is_an_error_instead_of_wrapping_to_an_old_value() {
        let mut attempt = issued_choice();
        attempt.revision = Revision(u64::MAX);
        assert_eq!(
            check_open(&attempt, Revision(u64::MAX)),
            Err(StudyError::RevisionExhausted)
        );
        assert_eq!(attempt.revision, Revision(u64::MAX));
    }
}
