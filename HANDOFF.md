# Session handoff

Written 2026-09-04, at the end of the version 0.3 planning session. Read this first when opening Claude Code in this directory for the first time.

## What this project is

A cel exposure and finishing compositor for 2D animation, including anime. Windows only, offline, open source, built by one person. Currently in planning: **no code exists yet.**

## What you need to know before doing anything

**The owner has no programming background and cannot read code.** This is not a footnote. It determined the language, the interface technology, the renderer, the size of the first milestone and the entire quality process.

Human code review does not exist on this project and must never be assumed as a backstop. It has been replaced by verification against independent fixtures and artifacts a non-programmer can judge. **Read `Markdown/12_Development_Operating_Guide.md` before anything else** — it defines what "done" means here, and nothing else in the pack means what it appears to mean without it.

The single most important rule: **expected values in `Fixtures/` and document 25 are read-only to implementation work.** Changing one to make a build pass is the one failure this project has no other defense against. A fixture change is a specification proposal, submitted separately, approved by the owner before any code depends on it.

## What happened in this session

Version 0.2 was a genuinely good 32-document planning pack whose open decisions were all owner decisions that had never been asked. A structured design interview closed them.

Decisions closed: Windows 11 x64 only on a declared reference machine (Ryzen 9 9900X, 64 GB, RTX 4070 Ti Super). Open-source distribution, decided early specifically to remove license review from a project with no legal reviewer. Rust core with rayon, Tauri interface in HTML and CSS. CPU-only tile-based rendering with GPU deferred behind a measured trigger. The owner draws the reference shot, which closes the last rights-clearance dependency.

The first milestone was cut roughly in half. G1 split into G1-core (import, exposure, layers, transforms, undo, save and recovery, export, color, offline) and G1-rest (masks, effects, preview cache), the latter parked with written revisit triggers.

The build-versus-extend off-ramp in document 30 was closed **on preference, not on evidence** — the comparison against Natron, Fusion and OpenToonz will not be run, and the pack says so plainly rather than implying evidence it does not have.

Word review copies and the checksum manifest were dropped. The pack is now a git repository.

## Where things stand

Everything in this repository is specification. **No test has been run. No performance number has been measured.** Treat every number in the pack as a target or an estimate, never as a result.

Next action is B-01 in `Markdown/15_Initial_Backlog.md`: draw the reference shot per document 22, then run SP-01 save and reopen with an interrupted write, SP-03 scrub latency, SP-04 render determinism, SP-05 frame transport into WebView2, and SP-06 viewer color exactness.

SP-05 and SP-06 matter most, because they test the two real risks of the Tauri decision: whether frames reach the viewer fast enough, and whether the viewer displays them without altering color. Both are preview-side only and neither affects exported output.

## Open questions the owner has not answered

Weekly capacity, deliberately uncommitted. Consequence: document 13 has no dates and none may be invented.

The public product name. `TotallyNotAfterEffects` is a working title and a joke; it references a competitor trademark and is unsuitable for distribution. Deferred until there is something to name.

Expression runtime for G2. Not decided, and must not influence G1-core design.

## Things a fresh session is likely to get wrong

Do not propose adding masks, effects or a preview cache. They are parked deliberately, are fully specified, and have written triggers. Building them is explicitly forbidden in `AGENTS.md`.

Do not propose a GPU path. It is trigger-gated on a recorded stopwatch reading, not on the fact that a 4070 Ti Super is sitting idle.

Do not propose C++ or a native UI toolkit without reading ADR-003 and ADR-004 first. Both were considered and rejected for reasons specific to this project.

Do not treat fast code generation as a reason to widen scope. The bottleneck is owner verification time, and generating code does not reduce it. This is the most common way this project would fail.

Do not report a task complete without a verification artifact the owner can judge.

## Repository layout

`Markdown/` the 32 planning documents, the only source of truth. `docs/adr/` full architecture decision records. `Schemas/` the draft project schema. `Fixtures/` fixture data and expected values, read-only to implementation. `design/` interface design work, currently empty. `CONTEXT.md` project vocabulary. `AGENTS.md` and `CLAUDE.md` enforceable agent rules.

## Suggested first session

Either draw the reference shot and start B-01, or build a deliberately minimal Tauri and Rust spike that opens a window and displays one composited PNG. The second is worth doing before any design work, because five designed screens for an application that has never launched risks designing against assumptions the first working build contradicts.
