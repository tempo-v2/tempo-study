//! The selected content and newly initialized attempt returned together.
//!
//! Use [`Started`] in the app after [`crate::next`] succeeds. Retain the exact
//! exercise definition for grading/history and commit the attempt with whatever
//! issue-time metadata the app needs. Only project its prompt to the UI. No
//! persistence or learning transition has happened merely by receiving this value.

use crate::{Attempt, Exercise};

/// A proposed new attempt paired with the content that initialized it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Started<D, S, P> {
    /// The selected definition, which may contain hidden answers.
    pub exercise: Exercise<D>,
    /// The new attempt at response revision zero.
    pub attempt: Attempt<S, P>,
}

#[cfg(test)]
mod tests {
    use crate::{
        Availability, next,
        test_support::{ChoiceGrader, OneExercise, choice_exercise},
    };
    use tempo_types::{AttemptId, Revision};

    #[test]
    fn the_app_receives_its_new_attempt_id_and_the_exact_selected_content() {
        let mut source = OneExercise(Some(choice_exercise()));
        let id = AttemptId::from("fresh-run");
        let Availability::Ready(started) =
            next(&mut source, &ChoiceGrader, "dataset".into(), id.clone()).unwrap()
        else {
            panic!("exercise is available")
        };

        assert_eq!(started.attempt.id, id);
        assert_eq!(started.attempt.revision, Revision(0));
        assert_eq!(started.attempt.content_version, started.exercise.version);
        assert_eq!(started.attempt.target, started.exercise.target);
        assert_eq!(started.exercise.definition, 'B');
    }
}
