use crate::{
    attempt::{Attempt, AttemptStatus},
    start::Started,
};
use tempo_types::prelude::*;

use crate::{
    Availability, grader::Grader, next_result::NextResult, question_source::QuestionSource,
    start::StartError,
};

/// Returns the next available exercise from the question source, or indicates if there are no more exercises.
pub fn next_available_exercise_if_any<S, G>(
    source: &mut S,
    grader: &G,
    dataset: DatasetId,
    id: AttemptId,
) -> NextResult<G::Definition, G::State, G::Prompt, S::Error>
where
    S: QuestionSource<Definition = G::Definition>,
    G: Grader + ?Sized,
{
    let exercise = match source.next().map_err(StartError::Source)? {
        Availability::Empty => return Ok(Availability::Empty),
        Availability::Ready(exercise) => exercise,
    };
    let start = grader
        .start(&exercise.definition)
        .map_err(StartError::Grading)?;
    let attempt = Attempt {
        id,
        dataset,
        target: exercise.target.clone(),
        content_version: exercise.version,
        revision: Revision(0),
        state: start.state,
        prompt: Some(start.prompt),
        terminal: None,
        status: AttemptStatus::Active,
    };
    Ok(Availability::Ready(Started { exercise, attempt }))
}
