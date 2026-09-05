# Session handoff

Written 2026-09-04 at the end of the version 0.3 planning session, and rewritten in part on 2026-09-05 when B-08a merged. Read this first when opening Claude Code in this directory for the first time.

## What this project is

A cel exposure and finishing compositor for 2D animation, including anime. Windows only, offline, open source, built by one person. Planning is complete and B-01's feasibility spikes have been run and recorded. Production code now exists under `src/` and `tests/`; everything under `spikes/` is quarantined per document 06, is discarded at integration, and must not be reused.

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

`docs/DEPENDENCIES.md` is generated from `Cargo.lock` and is what ADR-005 asks for. `Cargo.lock` is now committed; it had been gitignored from the spike era, which contradicted ADR-005 outright.

## Suggested next session

**Nothing large, without the owner.** G1-core's remaining piece is the viewer: a window, a
transport, playback and a work area. Every one of those is a decision that has not been made -
which frame is shown at rest, what a scrub does while the mouse is down, and whether playback
drops frames or slows down when it cannot keep up. Do not start it. Ask.

Ten decisions are PROVISIONAL or OPEN and waiting on the owner: D-22 through D-30 in
`Markdown/14_Decisions_Risks.md`. None of them blocks anything today, because each was assumed
one way and the assumption is written down, but D-28 in particular changes a default the owner
will feel: an export whose range contains a frame with no drawing is currently **refused before
anything is written**, per document 07, and document 28 says the same situation should write
those frames transparent with a warning.

What is left that needs nobody: **nothing that has been named.** Joining T-07's two halves was
the last decision-free item on the list and it is done - `tests/t07e_roundtrip_export.rs`,
`verification/T-07e_roundtrip_table.md`, 23 checks. Everything else in G1-core either needs a
decision from the list above or is the viewer.

What the T-07 export half settled and left:

- A project saved, reopened from the file on disk and exported produces files byte-identical to
  the same export from the project in memory, and byte-identical to the frames T-08 committed.
  The artifacts are `verification/T-07e_project.json` and `verification/T-07e_reopened.json`,
  which are the same file and can be compared in any diff tool.
- Q-01 - "no known reproducible project corruption in the release candidate" - is left open on
  purpose. It is a claim about a release candidate, not a check, and there is no release
  candidate.
- `src/persist.rs` has now been broken on purpose eight more times, in ways that damage the
  picture without damaging the text: a switched-off layer written as on, an exposure span
  dropped, a drawing left out of an asset, scale read back as a percentage, layer order
  reversed, exposure sheets ignored on open, a layer's last frame lost, and drawing numbers
  shifted by one. All eight were caught.

What T-08 settled and left:

- `src/export.rs` writes a declared inclusive frame range as a PNG sequence. Both ends are
  included, so 0 to 239 is 240 files. Naming is a `%0Nd` pattern. Cancellation is read between
  frames, never inside one, so a stopped job leaves whole files. A write failure names the frame,
  the path and how many frames were finished. `verification/T-08_export_table.md` is 47 checks
  and `verification/T-08_frames/` is six exported frames of the reference shot.
- `src/png_out.rs` is the only place this build encodes a PNG. The trace and the export both go
  through it, so they cannot drift in colour type, depth or chunk encoding. `src/trace.rs` still
  owns the trace's own tags.
- **There is no video file and no encoder.** D-30 records why: a codec is a dependency and a
  licence that follows the output, which is the owner's decision, not an agent's. Nothing in
  `src/export.rs` assumes an image sequence beyond its own module.
- D-28 and D-29 are new and PROVISIONAL. D-28 is a genuine conflict between documents 07 and 28
  about what a missing drawing should do to an export; the code follows 07 and offers 28's
  behaviour as an explicit override that still warns. D-29 is how a composition that starts
  before frame zero names its files: `shot_-0012.png`, sign in front of the padded digits.
- Export writes straight alpha by default and never bakes in a display transform, per document 21
  line 31. Premultiplied output and sixteen bits are both available and both tested against
  hand-derived numbers.

What B-08a settled and left:

- `src/compose.rs` is document 20's evaluation order at one frame: a `Project`, a composition ID
  and a frame number in, the renderer's `FramePlan` out, and `render_frame` beside it for the
  whole thing. `verification/B-08a_frames/` holds four frames of the reference shot rendered
  from `verification/B-08a_project.json`, which is a real project file, not test scaffolding.
- `FrameLog` finally has a production caller, which is what B-04b built it for.
- The default tile size is now `compose::DEFAULT_TILE_SIZE`, 128 pixels, taken from the
  measurement in `verification/B-05a_scaling_table.md`. It is a tunable, not a contract: output
  is byte-identical at every size tested.
