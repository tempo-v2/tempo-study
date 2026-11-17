//! A grader's proposed result for one answer, before Study validates it.
//!
//! Return [`GradeStep`] from [`crate::Grader::answer`]. `feedback` describes this
//! answer; `assessment` proposes a learning outcome; `complete` says presentation
//! is finished. These are separate: a first miss can emit `Again` and still have
//! another prompt. Study keeps the first terminal grade and emits it only once.

use tempo_types::Grade;

/// Grader → Study proposal for advancing one answer.
///
/// An active step must have a prompt; a complete step must not. `Good` requires
/// completion. Completion requires either an assessment here or one already
/// recorded on the attempt. [`crate::submit()`] enforces these rules.
pub struct GradeStep<S, P, F> {
    /// Replacement grader state for resuming after this answer.
    pub state: S,
    /// Next safe question, or `None` if presentation has finished.
    pub prompt: Option<P>,
    /// Per-answer explanation or correctness feedback.
    pub feedback: F,
    /// Proposed terminal outcome; may be absent on an intermediate correct move.
    pub assessment: Option<Grade>,
    /// Whether there are no more answers to accept in this run.
    pub complete: bool,
}

#[cfg(test)]
mod tests {
    use crate::{Grader, test_support::ChoiceGrader};
    use tempo_types::Grade;

    #[test]
    fn a_miss_can_propose_an_assessment_while_offering_a_correction_prompt() {
        let step = ChoiceGrader.answer(&'B', &(), &'A').unwrap();
        assert!(!step.feedback);
        assert_eq!(step.assessment, Some(Grade::Again));
        assert!(!step.complete);
        assert_eq!(step.prompt.as_deref(), Some("Try again"));
    }
}
