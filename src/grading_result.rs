//! Return type for a grader's answer operation.
//!
//! Use this alias when implementing [`crate::Grader::answer`] to keep its
//! signature readable: `Ok` carries a [`crate::GradeStep`], while `Err` means
//! the input could not be graded. A wrong answer still belongs in `Ok`.
//! See the implementation and same-module example in [`crate::grader`].

use crate::{GradeStep, GradingError};

/// A typed grading step or an input/state error.
pub type GradingResult<S, P, F> = Result<GradeStep<S, P, F>, GradingError>;
