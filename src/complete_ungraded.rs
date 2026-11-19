//! Complete previously authorized guided work without producing a grade.
use crate::{Attempt, AttemptStatus, StudyError, check_open::check_open};
use tempo_types::Revision;

/// Complete active, ungraded work with the subject owner's validated next state.
pub fn complete_ungraded<S: Clone, P: Clone>(
    attempt: &Attempt<S, P>,
    expected_revision: Revision,
    next_state: S,
) -> Result<Attempt<S, P>, StudyError> {
    let revision = check_open(attempt, expected_revision)?;
    if attempt.terminal.is_some() {
        return Err(StudyError::ClosedAttempt);
    }
    Ok(Attempt {
        revision,
        state: next_state,
        prompt: None,
        status: AttemptStatus::Complete,
        ..attempt.clone()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        start_selected_exercise,
        test_support::{ChoiceGrader, choice_exercise},
    };

    #[test]
    fn a_selected_exercise_can_complete_without_replacing_its_identity_or_recording_a_grade() {
        let exercise = choice_exercise();
        let started =
            start_selected_exercise(&exercise, &ChoiceGrader, "d".into(), || "a".into()).unwrap();
        let result = complete_ungraded(&started, Revision(0), started.state).unwrap();
        assert_eq!(result.dataset, started.dataset);
        assert_eq!(result.id, started.id);
        assert_eq!(result.target, started.target);
        assert_eq!(result.content_version, started.content_version);
        assert_eq!(result.revision, Revision(1));
        assert_eq!(result.status, AttemptStatus::Complete);
        assert_eq!(result.terminal, None);
        assert_eq!(result.prompt, None);
        assert_eq!(
            complete_ungraded(&result, Revision(1), started.state),
            Err(StudyError::ClosedAttempt)
        );
    }
}
