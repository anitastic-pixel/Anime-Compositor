# Session handoff

Rewritten 2026-09-08, at the end of B-12c. Read this first when opening Claude Code in this
directory. It is orientation only: what this project is, what state it is in today, and the things
a fresh session gets wrong. What each unit of work settled is in `Markdown/15_Initial_Backlog.md`
entry by entry, and the evidence is one file per unit under `verification/`.

## What this project is

A cel exposure and finishing compositor for 2D animation, including anime. Windows only, offline,
open source, built by one person. The planning pack in `Markdown/` is the specification and the
source of truth; `docs/adr/` holds the architecture decision records.

## What you need to know before doing anything

**The owner has no programming background and cannot read code.** This is not a footnote. It
determined the language, the interface technology, the renderer, the size of the first milestone
and the entire quality process.

Human code review does not exist here and must never be assumed as a backstop. It has been
replaced by verification against independent fixtures and artifacts a non-programmer can judge.
**Read `Markdown/12_Development_Operating_Guide.md` before anything else** — it defines what "done"
means here, and nothing else in the pack means what it appears to mean without it.

The single most important rule: **expected values in `Fixtures/` and document 25 are read-only to
implementation work.** Changing one to make a build pass is the one failure this project has no
other defense against. A fixture change is a specification proposal, submitted separately, approved
by the owner before any code depends on it.

Second rule, the one that catches agents rather than code: **do not report a task complete without
a verification artifact the owner can judge.** If a change cannot be demonstrated that way, say so
rather than declaring it done.

## Where things stand

**G0 passed on 2026-09-04. G1-core is built, and B-12 — the acceptance run — is what is left.**

Everything G1-core names exists: the colour and compositing core, PNG import with gap, Unicode and
format diagnostics, the rational time model and exposure spans, the command model with undo and
redo, the tiled multithreaded renderer, trace mode, masks and mattes, effects, the bounded preview
cache, persistence with autosave and recovery, PNG sequence export, the offline package, and the
editing window in `app/`. Every W-01 and W-02 step except one is reachable in that window by
keyboard; the exception is **creating a composition, which has no control at all** — a composition
comes from a fixture script and the window edits one that already exists.

**B-12 is a person, not a test.** The owner completes W-01 and W-02 on the reference shot unaided
and writes down what was awkward. `verification/B-12_acceptance_run.md` is the sheet for that run,
written to be used in front of the window: where each control is, what right looks like, what wrong
looks like. Nothing an agent can run substitutes for it.

**Three decisions are waiting on the owner before or during that run**, and they are all one
measurement read three ways. `verification/B-12c_owner_brief.md` is the brief. In short: the
performance fixture document 08 declares has now been built and measured
(`verification/T-06_declared_fixture.md`), which closes **D-41**; that measurement fired the
reopening condition **D-40** set for itself, so the cache budget is open again; and the same file
shows the declared ten-layer shot rendering about nine times slower than a 24 fps clock allows,
which is not in the register at all and probably should be. Do not act on any of them. Opening,
closing or reopening a register entry is a note in `Markdown/14_Decisions_Risks.md`, and the
register is the owner's.

**Around twenty entries in document 14 are PROVISIONAL or OPEN.** None blocks work today, because
each was assumed one way and the assumption is written down.
`verification/B-12b_provisional_decisions.md` is the same courtesy done properly for the two
newest, D-45 and D-46, and is the model to follow when an agent has to assume again.

## Things a fresh session is likely to get wrong

Do not treat fast code generation as a reason to widen scope. The bottleneck is owner verification
time, and generating code does not reduce it. This is the most common way this project would fail.

Do not propose a GPU path. ADR-006 gates it on a measured result on a real shot, and
`verification/T-06_declared_fixture.md` is now exactly such a reading — which makes it the owner's
question to open, not an agent's to answer by building.

Do not propose C++ or a native UI toolkit without reading ADR-003 and ADR-004 first. Both were
considered and rejected for reasons specific to this project.

Do not reuse anything under `spikes/`. It is quarantined by document 06, excluded from the cargo
workspace, and written to be discarded. Do not cite SP-07 as evidence that the compositing math is
correct: it measures cost and determinism, and its colour arithmetic is provisional.

Do not start G1-rest or G2 work — camera and parenting, expressions, collect and package,
additional formats. They are scoped and scheduled and none of them is next.

Do not edit a `verification/*.md` by hand. Every one is written by a test or a script, CI runs
`git diff --exit-code -- verification/`, and a hand-edited artifact is caught on the next run.
The exceptions are the pages written *for* a person rather than by a test — the acceptance run
sheet, the owner brief, the photograph pages — and each says so in itself.

Do not run the hardening script while a full test run or a timing measurement is in flight. It
mutates source files on purpose, and the other run's artifacts come out wrong.

## Repository layout

`src/` and `tests/` are the `anime_compositor` crate; `app/` is `anime_compositor_app`, the Tauri
shell and the page. `Markdown/` is the specification, `docs/adr/` the decision records, `Schemas/`
the project schema, `Fixtures/` the fixture data and expected values, `verification/` the artifacts
the owner reads, `tools/` the few generators and checks that are not `cargo test`, `Licenses/` the
archived licence texts, `design/` interface design work, `spikes/` the quarantined G0 code.
`CONTEXT.md` is the vocabulary; `AGENTS.md` and `CLAUDE.md` are the enforceable agent rules.

`cargo test --workspace` is the build. Three things are not in it and are run by hand: the
`#[ignore]`d timing tests (`tests/t06_envelope.rs`, `tests/b12b_declared_fixture.rs`,
`tests/b10_full_shot.rs`), the photographs (`tools/capture_window.ps1`), and the cross-reference
audit (`python tools/audit_references.py`, which reports a named file that does not exist or a
score quoted differently from the artifact that states it).
