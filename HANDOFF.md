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

**B-08, the viewer.** The owner authorised it on 2026-09-05 and answered the two questions that
were blocking it, both now CLOSED in `Markdown/14_Decisions_Risks.md`: **D-32**, playback that
cannot keep up drops frames and holds real time, and must report how many it skipped rather than
hiding it; and **D-33**, the preview defaults to draft resolution with an always-visible
indicator of which resolution is showing. Everything else left in G1-core routes through B-08 -
B-11's remaining two thirds and B-12's acceptance run both depend on it.

B-08's first slice is done and is the headless middle of it: `src/preview.rs`, checked by
`tests/b08_preview.rs`, artifact `verification/B-08_preview_table.md`. It holds the resolution
choice, the scale from a frame plan to a preview extent, and the playback clock that answers
which frame belongs on screen at a given instant. Nothing in it sleeps or reads a clock - the
caller supplies the elapsed time - which is what makes D-32's dropped frames checkable by a
table rather than only by watching. The row that matters most to the exit condition is there
already: a full-resolution preview of frame 100 against a real export of the same frame, 0 of
8,294,400 samples differing.

What is left of B-08 is the part with a window in it: the Tauri shell, the transport, the
wall-clock loop wired to `Playback`, the resolution indicator on screen, and the screenshots.
**D-36** picks the transport, which SP-05 measured twice and deliberately did not choose between:
a custom URI scheme rather than raw IPC, PROVISIONAL, one function to reverse.

**Read `verification/B-08_preview_latency.md` before planning that work.** It measures the
production preview path rather than a spike, and the number is not the one SP-05 led anyone to
expect: 12.2 frames per second in draft and 10.0 at full, against a 24 fps target, with about
three quarters of every frame spent reading and decoding cels. Decoding costs the same whatever
the preview extent, because a drawing is decoded at its own size before anything scales it, so
D-33's draft default buys back rendering time and nothing else. Nothing is broken by this and no
requirement is unmet - D-32 already decided what playback does when it cannot keep up. But it
fired document 23's revisit trigger for the bounded preview cache, whose wording is "measured
preview latency, recorded with numbers", and **that is registered as D-37 and left for the
owner**. Do not build the cache. A fired trigger is a reason to ask, not a permission.

`verification/D-37_decode_cost.md` is the arithmetic that decision needs, and it was taken far
enough that the owner can answer without any further work: what one cel costs to decode,
measured per layer, beside a count of how often the shot repeats itself derived from the
exposure sheet by `verification/derive_d37_reuse.py`. The short version is that a cache of one
cel avoids nothing, four avoids 54%, and all 57 avoids 94% at a cost of 473 MB. Two things in
it are worth knowing before any performance work here: decoding costs what comes out rather
than what goes in, so no drawing in this shot is cheap; and the single most expensive cel is
the background, which never changes.

**Why the window was not started here.** It needs Tauri, which ADR-004 already accepts, so it is
not a decision - it is a dependency intake of roughly two hundred crates. That means regenerating
`docs/DEPENDENCIES.md`, archiving every new licence text under `Licenses/`, and handing the owner
a diff far larger than anything they have been asked to judge so far, on top of three pull
requests they have not merged yet. The right order is merges first, intake second. Nothing about
the window is blocked otherwise: D-32, D-33 and D-36 have already decided how it behaves.

Of the two smaller viewer questions, one is now answered and one is still open. **D-35** records
the frame at rest as the work area's first frame, PROVISIONAL and cheap to reverse. What a scrub
does while the mouse is held down is still unanswered and is not yet needed, because nothing
scrubs. Assume one way when it is, write the assumption down as a decision entry, and say so in
the artifact, which is what this project has done with every other open decision.

Eleven decisions are PROVISIONAL or OPEN and waiting on the owner: D-22 through D-30 and D-34 in
`Markdown/14_Decisions_Risks.md`. None of them blocks anything today, because each was assumed
one way and the assumption is written down, but D-28 in particular changes a default the owner
will feel: an export whose range contains a frame with no drawing is currently **refused before
anything is written**, per document 07, and document 28 says the same situation should write
those frames transparent with a warning.

What is left that needs nobody: two named items were found and both are done, described first
below - B-10's declared artifact was only six frames when document 15 asks for the exported
240-frame sequence and a byte comparison of two exports, and B-11's dependency record was a
stale eleven-crate table. Before those, the owner asked for more
engine hardening after the day's queue emptied, and that became H-01 and then H-02, described
below. Joining
T-07's two halves was the last decision-free item on the backlog itself and it is done - `tests/t07e_roundtrip_export.rs`,
`verification/T-07e_roundtrip_table.md`, 23 checks. Everything else in G1-core either needs a
decision from the list above or is the viewer.

