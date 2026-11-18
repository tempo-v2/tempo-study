//! The answer transition Study proposes for the app to commit.
//!
//! Use [`Progress`] after [`crate::submit()`]. Persist its attempt and project its
//! feedback to presentation. Feed `assessment` to Learning only when it is `Some`:
//! it means **newly emitted now**. `attempt.terminal` is the lifetime outcome and
//! must not be fed to Learning again on every correction.

use crate::Attempt;
use serde::{Deserialize, Serialize};
use tempo_types::Grade;

/// Proposed attempt state, per-answer feedback, and at most one new outcome.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Progress<S, P, F> {
    /// Replacement attempt state; inputs to Study remain unchanged.
    pub attempt: Attempt<S, P>,
    /// Feedback for the latest answer, even if no new assessment was emitted.
    pub feedback: F,
    /// Newly emitted assessment only; absent after an earlier terminal outcome.
    pub assessment: Option<Grade>,
}

#[cfg(test)]
mod tests {
    use crate::{
        submit,
        test_support::{ChoiceGrader, issued_choice},
    };
    use tempo_types::{Grade, Revision};

    #[test]
    fn a_consumer_records_only_new_assessments_while_still_showing_correction_feedback() {
        let miss = submit(&ChoiceGrader, &'B', &issued_choice(), Revision(0), &'A').unwrap();
        let correction = submit(&ChoiceGrader, &'B', &miss.attempt, Revision(1), &'B').unwrap();

        let outcomes: Vec<_> = [miss.assessment, correction.assessment]
            .into_iter()
            .flatten()
            .collect();
        assert_eq!(outcomes, vec![Grade::Again]);
        assert!(correction.feedback);
        assert_eq!(correction.attempt.terminal, Some(Grade::Again));
        // The app can show correct feedback now without recording a second grade.
    }
}
