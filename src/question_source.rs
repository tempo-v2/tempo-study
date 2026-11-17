//! The content-supply port used when Study needs another exercise.
//!
//! Implement [`QuestionSource`] in an application adapter that already knows
//! queue policy and content storage. Study asks only for an exercise, normal
//! emptiness, or an operational error; it never imports Queue or Store.
//! `Empty` is temporary: a later call can return content after eligibility changes.

use crate::{Availability, Exercise};

/// Supply the next selected exercise without exposing selection policy to Study.
pub trait QuestionSource {
    /// The same definition type accepted by the injected grader.
    type Definition;
    /// Operational failure, distinct from a successful empty selection.
    type Error;

    /// Obtain currently eligible content; selecting it does not create an attempt.
    fn next(&mut self) -> Result<Availability<Exercise<Self::Definition>>, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::VecDeque, convert::Infallible};
    use tempo_types::ContentVersion;

    // An in-memory adapter is enough to demonstrate the port. The real app's
    // adapter asks Queue for a target and resolves it through a Store snapshot.
    struct PreparedExercises(VecDeque<Exercise<char>>);

    impl QuestionSource for PreparedExercises {
        type Definition = char;
        type Error = Infallible;

        fn next(&mut self) -> Result<Availability<Exercise<char>>, Infallible> {
            Ok(self
                .0
                .pop_front()
                .map(Availability::Ready)
                .unwrap_or(Availability::Empty))
        }
    }

    #[test]
    fn a_source_chooses_content_without_creating_attempts_or_grading() {
        let first = Exercise {
            target: "choice-1".into(),
            version: ContentVersion(1),
            definition: 'B',
        };
        let second = Exercise {
            target: "choice-2".into(),
            version: ContentVersion(1),
            definition: 'A',
        };
        let mut source = PreparedExercises(VecDeque::from([first.clone(), second.clone()]));

        assert_eq!(source.next().unwrap(), Availability::Ready(first));
        assert_eq!(source.next().unwrap(), Availability::Ready(second));
        assert_eq!(source.next().unwrap(), Availability::Empty);
    }
}
