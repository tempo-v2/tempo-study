//! One issued run through an exercise, including resumable grader state.
//!
//! Use [`Attempt`] when answering, resuming, or abandoning that run. The content
//! definition stays separate in [`crate::Exercise`]; the prompt contains only
//! what presentation may show. The app persists attempts together with related
//! learning/history writes. Starting a fresh repeat creates another attempt.

use serde::{Deserialize, Serialize};
use tempo_types::{AttemptId, ContentVersion, DatasetId, Grade, Revision, TargetId};

// Preserve the existing `tempo_study::attempt::AttemptStatus` import path.
pub use crate::attempt_status::AttemptStatus;

/// Generic state for one attempt. `S` and `P` are supplied by the grader.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attempt<S, P> {
    /// This run's identity; unchanged across its answer commands.
    pub id: AttemptId,
    /// The dataset that issued the attempt.
    pub dataset: DatasetId,
    /// The learnable item whose history this run contributes to.
    pub target: TargetId,
    /// The semantic definition used when the attempt was issued.
    pub content_version: ContentVersion,
    /// State version expected by the next answer or abandon command.
    pub revision: Revision,
    /// Private, typed grading state needed to resume the exercise.
    pub state: S,
    /// The current safe presentation, absent after completion/abandonment.
    pub prompt: Option<P>,
    /// The first terminal grade, retained even while correction continues.
    pub terminal: Option<Grade>,
    /// Whether the run can still accept an answer; independent of its grade.
    pub status: AttemptStatus,
}

#[cfg(test)]
mod tests {
    use crate::{
        submit,
        test_support::{ChoiceGrader, issued_choice},
    };
    use tempo_types::Revision;

    #[test]
    fn an_answer_preserves_identity_and_returns_a_new_resumable_state() {
        let issued = issued_choice();
        let progress = submit(&ChoiceGrader, &'B', &issued, Revision(0), &'A').unwrap();

        assert_eq!(progress.attempt.id, issued.id);
        assert_eq!(progress.attempt.target, issued.target);
        assert_eq!(progress.attempt.content_version, issued.content_version);
        assert_eq!(progress.attempt.revision, Revision(1));
        assert_eq!(progress.attempt.prompt.as_deref(), Some("Try again"));
        // Study proposes the replacement. It does not mutate the caller's copy.
        assert_eq!(issued.revision, Revision(0));
        assert_eq!(issued.terminal, None);
    }
}
