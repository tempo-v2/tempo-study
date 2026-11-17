//! Generic exercise progression with explicit content and question-source ports.
//!
//! Study coordinates three operations: [`next`] starts an exercise, [`submit()`]
//! proposes an answer transition, and [`abandon()`] closes an attempt. It neither
//! selects queue policy nor decides answer correctness nor writes to storage.
//!
//! Read [`QuestionSource`] for where an exercise comes from, [`Grader`] for how
//! correctness is supplied, and [`Attempt`] / [`Progress`] for what crosses the
//! application boundary. Each concept lives in a named module with usage tests.
//! The application persists a proposed transition and its learning effect.

pub mod abandon;
pub mod attempt;
pub mod attempt_status;
pub mod availability;
pub mod check_open;
pub mod exercise;
pub mod grade_step;
pub mod grader;
pub mod grader_start;
pub mod grading_error;
pub mod grading_result;
pub mod next_available_exercise_if_any;
pub mod next_result;
pub mod progress;
pub mod question_source;
pub mod start;
pub mod start_error;
pub mod started;
pub mod study;
pub mod study_error;
pub mod submit;
pub mod submit_result;

pub use abandon::abandon;
pub use attempt::Attempt;
pub use attempt_status::AttemptStatus;
pub use availability::Availability;
pub use exercise::Exercise;
pub use grade_step::GradeStep;
pub use grader::Grader;
pub use grader_start::GraderStart;
pub use grading_error::GradingError;
pub use grading_result::GradingResult;
pub use next_available_exercise_if_any::next_available_exercise_if_any;
pub use next_available_exercise_if_any::next_available_exercise_if_any as next;
pub use next_result::NextResult;
pub use progress::Progress;
pub use question_source::QuestionSource;
pub use start_error::StartError;
pub use started::Started;
pub use study_error::StudyError;
pub use submit::submit;
pub use submit_result::SubmitResult;

#[cfg(test)]
mod test_support;
