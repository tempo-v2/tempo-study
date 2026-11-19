//! Apply a single sticky Again when assistance reveals the prescribed answer.
use crate::{Attempt, AttemptStatus, Progress, StudyError, check_open::check_open};
use tempo_types::{Grade, Revision};

pub fn give_up<S: Clone, P: Clone, F>(
    attempt: &Attempt<S, P>,
    expected_revision: Revision,
    feedback: F,
) -> Result<Progress<S, P, F>, StudyError> {
    let revision = check_open(attempt, expected_revision)?;
    let assessment = attempt.terminal.is_none().then_some(Grade::Again);
    let mut updated = attempt.clone();
    updated.revision = revision;
    updated.terminal = attempt.terminal.or(assessment);
    Ok(Progress {
        attempt: updated,
        feedback,
        assessment,
    })
}

/// End a blind recall for assistance. The first miss is sticky, even if the
/// learner had already missed before choosing to leave. No correction was played.
pub fn concede<S: Clone, P: Clone, F>(
    attempt: &Attempt<S, P>,
    expected_revision: Revision,
    feedback: F,
) -> Result<Progress<S, P, F>, StudyError> {
    let mut result = give_up(attempt, expected_revision, feedback)?;
    result.attempt.status = AttemptStatus::Abandoned;
    result.attempt.prompt = None;
    Ok(result)
}
