# tempo-study

Generic exercise progression. Study asks a **question source** for content, asks
an injected **grader** what an answer means, and returns an uncommitted transition.
It owns the attempt lifecycle and revision checks. It knows no chess moves, queue
policy, learning schedule, storage implementation, or UI transport.

## A short reading route

1. [exercise.rs](src/exercise.rs): the selected target, content version, and full definition.
2. [attempt.rs](src/attempt.rs): one run through that content, including resumable state.
3. [grader.rs](src/grader.rs): the correctness interface and a complete non-chess implementation example.
4. [question_source.rs](src/question_source.rs): the supply interface and an in-memory source example.
5. [next_available_exercise_if_any.rs](src/next_available_exercise_if_any.rs): how those two ports start a run.
6. [submit.rs](src/submit.rs): what happens when an answer is submitted.
7. [progress.rs](src/progress.rs): what the application should persist and pass to Learning.

Each file starts with when to reach for its concept. Its `#[cfg(test)]` module
contains runnable scenarios. Shared setup is in [test_support.rs](src/test_support.rs);
the actions and assertions stay beside the concept being explained. `lib.rs` is
an index and public export list.

## Choose a value by its job

| Concept / source | When you use it | What its example demonstrates |
| --- | --- | --- |
| [`Exercise<Definition>`](src/exercise.rs) | Return selected content from a question source | Definition contains the answer; safe prompt is a separate value |
| [`QuestionSource`](src/question_source.rs) | Adapt Queue/content selection for Study | Supply selected exercises without creating attempts or grading |
| [`Availability`](src/availability.rs) | Handle currently empty selection | Empty succeeds normally and the source can later refill |
| [`Grader`](src/grader.rs) | Add a subject or grading scheme | A text-answer grader works through the same Study operation |
| [`GraderStart`](src/grader_start.rs) | Return initial state and prompt from `Grader::start` | Starting does not grade an answer |
| [`GradeStep`](src/grade_step.rs) | Propose one answer's state/feedback/result | A miss can emit Again while offering a correction prompt |
| [`Started`](src/started.rs) | Receive selected content and a newly issued attempt | App-supplied ID and exact content version remain associated |
| [`Attempt`](src/attempt.rs) | Resume, answer, or abandon a run | Progress advances state while preserving identity and the original input |
| [`AttemptStatus`](src/attempt_status.rs) | Check presentation lifecycle | Active and Complete are both compatible with a failed outcome |
| [`Progress`](src/progress.rs) | Commit a response and dispatch a new outcome | Correct feedback after a miss emits no second assessment |
| [`StartError`](src/start_error.rs) | Handle failure to obtain/initialize content | Source failure and malformed definition are distinct |
| [`GradingError`](src/grading_error.rs) | Report input the grader cannot evaluate | An unrecognized answer differs from a valid but wrong answer |
| [`StudyError`](src/study_error.rs) | Handle rejected attempt transitions | Stale-view errors carry expected and actual revisions |

The short [NextResult](src/next_result.rs), [GradingResult](src/grading_result.rs),
and [SubmitResult](src/submit_result.rs) aliases make the corresponding signatures
readable; their documentation points to the operations that exercise them.

## Follow the data through one operation

The application calls `next` (the short alias for
`next_available_exercise_if_any`). A `QuestionSource` returns an `Exercise`; the
grader receives its definition and returns initial state plus a safe prompt;
Study returns `Started { exercise, attempt }`. The application then commits the
attempt and its issue-time metadata. Content definitions can contain expected
answers and are not UI payloads.

For an answer, the application supplies the existing attempt, the matching
content definition, an expected response revision, and a typed answer. `submit`
checks the revision/lifecycle, invokes the grader, validates the proposal, and
returns `Progress`. The original attempt remains unchanged. The app commits the
replacement attempt, new assessment if any, learning effect, and receipt together.
Receipt replay and dataset/content currency checks belong to the app and Store.

### Three values that are easy to confuse

- **Feedback** describes the latest answer. A correction can receive positive feedback.
- **`progress.assessment`** is an outcome emitted by this transition. Only this new value should be sent to Learning.
- **`attempt.terminal`** is the first outcome already attached to the attempt. It stays Again after a miss, including after a correct completion.

The first miss can emit Again while the run remains Active. Correcting the answer
can finish presentation, but does not turn that attempt into a Good or emit a
second assessment. A fresh attempt is needed for a clean repeat. `submit.rs`
demonstrates all three stages, as well as an intermediate correct step with no
terminal assessment and invalid grader proposals that Study rejects.

[`abandon`](src/abandon.rs) closes presentation without inventing a grade. Leaving
before an answer stays ungraded; leaving after a miss preserves the earlier Again.
[`check_open`](src/check_open.rs) contains the shared revision/lifecycle/overflow
guard used by both submit and abandon.

## Compatibility and boundaries

Existing root imports (`tempo_study::Attempt`, `Grader`, `next`, `submit`, etc.)
remain available, as do the new named module paths. `attempt::AttemptStatus`,
`grader::{GradeStep, GraderStart, GradingError, GradingResult, Grade}`,
`start::{Started, StartError}`, and `study::StudyError` are compatibility re-exports;
the definitions live in their own named files. There is one shared `Grade` type
in `tempo-types`, so Learning does not acquire a dependency on Study.

The progression rules are real. The application can replace a source or grader
without changing these interfaces. The test support is deliberately non-chess.
No scheduler, database, or UI type enters Study's public contract.

## Run a specific scenario or all examples

From the assembled sibling-repository workspace:

```sh
cargo test -p tempo-study
cargo test -p tempo-study submit::tests::a_miss_records_once_and_correction_keeps_the_original_failure
cargo test -p tempo-study grader::tests
cargo doc -p tempo-study --no-deps --open
```
