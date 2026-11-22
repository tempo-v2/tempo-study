//! A selected learnable target plus the exact definition its grader will use.
//!
//! The application supplies selected content to Study, which passes its
//! definition to the grader. The definition may include expected answers.
//! The grader creates a separate prompt for presentation.

use serde::{Deserialize, Serialize};
use tempo_types::{ContentVersion, TargetId};

/// Selected content, before an attempt has been issued.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise<D> {
    /// Stable identity across repeats and compatible metadata changes.
    pub target: TargetId,
    /// The semantic revision of this definition.
    pub version: ContentVersion,
    /// Grader-specific content, potentially including expected answers.
    pub definition: D,
}
