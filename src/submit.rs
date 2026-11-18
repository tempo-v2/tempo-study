//! Validate one answer transition while keeping assessment emission exactly once.
//!
//! Use [`submit`] after the application has found the attempt, checked current
//! content/dataset identity, and handled receipt replay. Study checks the expected
//! response revision, asks the grader, and returns a proposal. The app commits
//! its attempt, any new assessment, Learning effect, and receipt atomically.
//!
//! No queue policy, storage access, or scheduling decision enters this function.

use crate::{
    Attempt, AttemptStatus, Grader, Progress, StudyError, SubmitResult, check_open::check_open,
};
use tempo_types::{Grade, Revision};

/// Propose progress for one answer without mutating the caller's attempt.
///
/// The first terminal grade is sticky. Later correction can finish presentation
/// but emits no second assessment; a new attempt is needed for a clean `Good`.
/// The app is responsible for supplying the definition pinned to this attempt.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        GradeStep, GraderStart, GradingError,
        test_support::{ChoiceGrader, issued_choice},
    };

    #[test]
    fn a_miss_records_once_and_correction_keeps_the_original_failure() {
        let attempt = issued_choice();
        let miss = submit(&ChoiceGrader, &'B', &attempt, Revision(0), &'A').unwrap();
        assert_eq!(miss.assessment, Some(Grade::Again));
        assert_eq!(miss.attempt.status, AttemptStatus::Active);

        let correction = submit(&ChoiceGrader, &'B', &miss.attempt, Revision(1), &'B').unwrap();
        assert!(correction.feedback);
        assert_eq!(correction.assessment, None);
        assert_eq!(correction.attempt.terminal, Some(Grade::Again));
        assert_eq!(correction.attempt.status, AttemptStatus::Complete);
        assert_eq!(correction.attempt.prompt, None);
    }

    #[test]
    fn a_fresh_attempt_at_the_same_target_can_record_a_clean_success() {
        let first = issued_choice();
        let failed = submit(&ChoiceGrader, &'B', &first, Revision(0), &'A').unwrap();
        let mut repeat = issued_choice();
        repeat.id = "fresh-repeat".into();
        assert_eq!(repeat.target, first.target);
        assert_ne!(repeat.id, first.id);

        let clean = submit(&ChoiceGrader, &'B', &repeat, Revision(0), &'B').unwrap();
        assert_eq!(failed.attempt.terminal, Some(Grade::Again));
        assert_eq!(clean.assessment, Some(Grade::Good));
    }

    #[test]
    fn an_old_response_revision_cannot_answer_the_current_prompt() {
        let miss = submit(&ChoiceGrader, &'B', &issued_choice(), Revision(0), &'A').unwrap();
        assert_eq!(
            submit(&ChoiceGrader, &'B', &miss.attempt, Revision(0), &'B'),
            Err(StudyError::RevisionConflict {
                expected: Revision(0),
                actual: Revision(1)
            })
        );
    }

    #[test]
    fn a_completed_attempt_rejects_another_answer_even_with_the_current_revision() {
        let done = submit(&ChoiceGrader, &'B', &issued_choice(), Revision(0), &'B').unwrap();
        assert_eq!(
            submit(&ChoiceGrader, &'B', &done.attempt, Revision(1), &'B'),
            Err(StudyError::ClosedAttempt)
        );
    }

    #[test]
    fn an_ungradable_answer_leaves_the_callers_state_unchanged() {
        let attempt = issued_choice();
        let original = attempt.clone();
        assert!(matches!(
            submit(&ChoiceGrader, &'B', &attempt, Revision(0), &'Z'),
            Err(StudyError::Grading(GradingError::InvalidAnswer(_)))
        ));
        assert_eq!(attempt, original);
    }

    // Deliberately configurable output lets these tests check the Study/grader
    // contract without hiding the interesting invalid combinations in a fixture.
    struct StepGrader {
        complete: bool,
        has_prompt: bool,
        assessment: Option<Grade>,
    }

    impl Grader for StepGrader {
        type Definition = char;
        type State = ();
        type Answer = char;
        type Prompt = String;
        type Feedback = bool;

        fn start(&self, _: &char) -> Result<GraderStart<(), String>, GradingError> {
            Ok(GraderStart {
                state: (),
                prompt: "First question".into(),
            })
        }
        fn answer(
            &self,
            _: &char,
            _: &(),
            _: &char,
        ) -> Result<GradeStep<(), String, bool>, GradingError> {
            Ok(GradeStep {
                state: (),
                prompt: self.has_prompt.then(|| "Next question".into()),
                feedback: true,
                assessment: self.assessment,
                complete: self.complete,
            })
        }
    }

    #[test]
    fn an_intermediate_correct_step_can_advance_without_a_terminal_assessment() {
        let grader = StepGrader {
            complete: false,
            has_prompt: true,
            assessment: None,
        };
        let progress = submit(&grader, &'B', &issued_choice(), Revision(0), &'B').unwrap();
        assert!(progress.feedback);
        assert_eq!(progress.assessment, None);
        assert_eq!(progress.attempt.terminal, None);
        assert_eq!(progress.attempt.status, AttemptStatus::Active);
        assert_eq!(progress.attempt.revision, Revision(1));
    }

    #[test]
    fn inconsistent_grader_proposals_are_rejected_before_the_app_can_commit_them() {
        let invalid = [
            ("complete with a next prompt", true, true, Some(Grade::Good)),
            ("active without a prompt", false, false, None),
            (
                "successful before completion",
                false,
                true,
                Some(Grade::Good),
            ),
            ("complete without any terminal result", true, false, None),
        ];
        for (description, complete, has_prompt, assessment) in invalid {
            let grader = StepGrader {
                complete,
                has_prompt,
                assessment,
            };
            assert_eq!(
                submit(&grader, &'B', &issued_choice(), Revision(0), &'B'),
                Err(StudyError::InvalidGraderTransition),
                "{description}"
            );
        }
    }
}
