pub mod abandon;
pub mod attempt;
pub mod check_open;
pub mod exercise;
pub mod grader;
pub mod next_available_exercise_if_any;
pub mod next_result;
pub mod progress;
pub mod question_source;
pub mod start;
pub mod study;
pub mod submit;
pub mod submit_result;

pub use exercise::Exercise;
pub use grader::Grader;
pub use next_available_exercise_if_any::next_available_exercise_if_any;
pub use question_source::QuestionSource;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Availability<T> {
    Ready(T),
    Empty,
}

#[cfg(test)]
mod tests {
    use tempo_types::{ContentVersion, Revision};

    use crate::{
        abandon::abandon,
        attempt::{Attempt, AttemptStatus},
        exercise::Exercise,
        grader::{Grade, GradeStep, Grader, GraderStart, GradingError},
        next_available_exercise_if_any::next_available_exercise_if_any,
        question_source::QuestionSource,
        study::StudyError,
    };

    use super::*;
    use std::convert::Infallible;
    use submit::submit;

    struct ChoiceGrader;
    impl Grader for ChoiceGrader {
        type Definition = char;
        type State = ();
        type Answer = char;
        type Prompt = String;
        type Feedback = bool;
        fn start(&self, _: &char) -> Result<GraderStart<(), String>, GradingError> {
            Ok(GraderStart {
                state: (),
                prompt: "Choose A or B".into(),
            })
        }
        fn answer(
            &self,
            expected: &char,
            _: &(),
            answer: &char,
        ) -> Result<GradeStep<(), String, bool>, GradingError> {
            let correct = answer == expected;
            Ok(GradeStep {
                state: (),
                prompt: if correct {
                    None
                } else {
                    Some("Try again".into())
                },
                feedback: correct,
                assessment: Some(if correct { Grade::Good } else { Grade::Again }),
                complete: correct,
            })
        }
    }
    struct Source(Option<Exercise<char>>);

    impl QuestionSource for Source {
        type Definition = char;
        type Error = Infallible;
        fn next(&mut self) -> Result<Availability<Exercise<char>>, Infallible> {
            Ok(self
                .0
                .take()
                .map(Availability::Ready)
                .unwrap_or(Availability::Empty))
        }
    }
    fn issued() -> Attempt<(), String> {
        let mut source = Source(Some(Exercise {
            target: "choice".into(),
            version: ContentVersion(1),
            definition: 'B',
        }));
        let Availability::Ready(started) = next_available_exercise_if_any(
            &mut source,
            &ChoiceGrader,
            "dataset".into(),
            "attempt".into(),
        )
        .unwrap() else {
            panic!("fixture ready")
        };
        started.attempt
    }
    #[test]
    fn non_chess_choice_uses_same_protocol_and_sticky_failure() {
        let attempt = issued();
        let miss = submit(&ChoiceGrader, &'B', &attempt, Revision(0), &'A').unwrap();
        assert_eq!(miss.assessment, Some(Grade::Again));
        let correction = submit(&ChoiceGrader, &'B', &miss.attempt, Revision(1), &'B').unwrap();
        assert_eq!(correction.assessment, None);
        assert_eq!(correction.attempt.terminal, Some(Grade::Again));
        assert_eq!(correction.attempt.status, AttemptStatus::Complete);
        let clean = submit(&ChoiceGrader, &'B', &issued(), Revision(0), &'B').unwrap();
        assert_eq!(clean.assessment, Some(Grade::Good));
    }
    #[test]
    fn stale_revision_and_closed_attempt_do_not_call_next_question() {
        let attempt = issued();
        assert!(matches!(
            submit(&ChoiceGrader, &'B', &attempt, Revision(1), &'B'),
            Err(StudyError::RevisionConflict { .. })
        ));
        let done = submit(&ChoiceGrader, &'B', &attempt, Revision(0), &'B').unwrap();
        assert_eq!(
            submit(&ChoiceGrader, &'B', &done.attempt, Revision(1), &'B'),
            Err(StudyError::ClosedAttempt)
        );
        let abandoned = abandon(&attempt, Revision(0)).unwrap();
        assert_eq!(abandoned.terminal, None);
        assert_eq!(abandoned.status, AttemptStatus::Abandoned);
    }
    #[test]
    fn empty_is_a_normal_source_result() {
        assert!(matches!(
            next_available_exercise_if_any(
                &mut Source(None),
                &ChoiceGrader,
                "d".into(),
                "a".into()
            )
            .unwrap(),
            Availability::Empty
        ));
    }
}
