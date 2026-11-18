//! Apply a single sticky Again when assistance reveals the prescribed answer.
use crate::{Attempt, Progress, StudyError, check_open::check_open};
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
