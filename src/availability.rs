//! Distinguish a usable result from normal exhaustion of current choices.
//!
//! Use [`Availability`] when asking for the next exercise. `Empty` means there
//! is nothing eligible now, not that the source failed or can never supply more.
//! Operational failures remain in the surrounding `Result`.

/// A source or start operation can succeed without producing an exercise.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Availability<T> {
    /// An exercise (or its started attempt) is available.
    Ready(T),
    /// Nothing is currently eligible; the caller can ask again later.
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        QuestionSource,
        test_support::{OneExercise, choice_exercise},
    };

    #[test]
    fn exhaustion_is_a_successful_result_and_the_source_can_later_refill() {
        let mut source = OneExercise(None);
        assert_eq!(source.next().unwrap(), Availability::Empty);

        // A real app adapter can supply newly eligible queue content later.
        source.0 = Some(choice_exercise());
        assert!(matches!(source.next().unwrap(), Availability::Ready(_)));
        assert_eq!(source.next().unwrap(), Availability::Empty);
    }
}