What B-10's full-shot export settled and left:

- Document 15 names B-10's artifact as the exported 240-frame sequence plus a byte comparison of
  two consecutive exports proving determinism. Six frames existed. `tests/b10_full_shot.rs` now
  exports the whole shot twice and compares all 240 pairs; it is `#[ignore]`d and run with
  `cargo test --release --test b10_full_shot -- --ignored`, taking about four minutes.
- **The frames are not committed and cannot be.** 240 frames at 1920x1080 is roughly 480 MB and
  `.git` is already 438 MB. They are written to `target/b10_full/pass1`, which is gitignored and
  left in place after the run so the owner can open them. What is committed is
  `verification/B-10_contact_sheet.png`, one 2560x1350 grid of all 240 frames, which is also the
  more useful artifact: nobody flips through 240 separate files.
- The determinism row has a negative control. Every frame is also compared against the *next*
  frame's file, and none of the 239 neighbouring pairs may match - layer 2 changes drawing on
  every frame of this shot, so a comparison that passed there would be comparing a file with
  itself.
- The export uses `MissingSource::RenderTransparent`, D-28's recorded override, because the
  default refuses this job outright: twenty of the 240 frames ask layer 3 for the drawing the
  fixture deliberately does not contain. That default is T-08's row and is unchanged.
- `fidelity_incomplete` stays **false** across the whole shot, which was not what this session
  expected. The flag is raised only for a parked feature bypassed on a drawn layer
  (`ProjectFeatureUnsupported`, today only masks), and a missing drawing is not that - it is a
  media gap, reported as one. The code was right and the expectation was wrong.
- Exactly four `MEDIA_SEQUENCE_GAP` diagnostics are emitted for the twenty affected frames: three
  in full and one summary, which is D-25's rate limit doing its job at full shot length for the
  first time.

What B-11's dependency and licence record settled and left:

- There is now one dependency record, `docs/DEPENDENCIES.md`, generated from `cargo metadata` and
  the committed `Cargo.lock`. The file that stood there listed eleven crates and predated `rayon`
  and `serde_json`; the graph is twenty-eight. `tests/b11_dependency_record.rs` compares the two
  in both directions on every run, so a crate added without an entry fails a named row.
- `Licenses/` holds every crate's licence and notice files as shipped inside the crate, copied
  from the local registry. All 28 shipped at least one. That is document 10's "archive the exact
  license" done rather than promised.
- **It reaches no legal conclusion, deliberately.** Document 10 reserves those for a reviewer and
  there has not been one, so the reviewer and date fields are blank rather than invented. Three
  entries are flagged for a reviewer instead of decided: `unicode-ident`'s Unicode-3.0 term is an
  `AND` and not an `OR`; `zlib-rs` offers no alternative to the Zlib licence; and `memchr`'s
  public-domain option is avoidable by taking its MIT half.
- D-31 was raised here and the owner closed it the same day: this project is **MIT OR
  Apache-2.0**. Whether to be open source was never open - D-03 and ADR-010 settled that - and
  what was missing was that the repository did not say so. `Cargo.toml` now declares the licence,
  `LICENSE-MIT` and `LICENSE-APACHE` are in the root, and two rows of the B-11 check hold both.
  One thing is left, as D-34: the copyright line reads `Copyright (c) 2026 anitastic-pixel`,
  the GitHub identity that owns the repository, because a legal name is not an agent's to guess.

What H-01, the whole-picture check, settled and left:

- Four frames of the reference shot are composited twice - once by the real tiled, threaded
  renderer, once by a naive compositor written inside `tests/h01_whole_picture.rs` straight from
  document 21 and D-17, sharing no code with it - and all 2,073,600 pixels of each frame must
  agree exactly. They do. The artifacts are `verification/H-01_whole_picture_table.md` and the
  two images `H-01_renderer_frame.png` and `H-01_independent_frame.png`, which the owner can
  flip between.
- This closes the gap the hardening report had been naming for itself: until now every table
  read named pixels and named values, so a one-pixel shift, a slight dim or a channel swap
  would have passed all of them. Six such faults were made on purpose and all six were caught.
- What it is not: independent verification in document 12's sense. Both compositors were
  written from the same document by the same agent, so a misreading of document 21 would appear
  in both. It also covers four frames of one composition whose layers all sit at the identity
  transform; moved, turned and scaled layers stay with B-05a's table.

