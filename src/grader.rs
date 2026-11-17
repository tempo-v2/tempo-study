use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

/// Represents the possible grades for an attempt.
/// `Good` indicates a correct or satisfactory response.
/// `Again` indicates an incorrect or unsatisfactory response
/// that requires retrying.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Grade {
    Good,
    Again,
}

/// Represents the initial state and prompt when starting a grading session.
/// Contains the initial state and the prompt to be presented to the user.
/// This struct is returned by the `start` method of the `Grader` trait.
pub struct GraderStart<S, P> {
    pub state: S,
    pub prompt: P,
}

/// Represents a single step in the grading process.
/// Contains the current state, optional prompt, feedback, assessment,
/// and completion status.
///
/// This allows for tracking the progress of a grading session step by step.
pub struct GradeStep<S, P, F> {
    pub state: S,
    pub prompt: Option<P>,
    pub feedback: F,
    pub assessment: Option<Grade>,
    pub complete: bool,
}

/// Defines the behavior of a grader.
/// A grader is responsible for managing the grading process, including
/// starting a session and handling answers.
pub trait Grader {
    /// The definition of the item being graded--it describes the content or task that the grader will evaluate.
    type Definition;
    /// The state of the grading session, which can be updated as the user interacts with the grader.
    /// Must implement `Clone` to allow for state management across steps.
    type State: Clone;
    /// The type of answer provided by the user during the grading session.
    /// This allows the grader to evaluate the user's response and provide feedback accordingly.
    type Answer;
    /// The type of prompt presented to the user during the grading session.
    /// Must implement `Clone` to allow for reuse across steps.
    type Prompt: Clone;
    /// The type of feedback provided by the grader after evaluating an answer.
    /// Must implement `Clone` to allow for reuse across steps.
    type Feedback: Clone;

    /// Starts a new grading session for the given definition.
    fn start(
        &self,
        definition: &Self::Definition,
    ) -> Result<GraderStart<Self::State, Self::Prompt>, GradingError>;

    /// Handles an answer provided by the user during the grading session.
    /// Evaluates the answer, updates the state, and returns the next step in the grading process.
    fn answer(
        &self,
        definition: &Self::Definition,
        state: &Self::State,
        answer: &Self::Answer,
    ) -> GradingResult<Self::State, Self::Prompt, Self::Feedback>;
}

/// Represents the result of a grading step, which can be either a successful `GradeStep` or a `GradingError`.
pub type GradingResult<S, P, F> = Result<GradeStep<S, P, F>, GradingError>;

/// Represents an error that can occur during the grading process.
/// Variants include invalid definitions, states, or answers provided during grading.
/// Each variant contains a descriptive message explaining the error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GradingError {
    InvalidDefinition(String),
    InvalidState(String),
    InvalidAnswer(String),
}

/// Formats the `GradingError` for display purposes.
/// This implementation simply uses the `Debug` representation of the error.
impl fmt::Display for GradingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Implements the `Error` trait for `GradingError`, allowing it to be used as a standard error type in Rust.
impl Error for GradingError {}
