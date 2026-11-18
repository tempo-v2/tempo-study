//! Readable return type for one answer transition.
//!
//! Use this alias in an adapter around [`crate::submit()`]. `Ok(Progress)` can
//! describe a correct or incorrect answer; `Err(StudyError)` means no valid
//! transition was proposed. The executable scenarios live in [`crate::submit()`].

use crate::{Progress, StudyError};

/// Proposed progress or a rejected transition, with all grader types retained.
pub type SubmitResult<S, P, F> = Result<Progress<S, P, F>, StudyError>;
