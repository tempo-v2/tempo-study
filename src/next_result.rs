//! Readable return type for starting the next available exercise.
//!
//! Use this alias for wrappers around [`crate::next`]. There are three meaningful
//! cases: `Ok(Ready(Started))`, `Ok(Empty)`, and `Err(StartError)`.
//! The scenarios live beside [`crate::next_available_exercise_if_any()`] and
//! [`crate::start_error`], where those cases are produced.

use crate::{Availability, StartError, Started};

/// A new attempt, normal current emptiness, or a typed start failure.
pub type NextResult<D, S, P, E> = Result<Availability<Started<D, S, P>>, StartError<E>>;
