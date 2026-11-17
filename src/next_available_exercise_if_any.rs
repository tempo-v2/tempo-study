//! Ask the source for one exercise, then let its grader initialize an attempt.
//!
//! Use this when the app needs another question. Queue selection is hidden behind
//! [`QuestionSource`]; correctness is hidden behind [`Grader`]. The app supplies
//! dataset/attempt IDs and later commits the returned proposal. `next` is the
//! shorter public alias for this same function.

use crate::{
    Attempt, AttemptStatus, Availability, Grader, NextResult, QuestionSource, StartError, Started,
};
use tempo_types::{AttemptId, DatasetId, Revision};

/// Start currently available content without persisting anything.
///
/// An empty source is a normal result. A source failure is returned unchanged
/// inside `StartError::Source`; invalid content becomes `StartError::Grading`.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{ChoiceGrader, OneExercise, choice_exercise};

    #[test]
    fn one_selected_exercise_becomes_an_active_attempt_at_revision_zero() {
        let mut source = OneExercise(Some(choice_exercise()));
        let Availability::Ready(started) = next_available_exercise_if_any(
            &mut source,
            &ChoiceGrader,
            "dataset".into(),
            "run-1".into(),
        )
        .unwrap() else {
            panic!("exercise is ready")
        };

        assert_eq!(started.attempt.revision, Revision(0));
        assert_eq!(started.attempt.status, AttemptStatus::Active);
        assert_eq!(started.attempt.terminal, None);
        assert_eq!(started.attempt.prompt.as_deref(), Some("Choose A or B"));
        assert_eq!(started.exercise.definition, 'B');
    }

    #[test]
    fn empty_is_normal_and_the_caller_can_ask_again_after_content_changes() {
        let mut source = OneExercise(None);
        assert!(matches!(
            next_available_exercise_if_any(
                &mut source,
                &ChoiceGrader,
                "dataset".into(),
                "unused-id".into(),
            )
            .unwrap(),
            Availability::Empty
        ));

        source.0 = Some(choice_exercise());
        assert!(matches!(
            next_available_exercise_if_any(
                &mut source,
                &ChoiceGrader,
                "dataset".into(),
                "new-run".into(),
            )
            .unwrap(),
            Availability::Ready(_)
        ));
    }
}