- **This was the headless half of B-08 only.** There is no viewer, no transport, no playback and
  no work area, and none of them may be started without the owner: which frame is shown, what a
  scrub does, and whether playback drops frames or slows down are all decisions nobody has made.
- Step 9 of document 20, the output or display transform, is deliberately not done here.
  `render_frame` returns a working-space buffer, because the viewer and an export want different
  destinations and doing it once in the middle would do it twice.
- Mattes still do not render. A layer carrying one is drawn as if it had none and earns a
  `PROJECT_FEATURE_UNSUPPORTED` line per D-24, so it is visible rather than silent.
- D-26 is PROVISIONAL and needs the owner: document 28 names no identifier for a render request
  naming a frame outside the composition's range, and `COMMAND_INVALID_VALUE` is reused.

What B-09 settled, which earlier drafts of this file listed as open:

- `src/inspect.rs` is deleted. The human-diffable dump and the save format are the same thing:
  `persist::to_json` writes both, so the B-05 artifacts still diff by eye and there is no second
  spelling of a project that could drift from the first. `verification/B-05_project_*.json` are
  now literally what would be on disk.
- `tests/b04_exposure.rs`'s twenty-line integer-array scanner is gone, replaced by
  `serde_json::from_str`. Net deletion.
- Document 09's startup sweep for orphaned `.tmp` siblings is required by no document in the
  pack - a grep for it across `Markdown/` finds nothing, and document 07 line 29 and ADR-008 ask
  only for the temp-sibling write pattern, which `persist::save` and `persist::autosave` both
  follow and both clean up after themselves on failure. The only case left is a process killed
  mid-write, and cleaning that up needs a hook at application startup, which does not exist until
  the shell does. It belongs to B-08, not here.

What B-09 left for later:

- Recovery is a listing, not a flow. `persist::recovery_candidates` finds the autosave slots
  beside a project and orders them newest first, and `persist::recovery_diagnostic` raises
  `PROJECT_RECOVERY_AVAILABLE`; nothing decides when to offer them or what "restore" does to the
  open document, because that is a user-interface decision and B-08 owns it.
- Autosave has no timer. `persist::autosave` writes one slot when called. What calls it, and how
  often, is B-08's.
- Relink takes the files the owner picked and never scans a directory. That is deliberate - a
  scan guesses - but it means the interface has to present a file picker rather than a
  "find missing media" button.
- Migration has nothing to migrate. `SCHEMA_VERSION` is 0 and there is no version 1, so
  `persist::load_str` refuses a newer file by name and there is no upgrade path to test yet. The
  first schema change is when that gets written.
- D-24 is PROVISIONAL and needs the owner: `PROJECT_FEATURE_UNSUPPORTED`, for a valid record of a
  parked feature that is not an effect. Masks are the only such record the schema can hold today.

What B-05b left for later:

- Trace shows four of document 21's seven layer render stages, because the renderer has four.
  Every manifest says which are missing and why. When masks, effects, mattes or the other blend
  modes arrive, they each add a `Stage` variant and a row to `missing_stages`.
- Trace re-renders the stack once per layer, which is O(n^2) in layers. That is deliberate:
  the stage images come from the same `render` the real frame does, so a trace cannot drift
  from what it claims to trace. If a composition ever has enough layers for that to hurt, the
  fix is a frame cache, not a second rendering path.
- D-23 is PROVISIONAL and needs the owner. ADR-012 says trace images are written in the working
  space; they are written display-encoded instead, because a linear-light PNG is unviewable and
  an unviewable diagnostic image defeats the ADR's own justification.

What B-05c left for later:

- Nothing assembles a `FramePlan` from a `Project`. **Closed by B-08a**, above.
- Blend modes are per-pixel, so they are tile-safe and need no margin. The first operation
  that does need one is the blur in R-05, which is parked.

Carried forward, still outstanding:

- D-22 is PROVISIONAL and needs the owner. Document 21's transform formula says `S(scale/100)`
  while the model and the renderer treat 1.0 as identity.
- No default tile size lives in `src/`. **Closed by B-08a**: `compose::DEFAULT_TILE_SIZE`.
- Cache invalidation domains. Document 26 requires every committed command to report which
  caches it dirties, and document 27 defines the domains. `Document::apply` reports none,
  because no cache exists. That is B-08b, PARKED.
- Colour4 and boolean property values, which document 19 lists and `Value` does not carry.
  They come due with effects, in B-07, PARKED.
- Installing the frame-level diagnostic rate limiter. **Closed by B-08a**: `plan_frame` takes
  a `FrameLog` and every per-layer diagnostic goes through it.
- D-25 is PROVISIONAL: the limit of three, and the choice to log a few in full and then
  summarise, are the loop's and not document 28's.
