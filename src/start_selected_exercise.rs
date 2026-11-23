//! Initialize the attempt for one selected exercise.
use crate::{Attempt, AttemptStatus, Exercise, Grader, GradingError};
use tempo_types::{AttemptId, DatasetId, Revision};

/// Initialize selected content and allocate the attempt ID after the grader starts.
pub fn start_selected_exercise<G: Grader + ?Sized>(
    exercise: &Exercise<G::Definition>,
    grader: &G,
    dataset: DatasetId,
    id: impl FnOnce() -> AttemptId,
) -> Result<Attempt<G::State, G::Prompt>, GradingError> {
    let start = grader.start(&exercise.definition)?;
    Ok(Attempt {
        id: id(),
        dataset,
        target: exercise.target.clone(),
        content_version: exercise.version,
        revision: Revision(0),
        state: start.state,
        prompt: Some(start.prompt),
        terminal: None,
        status: AttemptStatus::Active,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{ChoiceGrader, choice_exercise};

    #[test]
    fn initialization_sets_the_active_identity_and_safe_prompt() {
        let exercise = choice_exercise();
        let attempt = start_selected_exercise(&exercise, &ChoiceGrader, "dataset".into(), || {
            "run-1".into()
        })
        .unwrap();
        assert_eq!(attempt.id, AttemptId::from("run-1"));
        assert_eq!(attempt.dataset, DatasetId::from("dataset"));
        assert_eq!(attempt.target, exercise.target);
        assert_eq!(attempt.content_version, exercise.version);
        assert_eq!(attempt.revision, Revision(0));
        assert_eq!(attempt.status, AttemptStatus::Active);
        assert_eq!(attempt.terminal, None);
        assert_eq!(attempt.prompt.as_deref(), Some("Choose A or B"));
    }

    #[test]
    fn invalid_definition_returns_the_grading_error_before_allocating_an_id() {
        let mut exercise = choice_exercise();
        exercise.definition = 'C';
        let result = start_selected_exercise(&exercise, &ChoiceGrader, "dataset".into(), || {
            panic!("invalid content must not allocate an attempt ID")
        });
        assert!(matches!(result, Err(GradingError::InvalidDefinition(_))));
    }
}
