# Session handoff

Rewritten 2026-09-08, at the end of B-12c; its state brought up to date on 2026-09-27, at G4. Read this first when opening Claude Code in this
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

**G0 to G3 are passed; G4 is current (D-167, 2026-09-27).** The editor does what the planning pack
set out to do and more: exposure timing, layers, masks, shape layers, parenting, the camera,
expressions, precompositions, adjustment layers, the graph editor, EXR, WAV, packaging, GIF and MP4
export, over ninety effects, most of them also drawn by the graphics card, and an After
Effects-style window around it. Every decision the owner had not yet answered was accepted on
2026-09-27, and every playtest then waiting was accepted without a separate run.

**G4 is the smaller gaps first, B-108 to B-114, then the redesign, W-31 onward.** Both are
described at the end of `Markdown/15_Initial_Backlog.md`. The tool is for the owner's own use, so
release work (installer, signing, licence texts, a clean-machine test) waits, and so does the name
(D-11). `Markdown/00_Start_Here.md` is the running record of what was built when and what the owner
said about it; read its last paragraphs for the latest state.

## Things a fresh session is likely to get wrong

Do not treat fast code generation as a reason to widen scope. The bottleneck is owner verification
time, and generating code does not reduce it. This is the most common way this project would fail.

Do not propose C++ or a native UI toolkit without reading ADR-003 and ADR-004 first. Both were
considered and rejected for reasons specific to this project.

Do not reuse anything under `spikes/`. It is quarantined by document 06, excluded from the cargo
workspace, and written to be discarded. Do not cite SP-07 as evidence that the compositing math is
correct: it measures cost and determinism, and its colour arithmetic is provisional.

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

`cargo test --workspace` is the build. Two things are not in it and are run by hand: the
`#[ignore]`d timing tests (`tests/t06_envelope.rs`, `tests/b12b_declared_fixture.rs`,
`tests/b10_full_shot.rs`) and the photographs (`tools/capture_window.ps1`). The cross-reference
audit (`python tools/audit_references.py`, which reports a named file that does not exist or a
score quoted differently from the artifact that states it) was the third until B-12d, and is now
a CI gate.
