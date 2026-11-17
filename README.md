# tempo-study

Generic exercise and attempt progression. Correctness is supplied through a
`Grader`; obtaining an exercise is supplied through a `QuestionSource`. This
crate imports `tempo-types` and serialization support. It has no dependency on
Chess, Queue, Learning, Store, an application, a browser, or a game provider.

## Public contract

| Interface / value | Responsibility |
| --- | --- |
| `QuestionSource` | Return `Ready(Exercise<Definition>)`, normal `Empty`, or a typed operational error. |
| `Grader` | Supply associated definition, state, answer, prompt and feedback types; start an exercise and grade one answer. |
| `next` | Ask the source, initialize the grader, and return an uncommitted `Started` attempt. |
| `submit` | Check the expected response revision, invoke the grader, and return a proposed `Progress`. |
| `abandon` | Close an active attempt without inventing an assessment. |
| `Attempt<State, Prompt>` | Dataset/target/content identity, response revision, typed state, safe prompt, sticky terminal result and presentation status. |
| `Progress` | Updated attempt, feedback and an optional **newly emitted** assessment. |

A terminal assessment and presentation completion are separate. The first miss
can emit `Again` immediately while the user continues correcting the exercise.
That attempt never emits another assessment or becomes a successful attempt.
A fresh attempt is required for a clean repeat. A successful assessment requires
completed presentation. Stale answer revisions and closed attempts are errors.

These functions do not persist or mutate their inputs. The application commits
the returned attempt, assessment, learning effect and submission receipt together.
It owns receipt replay and validates dataset/content currency before submitting.
Learning issue-time and miss-revision metadata belongs to the application/store
wrapper, not to Study's understanding of correctness.

## Implemented and replaceable

The progression/revision/sticky-result rules are real. Production graders and
question sources implement the existing traits. Study's tests use a non-chess
multiple-choice grader through the exact same protocol; no chess type enters
its public surface. Errors distinguish source failure, invalid grading input,
stale revision, closed attempt and invalid grader transitions.

## Check

From the assembled development workspace:

```sh
cargo test -p tempo-study
```

The source contract lives in [src/lib.rs](src/lib.rs). The initial `=0.1.0`
Tempo dependency is resolved through the development workspace's explicit local
patch until the coordinated component versions are published. See the planning
repository for the cross-repository dependency rules and release procedure.
