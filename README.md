# tempo-study

Study owns attempt progression and revision checks. The application supplies
selected content and an injected grader. Study proposes the state to persist.

## Start an issued attempt

An [Exercise](src/exercise.rs) contains the selected target, content version,
and full definition. [start_selected_exercise](src/start_selected_exercise.rs)
passes the definition to the [Grader](src/grader.rs), allocates the attempt ID
after initialization, and returns an active [Attempt](src/attempt.rs). Its
prompt is safe for presentation. The definition may contain expected answers.

The application and Store own selection, acceptance, Learning effects and
receipt replay. The grader owns answer correctness.

## Submit and retain the first assessment

[submit](src/submit.rs) takes an active attempt, its matching definition, the
expected revision, and a typed answer. It checks the revision and lifecycle,
calls the grader, and returns a proposed [Progress](src/progress.rs).

- Feedback describes the latest answer.
- `progress.assessment` is the new outcome, if any, that the application sends
  to Learning.
- `attempt.terminal` preserves the first assessment. A miss stays Again through
  a correct completion.

[abandon](src/abandon.rs) closes an attempt without another assessment.
[check_open](src/check_open.rs) contains the shared revision, lifecycle and
overflow checks.

## Tests

From the assembled workspace, run:

```sh
cargo test -p tempo-study
```
