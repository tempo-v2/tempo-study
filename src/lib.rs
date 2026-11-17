//! Generic attempt progression. Exercise-specific correctness is supplied by a grader.
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};
use tempo_types::{AttemptId, ContentVersion, DatasetId, Grade, Revision, TargetId};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise<D> {
    pub target: TargetId,
    pub version: ContentVersion,
    pub definition: D,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Availability<T> {
    Ready(T),
    Empty,
}

/// Implemented by the application adapter; Study never imports Queue or Store.
pub trait QuestionSource {
    type Definition;
    type Error;
    fn next(&mut self) -> Result<Availability<Exercise<Self::Definition>>, Self::Error>;
}

pub struct GraderStart<S, P> {
    pub state: S,
    pub prompt: P,
}

pub struct GradeStep<S, P, F> {
    pub state: S,
    pub prompt: Option<P>,
    pub feedback: F,
    pub assessment: Option<Grade>,
    pub complete: bool,
}

pub type GradingResult<S, P, F> = Result<GradeStep<S, P, F>, GradingError>;
pub type NextResult<D, S, P, E> = Result<Availability<Started<D, S, P>>, StartError<E>>;
pub type SubmitResult<S, P, F> = Result<Progress<S, P, F>, StudyError>;

pub trait Grader {
    type Definition;
    type State: Clone;
    type Answer;
    type Prompt: Clone;
    type Feedback: Clone;
    fn start(
        &self,
        definition: &Self::Definition,
    ) -> Result<GraderStart<Self::State, Self::Prompt>, GradingError>;
    fn answer(
        &self,
        definition: &Self::Definition,
        state: &Self::State,
        answer: &Self::Answer,
    ) -> GradingResult<Self::State, Self::Prompt, Self::Feedback>;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptStatus {
    Active,
    Complete,
    Abandoned,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attempt<S, P> {
    pub id: AttemptId,
    pub dataset: DatasetId,
    pub target: TargetId,
    pub content_version: ContentVersion,
    pub revision: Revision,
    pub state: S,
    pub prompt: Option<P>,
    pub terminal: Option<Grade>,
    pub status: AttemptStatus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Started<D, S, P> {
    pub exercise: Exercise<D>,
    pub attempt: Attempt<S, P>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Progress<S, P, F> {
    pub attempt: Attempt<S, P>,
    pub feedback: F,
    /// Newly emitted assessment only. `attempt.terminal` is the sticky lifetime result.
    pub assessment: Option<Grade>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GradingError {
    InvalidDefinition(String),
    InvalidState(String),
    InvalidAnswer(String),
}
impl fmt::Display for GradingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl Error for GradingError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StudyError {
    RevisionConflict {
        expected: Revision,
        actual: Revision,
    },
    ClosedAttempt,
    RevisionExhausted,
    InvalidGraderTransition,
    Grading(GradingError),
}
impl fmt::Display for StudyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl Error for StudyError {}
impl From<GradingError> for StudyError {
    fn from(value: GradingError) -> Self {
        Self::Grading(value)
    }
}

#[derive(Debug)]
pub enum StartError<E> {
    Source(E),
    Grading(GradingError),
}
impl<E: fmt::Display> fmt::Display for StartError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source(e) => write!(f, "question source: {e}"),
            Self::Grading(e) => write!(f, "grader: {e}"),
        }
    }
}
impl<E: Error + 'static> Error for StartError<E> {}

pub fn next<S, G>(
    source: &mut S,
    grader: &G,
    dataset: DatasetId,
    id: AttemptId,
) -> NextResult<G::Definition, G::State, G::Prompt, S::Error>
where
    S: QuestionSource<Definition = G::Definition>,
    G: Grader + ?Sized,
{
    let exercise = match source.next().map_err(StartError::Source)? {
        Availability::Empty => return Ok(Availability::Empty),
        Availability::Ready(exercise) => exercise,
    };
    let start = grader
        .start(&exercise.definition)
        .map_err(StartError::Grading)?;
    let attempt = Attempt {
        id,
        dataset,
        target: exercise.target.clone(),
        content_version: exercise.version,
        revision: Revision(0),
        state: start.state,
        prompt: Some(start.prompt),
        terminal: None,
        status: AttemptStatus::Active,
    };
    Ok(Availability::Ready(Started { exercise, attempt }))
}

fn check_open<S, P>(attempt: &Attempt<S, P>, expected: Revision) -> Result<Revision, StudyError> {
    if attempt.revision != expected {
        return Err(StudyError::RevisionConflict {
            expected,
            actual: attempt.revision,
        });
    }
    if attempt.status != AttemptStatus::Active {
        return Err(StudyError::ClosedAttempt);
    }
    attempt
        .revision
        .0
        .checked_add(1)
        .map(Revision)
        .ok_or(StudyError::RevisionExhausted)
}

pub fn submit<G: Grader + ?Sized>(
    grader: &G,
    definition: &G::Definition,
    attempt: &Attempt<G::State, G::Prompt>,
    expected_revision: Revision,
    answer: &G::Answer,
) -> SubmitResult<G::State, G::Prompt, G::Feedback> {
    let revision = check_open(attempt, expected_revision)?;
    let step = grader.answer(definition, &attempt.state, answer)?;
    // Completed presentation has no next question. Success cannot precede completion.
    if step.complete == step.prompt.is_some()
        || (step.assessment == Some(Grade::Good) && !step.complete)
        || (step.complete && step.assessment.is_none() && attempt.terminal.is_none())
    {
        return Err(StudyError::InvalidGraderTransition);
    }
    let assessment = if attempt.terminal.is_none() {
        step.assessment
    } else {
        None
    };
    let mut updated = attempt.clone();
    updated.revision = revision;
    updated.state = step.state;
    updated.prompt = step.prompt;
    updated.terminal = attempt.terminal.or(assessment);
    updated.status = if step.complete {
        AttemptStatus::Complete
    } else {
        AttemptStatus::Active
    };
    Ok(Progress {
        attempt: updated,
        feedback: step.feedback,
        assessment,
    })
}

pub fn abandon<S: Clone, P: Clone>(
    attempt: &Attempt<S, P>,
    expected_revision: Revision,
) -> Result<Attempt<S, P>, StudyError> {
    let revision = check_open(attempt, expected_revision)?;
    let mut updated = attempt.clone();
    updated.revision = revision;
    updated.status = AttemptStatus::Abandoned;
    updated.prompt = None;
    Ok(updated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::Infallible;
    struct ChoiceGrader;
    impl Grader for ChoiceGrader {
        type Definition = char;
        type State = ();
        type Answer = char;
        type Prompt = String;
        type Feedback = bool;
        fn start(&self, _: &char) -> Result<GraderStart<(), String>, GradingError> {
            Ok(GraderStart {
                state: (),
                prompt: "Choose A or B".into(),
            })
        }
        fn answer(
            &self,
            expected: &char,
            _: &(),
            answer: &char,
        ) -> Result<GradeStep<(), String, bool>, GradingError> {
            let correct = answer == expected;
            Ok(GradeStep {
                state: (),
                prompt: if correct {
                    None
                } else {
                    Some("Try again".into())
                },
                feedback: correct,
                assessment: Some(if correct { Grade::Good } else { Grade::Again }),
                complete: correct,
            })
        }
    }
    struct Source(Option<Exercise<char>>);
    impl QuestionSource for Source {
        type Definition = char;
        type Error = Infallible;
        fn next(&mut self) -> Result<Availability<Exercise<char>>, Infallible> {
            Ok(self
                .0
                .take()
                .map(Availability::Ready)
                .unwrap_or(Availability::Empty))
        }
    }
    fn issued() -> Attempt<(), String> {
        let mut source = Source(Some(Exercise {
            target: "choice".into(),
            version: ContentVersion(1),
            definition: 'B',
        }));
        let Availability::Ready(started) = next(
            &mut source,
            &ChoiceGrader,
            "dataset".into(),
            "attempt".into(),
        )
        .unwrap() else {
            panic!("fixture ready")
        };
        started.attempt
    }
    #[test]
    fn non_chess_choice_uses_same_protocol_and_sticky_failure() {
        let attempt = issued();
        let miss = submit(&ChoiceGrader, &'B', &attempt, Revision(0), &'A').unwrap();
        assert_eq!(miss.assessment, Some(Grade::Again));
        let correction = submit(&ChoiceGrader, &'B', &miss.attempt, Revision(1), &'B').unwrap();
        assert_eq!(correction.assessment, None);
        assert_eq!(correction.attempt.terminal, Some(Grade::Again));
        assert_eq!(correction.attempt.status, AttemptStatus::Complete);
        let clean = submit(&ChoiceGrader, &'B', &issued(), Revision(0), &'B').unwrap();
        assert_eq!(clean.assessment, Some(Grade::Good));
    }
    #[test]
    fn stale_revision_and_closed_attempt_do_not_call_next_question() {
        let attempt = issued();
        assert!(matches!(
            submit(&ChoiceGrader, &'B', &attempt, Revision(1), &'B'),
            Err(StudyError::RevisionConflict { .. })
        ));
        let done = submit(&ChoiceGrader, &'B', &attempt, Revision(0), &'B').unwrap();
        assert_eq!(
            submit(&ChoiceGrader, &'B', &done.attempt, Revision(1), &'B'),
            Err(StudyError::ClosedAttempt)
        );
        let abandoned = abandon(&attempt, Revision(0)).unwrap();
        assert_eq!(abandoned.terminal, None);
        assert_eq!(abandoned.status, AttemptStatus::Abandoned);
    }
    #[test]
    fn empty_is_a_normal_source_result() {
        assert!(matches!(
            next(&mut Source(None), &ChoiceGrader, "d".into(), "a".into()).unwrap(),
            Availability::Empty
        ));
    }
}