What H-02, the same check with the layers moved, settled and left:

- The reference shot is composited twice again with layer 1 shrunk to half, layer 2 moved 320
  pixels right and 180 up, layer 3 doubled about the centre of the frame at half opacity and
  layer 4 shrunk to half. Every pixel of three frames agrees with the independent compositor to
  within 1e-6, and the largest disagreement seen was 2.13e-7 - about three f32 rounding steps.
  The artifacts are `verification/H-02_transformed_table.md` and the two frame images beside it.
- Why there is a tolerance here and not in H-01: resampling adds four weighted neighbours per
  pixel, and demanding identical bits would demand both compositors add them in the same order.
  The argument is written out in the artifact so it can be judged rather than trusted.
- Rotation is deliberately absent. `cos(90 degrees)` is not zero in floating point, so a rotated
  layer's samples miss the pixel centres by a hair and two correct implementations disagree in
  the last bits. Rotation is B-05a's, by named pixels.
- Two of its six deliberate faults survived the first draft, and both were fixture weaknesses
  worth remembering: with every anchor at the origin the transform chain can be composed in the
  wrong order unnoticed, and with every sampled layer transparent at its border, clamping to the
  edge pixel is indistinguishable from transparent black.

What H-04, the same check carried into the exported file, settled and left:

- H-01 to H-03 all stop in the working space. Every existing test of the step after it - the
  output encode - calls the build's own `encode` on both sides (`tests/t08_export.rs:668`,
  `tests/b02_color_alpha.rs`), so channel order, row order, stride and sixteen-bit byte order were
  unverifiable in principle, not merely unverified. H-04 exports through the real
  `export_sequence`, decodes with the `png` crate, and compares against an encoder written in the
  test from document 21 lines 7 and 31. `verification/H-04_exported_file_table.md`, 20 of 20.
- Four of its nine deliberate faults survived the first draft. **Three were H-03's lesson, one
  pass later and one unit downstream**: the opaque background makes every composited pixel solid,
  and both the unpremultiply and keeping alpha out of the colour curve are identities at alpha 1.
  The same answer worked - export frame 106 with the background layer removed. Frame 106 and not
  H-03's 110 because at 110 layer 3 asks for the absent drawing 7 and the export refuses.
  **If you are writing a whole-picture fixture, read the fault list in the mutation report first;
  this one was named, written up, and then walked into again immediately.**
- The fourth survivor is a new lesson: **a tolerance that is right for one fault is a hiding place
  for another.** One code value is the correct rounding allowance and also the exact size of a
  truncation bug. What separates them is the count: honest disagreement touched 1 sample of 8.3
  million, truncation touched 1,448,945. The count is now a bounded check, not a printed number.

What H-03, the same check with the blend modes on, settled and left:

- Every layer in H-01 and H-02 is normal, which the renderer routes past the unpremultiply
  entirely, so the arithmetic multiply, screen and add share had never touched an assembled
  frame. H-03 composites frames 0, 14 and 100 with layer 2 on multiply, layer 3 on screen at half
  opacity and layer 4 on add, against a third compositor written from document 21 lines 65-77.
  Transforms stay at the identity so a disagreement cannot be blamed on resampling. Largest
  disagreement seen: 2.81e-7, against a 1e-6 bound. `verification/H-03_blended_table.md`.
- Two of its nine deliberate faults survived the first draft and both are the same lesson, which
  is new to this project: **a picture with an opaque background cannot check anything about
  transparency.** Layer 1 fills the frame, so every blend lands on something solid, where the
  `As*Ad` weight is 1 and the difference between `As + Ad - As*Ad` and `min(1, As+Ad)` is nothing.
  Two more pictures were added: one layer over nothing, and frame 110 with the background removed.
- Frame 110 and not 100 because the count of half-transparent pixels landing on half-transparent
  pixels is 22,502 there and **zero** at frame 100. That count is a row of its own now, so the
  check cannot go hollow without saying so. It was found by measuring, not by assuming.
- The row written to catch the missing weight originally used multiply and could not fail:
  multiply's blend against an empty background is zero, so the mis-weighted term is zero either
  way. Screen is the mode that shows it.

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
  no work area. The viewer half is authorised as of 2026-09-05; D-32 and D-33 answer how playback
  behaves when it falls behind and what resolution the preview starts at, and the two remaining
  questions - which frame is shown at rest, what a scrub does while the mouse is held down - are
  to be assumed and written down rather than asked.
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
