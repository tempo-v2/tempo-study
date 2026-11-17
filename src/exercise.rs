//! A selected learnable target plus the exact definition its grader will use.
//!
//! Use [`Exercise`] at the question-source → Study boundary. The source adapter
//! selects content; Study passes its definition to the grader. A definition may
//! include expected answers and must not be sent directly to the UI. The grader
//! creates a separate safe prompt for presentation.

use serde::{Deserialize, Serialize};
use tempo_types::{ContentVersion, TargetId};

/// Selected content, before an attempt has been issued.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise<D> {
    /// Stable identity across repeats and compatible metadata changes.
    pub target: TargetId,
    /// The semantic revision of this definition.
    pub version: ContentVersion,
    /// Grader-specific content, potentially including expected answers.
    pub definition: D,
}

#[cfg(test)]
mod tests {
    use crate::{
        Availability, next,
        test_support::{ChoiceGrader, OneExercise, choice_exercise},
    };

    #[test]
    fn the_definition_goes_to_the_grader_while_the_prompt_goes_to_presentation() {
        let exercise = choice_exercise();
        let mut source = OneExercise(Some(exercise.clone()));
        let Availability::Ready(started) =
            next(&mut source, &ChoiceGrader, "dataset".into(), "run-1".into()).unwrap()
        else {
            panic!("exercise is available")
        };

        assert_eq!(started.exercise.definition, 'B');
        assert_eq!(started.attempt.prompt.as_deref(), Some("Choose A or B"));
        assert_eq!(started.attempt.target, exercise.target);
        assert_eq!(started.attempt.content_version, exercise.version);
    }
}
