//! Close an active attempt without inventing a learning outcome.
//!
//! Use [`abandon`] when the user leaves a run. It returns a replacement attempt
//! with no prompt and an advanced response revision. An existing terminal grade
//! remains; a previously ungraded attempt stays ungraded. The app commits it.

use crate::{Attempt, AttemptStatus, StudyError, check_open::check_open};
use tempo_types::Revision;

/// Propose abandonment after checking the caller's expected revision.
pub fn abandon<S: Clone, P: Clone>(
    attempt: &Attempt<S, P>,
    expected_revision: Revision,
) -> Result<Attempt<S, P>, StudyError> {
    let revision = check_open(attempt, expected_revision)?;
    let mut updated = attempt.clone();
    updated.revision = revision;
    updated.status = AttemptStatus::Abandoned;
    updated.prompt = None;
    Ok(updated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        submit,
        test_support::{ChoiceGrader, issued_choice},
    };
    use tempo_types::Grade;

    #[test]
    fn leaving_before_answering_closes_presentation_without_recording_failure() {
        let original = issued_choice();
        let abandoned = abandon(&original, Revision(0)).unwrap();
        assert_eq!(abandoned.status, AttemptStatus::Abandoned);
        assert_eq!(abandoned.revision, Revision(1));
        assert_eq!(abandoned.prompt, None);
        assert_eq!(abandoned.terminal, None);
        assert_eq!(original.status, AttemptStatus::Active);
        assert_eq!(
            submit(&ChoiceGrader, &'B', &abandoned, Revision(1), &'B'),
            Err(StudyError::ClosedAttempt)
        );
    }

    #[test]
    fn leaving_after_a_miss_preserves_the_grade_already_recorded() {
        let miss = submit(&ChoiceGrader, &'B', &issued_choice(), Revision(0), &'A').unwrap();
        let abandoned = abandon(&miss.attempt, Revision(1)).unwrap();
        assert_eq!(abandoned.terminal, Some(Grade::Again));
        assert_eq!(abandoned.status, AttemptStatus::Abandoned);
    }
}
