//! Whether presentation of an attempt can still accept an answer.
//!
//! Use [`AttemptStatus`] to decide whether a run is active, finished, or left
//! early. It does not say whether the run succeeded: an active attempt can
//! already have a sticky `Again`, and a completed correction can still be failed.

use serde::{Deserialize, Serialize};

/// The presentation lifecycle, independent of the attempt's terminal grade.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptStatus {
    /// The grader has a prompt and can accept another answer.
    Active,
    /// Presentation finished; no further answers are accepted.
    Complete,
    /// The user left the attempt; no further answers are accepted.
    Abandoned,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        submit,
        test_support::{ChoiceGrader, issued_choice},
    };
    use tempo_types::{Grade, Revision};

    #[test]
    fn an_active_correction_and_a_completed_correction_can_both_be_failed() {
        let miss = submit(&ChoiceGrader, &'B', &issued_choice(), Revision(0), &'A').unwrap();
        assert_eq!(miss.attempt.status, AttemptStatus::Active);
        assert_eq!(miss.attempt.terminal, Some(Grade::Again));

        let corrected = submit(&ChoiceGrader, &'B', &miss.attempt, Revision(1), &'B').unwrap();
        assert_eq!(corrected.attempt.status, AttemptStatus::Complete);
        assert_eq!(corrected.attempt.terminal, Some(Grade::Again));
    }
}
