use crate::attempt::{Attempt, AttemptStatus};
use crate::check_open::check_open;
use crate::study::StudyError;
use tempo_types::Revision;

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
