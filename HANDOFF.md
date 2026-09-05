# Session handoff

Written 2026-09-04 at the end of the version 0.3 planning session, and updated the same day when B-01 closed. Read this first when opening Claude Code in this directory for the first time.

## What this project is

A cel exposure and finishing compositor for 2D animation, including anime. Windows only, offline, open source, built by one person. Planning is complete and B-01's feasibility spikes have been run and recorded. **No production code exists yet** - everything under `spikes/` is quarantined per document 06 and is discarded at integration.

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

**B-01 is complete.** All three of its exit conditions are discharged and recorded in
`spikes/B-01_G0_spike_report.md`, which is the artifact the G0 gate is decided on.

Numbers in `Markdown/` are still targets and estimates. Numbers in the B-01 report are
measurements, taken on the reference machine, and the two must not be confused. The report
states what was run, what passed, what failed and what was not run with reasons.

What the spikes established: atomic save survives an interrupted write (SP-01, 4/4); the
renderer is byte-identical across thread counts and repeat runs, on synthetic layers and on
the real shot (SP-04, SP-07); the webview does not alter the bytes it is given, in readback
and on the physical display (SP-06); and the real reference shot composites at 12.02 ms per
frame with the sRGB encode fused into the tile (SP-07).

The two numbers that should shape G1-core work:

- **Transport, not rendering, is the preview bottleneck.** Compositing the real shot costs
  12.02 ms per frame; moving that frame into the webview costs 39.54 ms. JSON IPC is
  eliminated outright at 250 ms per frame. This makes the document 27 cache and a
  draft-resolution preview load-bearing rather than optional.
- **There is no serial sRGB encode stage, and one must not be built.** Document 21 line 117
  already makes colour conversion tile-safe. Doing it inside the tile rather than in a pass
  afterwards is worth 41.41 ms per frame for byte-identical output. An early version of the
  B-01 report got this wrong and the correction is recorded in place.

**The reference shot exists**, at `Fixtures/reference_shot/`. Layer 1 is the owner's
painting; layers 2-4 are generated, a specification decision the owner made explicitly
rather than one that was assumed. Both deliberate defects are present and protected by a
self-check: layer 3 drawing 007 is absent, and one layer 2 file carries a Japanese filename.
20 of the 240 composition frames reference the absent drawing.

Next action is the G0 gate in `Markdown/00_Start_Here.md`, which is the owner's decision, not
an agent's.

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

Do not cite SP-07 as evidence that the compositing math is correct. It measures cost and determinism on real media. Its colour arithmetic is provisional, document 25's expected values were not consulted, and establishing correctness is B-02's job.

Do not reuse anything under `spikes/`. It is quarantined by document 06 and written to be discarded. The SP-03 compositor is a deliberate copy of SP-04's, not a shared module.

## Where production code now lives

`Cargo.toml`, `src/` and `tests/` at the repository root are the production crate, `anime_compositor`, laid out per document 29. The `spikes/` directory is deliberately excluded from that workspace so nothing under `src/` can depend on it.

`verification/` holds the artifacts the owner reads: one file per completed task, plus the scripts that derive expected values independently of the code under test.

## Suggested next session

B-03: PNG import and the sequence manifest, including numeric pattern detection, gap reporting and Unicode paths. Its fixtures are already sitting in the reference shot, unrepaired: layer 3 drawing 007 is absent and one layer 2 file carries a Japanese filename. The artifact it owes is a fixture table plus the diagnostic text a user would actually see for the missing frame.
