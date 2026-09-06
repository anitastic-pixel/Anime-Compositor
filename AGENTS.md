# AGENTS.md - TotallyNotAfterEffects implementation rules

This repository is governed by the numbered planning documents in `Markdown/`. Before changing code, identify the requirement ID, task ID and authoritative contract.

## The reviewer cannot read code

The product owner has no programming background. Nobody on the human side will catch a bad implementation by reading a diff. Human review of source code is not part of this project's quality system and must never be assumed as a backstop.

What replaces it is evidence: independent fixtures with expected values written before the implementation, exported artifacts a person can look at, and honest reporting of what was and was not run. Document 12 defines that protocol in full. Every rule below exists to protect it.

## Never

- Modify an expected value in `Fixtures/` or document 25. Those are the specification. A change there is a specification change requiring explicit owner approval, proposed separately from any implementation work.
- Weaken, skip, delete, or mark-as-expected-failure a test in order to make a task pass.
- Generate a test's expected output by running the implementation under test.
- Report a task complete without attaching its verification artifact.
- Claim a benchmark, compatibility level, legal status or test result that was not actually measured.
- Add roadmap features while implementing an unrelated task.
- Change project schema without migration and schema fixtures.
- Change time, render or alpha math without updating independent fixtures and the governing specification.
- Introduce a dependency without recording purpose, version, license and distribution impact.
- Mutate the project model directly from UI code; use commands.
- Use widget references, memory addresses or display names as persistent identity.
- Silently substitute missing media, unsupported effects or unknown project semantics.
- Implement anything demoted to PARKED in document 23 without an explicit promotion decision.

## Always

- Attach a verification artifact to every completed task: fixture pass/fail with expected-versus-actual numbers, and where visual, exported PNGs or screenshots.
- State plainly which tests were run and which were not.
- Preserve stable IDs and immutable render/export snapshots.
- Treat media, project and expression input as untrusted.
- Use stable diagnostic IDs from document 28.
- Keep the render plan tile-based and the tile contract in document 21 intact.
- Preserve unknown serialized data rather than dropping it to simplify parsing.
- Update the relevant ADR in `docs/adr/` when a consequential decision changes.

## Pull requests

**One pull request at a time, based on `main`.** Do not open a second while the first is
unmerged, and do not base a branch on another branch. A stack of three was opened here once and
merged twenty seconds apart; GitHub had not yet re-pointed the third at `main`, so it merged into
the second one's branch and never reached the trunk. The work looked merged and was not.

If a stack cannot be avoided, merge the bottom one, then **confirm the next reports
`mergeable: MERGEABLE` against `main` before merging it** - not merely that its checks are green.
Merging in a rapid burst is what breaks this.

Merges into `main` here are squashed. A branch based on a branch therefore carries the *original*
commits of work that reached `main` *squashed*, and merging it adds every one of those lines a
second time. The repair is to rebuild the work on `main` and compare the two file lists and their
contents before opening the replacement, never to resolve such a conflict by hand.

Use `scripts/gh` rather than the GitHub CLI directly, and `scripts/push` rather than `git push`.
They supply the credentials this machine keeps in the git credential helper, and they refuse
deleting and history-rewriting operations, which stay the owner's to run.

## When a request conflicts with a contract

Do not silently pick a side. Name the conflict, name the affected documents, propose the smallest specification change, and stay blocked on that point until the owner decides.

## Task protocol

1. Read the task, its requirement and its dependent contracts.
2. State assumptions and affected files before broad changes.
3. Implement the smallest complete behavior.
4. Run the fixtures. Inspect failures rather than adjusting expectations.
5. Produce the verification artifact.
6. Report what passed, what failed, and what was not run.
