use tempo_types::Revision;

use crate::{
    attempt::{Attempt, AttemptStatus},
    check_open::check_open,
    grader::{Grade, Grader},
    progress::Progress,
    study::StudyError,
    submit_result::SubmitResult,
};

/// Submits an answer for the given attempt to the grader and returns
/// the resulting progress or an error.
pub fn submit<G: Grader + ?Sized>(
    grader: &G,
    definition: &G::Definition,
    attempt: &Attempt<G::State, G::Prompt>,
    expected_revision: Revision,
    answer: &G::Answer,
) -> SubmitResult<G::State, G::Prompt, G::Feedback> {
    let revision = check_open(attempt, expected_revision)?;
    let step = grader.answer(definition, &attempt.state, answer)?;
    // Completed presentation has no next question. Success cannot precede completion.
    if step.complete == step.prompt.is_some()
        || (step.assessment == Some(Grade::Good) && !step.complete)
        || (step.complete && step.assessment.is_none() && attempt.terminal.is_none())
    {
        return Err(StudyError::InvalidGraderTransition);
    }
    let assessment = if attempt.terminal.is_none() {
        step.assessment
    } else {
        None
    };
    let mut updated = attempt.clone();
    updated.revision = revision;
    updated.state = step.state;
    updated.prompt = step.prompt;
    updated.terminal = attempt.terminal.or(assessment);
    updated.status = if step.complete {
        AttemptStatus::Complete
    } else {
        AttemptStatus::Active
    };
    Ok(Progress {
        attempt: updated,
        feedback: step.feedback,
        assessment,
    })
}
