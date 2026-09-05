# Development operating guide: verification without reading code

Version 0.3 | 2026-09-04 | Accepted for baseline

## Why this document was rewritten

Version 0.2 described code review discipline. That assumed a reviewer who reads source. The product owner has no programming background, so that reviewer does not exist and cannot be created by wishing.

This is not a fatal problem, but it is a real one, and pretending otherwise is how this project would fail quietly. An agent that cannot be checked will eventually take the path of least resistance: weakening a test, generating an expectation from its own output, or reporting success it did not measure. None of those are malice; all of them are invisible without a control.

The control this project uses is evidence. The owner does not review code. The owner reviews numbers and pictures. Everything below exists to make that a real audit rather than a ritual.

## The division of responsibility

The agent is responsible for source code, its correctness, and its honest reporting. No human will inspect it, and no human approval of a diff should ever be inferred.

The owner is responsible for the specification: what the software must do, what values are correct, which artifacts look right, and whether a claimed result is acceptable. The owner is the authority on fixtures, requirements and scope, and the only party who may change them.

The fixtures are the contract between the two. This is why their integrity is protected more strictly than anything else in this repository.

---

## The fixture integrity rule

Expected values in `Fixtures/` and document 25 are specification. They are written from independent reasoning about correct behavior, before or apart from the implementation that must satisfy them.

Implementation work treats them as read-only. An agent may not edit an expected value, loosen a tolerance, delete a case, or mark one as expected-to-fail as part of making a task pass. If the implementation disagrees with a fixture, the possibilities are that the implementation is wrong or that the fixture is wrong, and distinguishing those is a specification question for the owner, not an implementation decision.

Proposing a fixture change is legitimate and sometimes correct. It must be proposed on its own, with the reasoning for why the expected value is wrong, and it must be approved before any code depends on the new value. It must never arrive bundled inside a task that is otherwise reporting success.

This single rule is the main defense against a green board that means nothing.

## Verification artifacts

Every completed task produces something the owner can personally judge. A summary sentence is not an artifact.

For numeric work, the artifact is a table of fixture cases with expected value, actual value, tolerance and pass or fail. The owner can read that table without knowing what the code does.

For visual work, the artifact is exported PNGs or screenshots, alongside the fixture table where numeric expectations also apply. Where a result is wrong, the render trace dump of intermediate layer buffers accompanies it, so the failure can be located by looking rather than by reading.

For persistence work, the artifact includes the saved project file itself and the result of reopening it, because inspectable JSON is one of the few places the owner can verify behavior directly.

For performance work, the artifact records the machine, build configuration and measured numbers. Estimates are never reported as measurements.

## Honest reporting requirements

Every completion report states which tests were run, which passed, which failed, and which were not run and why. "Tests pass" without that breakdown is not an acceptable report.

An agent that cannot complete a task states that plainly. A partial implementation reported as complete is worse than an incomplete one reported honestly, because the first one costs the owner the ability to trust every other report.

When an agent notices that its own earlier work was wrong, it says so directly and fixes it. The reporting standard is what a careful colleague would say, not what makes the session look successful.

---

## Session structure

Work proceeds one task at a time from the backlog in document 15, in dependency order. A session begins by identifying the task, its requirement ID and its governing contracts, and ends with the verification artifact.

Do not run several backlog tasks in one session because they seem small. The point of the narrow dependency chain in document 15 is that rendering and data model errors surface before a large interface hides them.

Do not build user interface for a feature whose contract is not yet passing its fixtures. Working pixels first, then the panel that shows them.

## What the owner does each session

Read the verification artifact. Check the fixture table for cases that were skipped rather than passed. Look at the exported images and judge whether they are actually right, including the ones that pass numerically.

Ask what was not run. Ask whether anything in `Fixtures/` changed, and refuse the change if it arrived as part of an implementation task rather than as its own proposal.

Confirm that nothing from the parked list in document 23 was built.

## Scope discipline for agents

Implement the requirement, not the requirement plus what seems obviously useful next. The parked features in document 23 are parked deliberately, and an agent adding them is not being helpful; it is spending the owner's most limited resource on work that was explicitly deferred.

Do not introduce abstraction ahead of a second call site. Do not refactor systems unrelated to the current task. Do not add a dependency without recording purpose, version, license and distribution impact, because distribution is open source and license compatibility is a real constraint.

## Limits of this protocol

This protocol catches wrong results. It does not catch bad architecture, accumulating internal complexity or security weaknesses that never surface as a failing fixture. Those risks are real and are accepted knowingly, mitigated only by keeping the codebase small, the scope narrow and the module boundaries in document 06 intact.

If external engineering review ever becomes available, architecture and safety review is where it should be spent, not on line-by-line correctness that fixtures already cover.

Related documents: 11, 15, 23, 25 and 28. Governance rules in root `AGENTS.md` and `CLAUDE.md` are the enforceable form of this guide.
