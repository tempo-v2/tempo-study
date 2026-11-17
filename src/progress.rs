use serde::{Deserialize, Serialize};

use crate::{attempt::Attempt, grader::Grade};

/// Represents the progress of an attempt, including the updated attempt state,
/// feedback from the grader, and any newly emitted assessment.
///
/// # Expected behavior
/// The `Progress` struct captures the result of applying a grader's feedback to an attempt.
/// The `attempt` field reflects the updated state, `feedback` contains the grader's feedback,
/// and `assessment` includes any newly emitted assessment, if applicable.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Progress<S, P, F> {
    /// The updated attempt state after applying the grader's feedback.
    pub attempt: Attempt<S, P>,
    /// Feedback from the grader corresponding to the latest attempt update.
    pub feedback: F,
    /// Newly emitted assessment only. `attempt.terminal` is the sticky lifetime result.
    pub assessment: Option<Grade>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attempt::AttemptStatus;

    #[test]
    fn test_progress_creation() {
        let attempt = Attempt {
            revision: 1.into(),
            status: AttemptStatus::Active,
            state: (),
            prompt: None,
            terminal: None,
            id: todo!(),
            dataset: todo!(),
            target: todo!(),
            content_version: todo!(),
        };
        let progress = Progress {
            attempt: attempt.clone(),
            feedback: true,
            assessment: Some(Grade::Good),
        };
        assert_eq!(progress.attempt, attempt);
        assert_eq!(progress.feedback, true);
        assert_eq!(progress.assessment, Some(Grade::Good));
    }
}
