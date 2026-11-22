//! Shared setup for usage tests. The scenarios stay beside the API they explain.
//!
//! A two-choice question makes Study's generic behavior visible without chess.

use crate::{
    Attempt, Exercise, GradeStep, Grader, GraderStart, GradingError, start_selected_exercise,
};
use tempo_types::{ContentVersion, Grade};

pub(crate) struct ChoiceGrader;

impl Grader for ChoiceGrader {
    type Definition = char;
    type State = ();
    type Answer = char;
    type Prompt = String;
    type Feedback = bool;

    fn start(&self, expected: &char) -> Result<GraderStart<(), String>, GradingError> {
        if !['A', 'B'].contains(expected) {
            return Err(GradingError::InvalidDefinition("expected A or B".into()));
        }
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
        if !['A', 'B'].contains(answer) {
            return Err(GradingError::InvalidAnswer("choose A or B".into()));
        }
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

pub(crate) fn choice_exercise() -> Exercise<char> {
    Exercise {
        target: "choice".into(),
        version: ContentVersion(1),
        definition: 'B',
    }
}

pub(crate) fn issued_choice() -> Attempt<(), String> {
    start_selected_exercise(&choice_exercise(), &ChoiceGrader, "dataset".into(), || {
        "attempt".into()
    })
    .unwrap()
}
