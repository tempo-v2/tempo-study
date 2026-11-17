//! The correctness port: implement this when adding a grading scheme.
//!
//! Reach for [`Grader`] when content defines what counts as success, how many
//! decisions a run needs, and what prompt follows an answer. Study calls this
//! port but never interprets chess moves, answer keys, or scoring rules.
//! Implementations receive values and return proposals; persistence, queue
//! selection, and scheduling belong elsewhere.

// Keep existing imports valid while each concept has its own source file.
pub use crate::grade_step::GradeStep;
pub use crate::grader_start::GraderStart;
pub use crate::grading_error::GradingError;
pub use crate::grading_result::GradingResult;
pub use tempo_types::Grade;

/// Supply correctness and presentation for one family of exercises.
///
/// Choose concrete associated types once per implementation. For example, a
/// chess grader can use a chess definition/state/answer/prompt; the example below
/// uses text. Study handles both through the same operations.
pub trait Grader {
    /// Full content, including the expected answers; never a UI wire payload.
    type Definition;
    /// Resumable private state, such as the index of the current decision.
    type State: Clone;
    /// One submitted response, such as a move or selected option.
    type Answer;
    /// Safe presentation of the current question, without an answer key.
    type Prompt: Clone;
    /// Feedback about this answer, separate from the terminal outcome.
    type Feedback: Clone;

    /// Validate a definition and produce the first state and prompt.
    fn start(
        &self,
        definition: &Self::Definition,
    ) -> Result<GraderStart<Self::State, Self::Prompt>, GradingError>;

    /// Propose the result of one answer without changing the input state.
    ///
    /// A wrong but well-formed answer is a step, not a `GradingError`. See
    /// [`GradeStep`] for the prompt/completion/assessment consistency rules.
    fn answer(
        &self,
        definition: &Self::Definition,
        state: &Self::State,
        answer: &Self::Answer,
    ) -> GradingResult<Self::State, Self::Prompt, Self::Feedback>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Attempt, AttemptStatus, submit};
    use tempo_types::{ContentVersion, Revision};

    // A complete example of adding a new subject without modifying Study.
    struct WordGrader;

    impl Grader for WordGrader {
        type Definition = &'static str;
        type State = usize;
        type Answer = String;
        type Prompt = String;
        type Feedback = bool;

        fn start(&self, _: &&'static str) -> Result<GraderStart<usize, String>, GradingError> {
            Ok(GraderStart {
                state: 0,
                prompt: "Translate 'cat' into German".into(),
            })
        }

        fn answer(
            &self,
            expected: &&'static str,
            state: &usize,
            answer: &String,
        ) -> GradingResult<usize, String, bool> {
            let correct = answer == expected;
            Ok(GradeStep {
                state: state + 1,
                prompt: if correct {
                    None
                } else {
                    Some("Try the translation again".into())
                },
                feedback: correct,
                assessment: Some(if correct { Grade::Good } else { Grade::Again }),
                complete: correct,
            })
        }
    }

    #[test]
    fn a_text_grader_plugs_into_the_same_submit_operation_as_any_other_subject() {
        let definition = "Katze";
        let start = WordGrader.start(&definition).unwrap();
        let attempt = Attempt {
            id: "translation-1".into(),
            dataset: "dataset".into(),
            target: "word-cat".into(),
            content_version: ContentVersion(1),
            revision: Revision(0),
            state: start.state,
            prompt: Some(start.prompt),
            terminal: None,
            status: AttemptStatus::Active,
        };
        let answer = String::from("Katze");
        let progress = submit(&WordGrader, &definition, &attempt, Revision(0), &answer).unwrap();

        assert_eq!(progress.attempt.state, 1);
        assert_eq!(progress.assessment, Some(Grade::Good));
        assert_eq!(progress.attempt.status, AttemptStatus::Complete);
        assert!(progress.feedback);
    }
}
