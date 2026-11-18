//! The first internal state and safe prompt produced by a grader.
//!
//! Return [`GraderStart`] from [`crate::Grader::start`]. Study wraps these values
//! in an attempt, supplying identity and revision metadata itself. A start does
//! not produce an assessment or write learning history.

/// Initial grading state and presentation, before any answer has been submitted.
pub struct GraderStart<S, P> {
    /// Grader-owned state that will be passed to the first answer call.
    pub state: S,
    /// The first question the user may see, without a hidden answer key.
    pub prompt: P,
}

#[cfg(test)]
mod tests {
    use crate::{Grader, test_support::ChoiceGrader};

    #[test]
    fn starting_creates_a_question_without_grading_an_answer() {
        let start = ChoiceGrader.start(&'B').unwrap();
        assert_eq!(start.prompt, "Choose A or B");
        let first_answer = ChoiceGrader.answer(&'B', &start.state, &'B').unwrap();
        assert!(first_answer.complete);
        assert!(first_answer.feedback);
    }
}
