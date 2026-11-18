//! Compatibility imports for the start result values.
//!
//! Open [`crate::started`] for selected content plus its new attempt, and
//! [`crate::start_error`] for source/initialization failures. The operation
//! itself lives in [`crate::next_available_exercise_if_any()`].

pub use crate::start_error::StartError;
pub use crate::started::Started;
