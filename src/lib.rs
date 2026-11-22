//! Progress exercises after the application has selected their content.
//!
//! [`start_selected_exercise()`] initializes an attempt through its grader.
//! [`submit()`] proposes the next state and any first assessment, and
//! [`abandon()`] ends the attempt without adding a grade. The application
//! persists accepted transitions and any related Learning effect.

pub mod abandon;
pub mod attempt;
pub mod attempt_status;
pub mod check_open;
mod complete_ungraded;
pub mod exercise;
mod give_up;
pub mod grade_step;

pub mod grader;
pub mod grader_start;
pub mod grading_error;
pub mod grading_result;
pub mod progress;
mod start_selected_exercise;
pub mod study;
pub mod study_error;
pub mod submit;
pub mod submit_result;

pub use abandon::abandon;
pub use attempt::Attempt;
pub use attempt_status::AttemptStatus;
pub use complete_ungraded::complete_ungraded;
pub use exercise::Exercise;
pub use give_up::{concede, give_up};
pub use grade_step::GradeStep;

pub use grader::Grader;
pub use grader_start::GraderStart;
pub use grading_error::GradingError;
pub use grading_result::GradingResult;
pub use progress::Progress;
pub use start_selected_exercise::start_selected_exercise;
pub use study_error::StudyError;
pub use submit::submit;
pub use submit_result::SubmitResult;

#[cfg(test)]
mod test_support;
