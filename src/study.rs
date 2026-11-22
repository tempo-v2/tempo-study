//! Compatibility import for Study's transition error.
//!
//! The definition and caller-facing usage scenario live in [`crate::study_error`].
//! Study's operations are [`crate::start_selected_exercise()`], [`crate::submit()`], and [`crate::abandon()`].

pub use crate::study_error::StudyError;
