# Start here

Version 0.3 | 2026-09-04 | Accepted baseline

## What this is

The planning baseline for a cel exposure and finishing compositor for 2D animation, built and owned by one person, distributed as open source, running offline on Windows.

This is a specification, not evidence that anything has been implemented. No test in this pack has been run. No performance number in it has been measured.

## What changed in version 0.3

Version 0.2 was a well-specified plan resting on two things that turned out to be false: that the open decisions would be resolved by measurement, and that a human would review the code. Version 0.3 corrects both.

Every gating decision is now closed. Windows 11 x64 only, on a declared reference machine. Open source. Rust core with rayon and a Tauri interface in HTML and CSS. CPU-only tile-based rendering, with GPU deferred behind a stopwatch. The owner draws the reference shot. See document 14.

The first milestone was cut roughly in half. G1 splits into G1-core, which is import, exposure, layers, transforms, undo, save and recovery, export, color and offline operation, and G1-rest, which is masks, effects and the preview cache. The parked work is fully specified and carries explicit revisit triggers. See D-12 and document 23.

Most importantly: the owner has no programming background and cannot read the code. Human source review is therefore removed from the quality system and replaced by verification against independent fixtures and artifacts a non-programmer can judge. Document 12 specifies this and is the most load-bearing document in the pack. See D-13.

## How to read this

Read document 12 first. Nothing else in this pack means what it appears to mean without it, because it defines what "done" is and how anyone knows a claim is true.

Then product intent: 01 charter, 02 workflows, 03 requirements, 04 scope, 23 priority matrix.

Then contracts: 18 ADRs, 19 data model, 20 time model, 21 rendering math, 24 commands, 26 undo, 28 diagnostics.

Then delivery: 07 project format, 11 verification, 22 reference shot, 25 fixture catalog, 29 build, 10 legal, 13 delivery.

Research context: 16, 30 and 31.

Before any code, read root `AGENTS.md`, `CLAUDE.md` and `CONTEXT.md`.

## Status vocabulary

ACCEPTED means the decision is the working contract until changed through document 14. PROVISIONAL means a recommended default still awaiting evidence. DEFERRED means deliberately postponed with a trigger, not rejected. PARKED means specified and deliberately not being built. OPEN means unresolved.

Performance thresholds remain unmeasured. No benchmark, compatibility level or legal clearance is implied anywhere in this pack.

---

## Current gate

**B-01 is recorded.** The reference shot is drawn and committed at `Fixtures/reference_shot/`, SP-01, SP-03, SP-04, SP-05 and SP-06 have been run, SP-07 adds the real-shot measurement ADR-006's exit condition asks for, and ADR-003, ADR-004 and ADR-006 are confirmed with none reopened. The artifact is `spikes/B-01_G0_spike_report.md`.

The two risks that decided the Tauri interface are settled. SP-06 found the webview alters nothing, in readback and on the physical display. SP-05 found frames do reach the viewer fast enough, but with almost no margin: 39.54 ms per frame at full resolution against a 24 fps target, which is 3.3 times the cost of compositing the frame in the first place. Neither failed, so ADR-004's native-surface fallback is not triggered, but the document 27 cache and a draft-resolution preview are load-bearing rather than optional.

Amended 2026-09-05. Those two halves went different ways, and the difference is worth stating so this paragraph is not read as licence to build the cache. The draft-resolution preview is now the viewer's default and is decided: D-33. The document 27 cache is not in G1-core at all - R-06a says "no bounded cache; render on demand and accept the cost", and B-08b is PARKED under D-12 with a revisit trigger in document 23. What absorbs the missing margin in G1-core is therefore the draft default plus D-32, which lets playback drop frames rather than stretch the clock, and not a cache.

Amended again later the same day, and this is the part to read carefully. That revisit trigger has fired. `verification/B-08_preview_latency.md` measured the production preview path - not a spike - and recorded 12.2 frames per second in draft and 10.0 at full resolution against a 24 fps target, with about three quarters of every frame spent reading and decoding cels. Decoding costs the same at both resolutions, because a drawing is decoded at its own size before anything scales it, so the draft default cannot reach that cost. The trigger firing did **not** unpark the cache by itself, and this paragraph was never licence to build it: it was recorded as D-37 and left to the owner, who answered it the same day and unparked it. B-08b is being built, bounded by D-37 and ADR-015 to a cache of decoded cels with a memory ceiling, in the preview path only. G1-core is unchanged and nothing in it is unmet - D-32 already says what playback does when it cannot keep up, and now there is a number saying how often that will be, and something being done about the number. Masks and effects are still parked; their triggers have not fired.

**G0 is passed. The owner opened the gate on 2026-09-04.** The instruction was "open the gate; proceed to B-02". Production implementation is under way.

This decision was the owner's, as it had to be. An agent must not open a gate on its own reading of the evidence, and that rule still stands for G1 and G2.

**G1 is passed. The owner opened the gate on 2026-09-15.** The answer was "Pass now, gaps noted": the three gaps are written down as accepted and open in D-54. The owner accepted the measured memory headroom the same day (D-55), which is the other half of what document 04 asks before G2. B-13 is done by the owner's word on 2026-09-16. Current work is B-14 in `Markdown/15_Initial_Backlog.md`, the expressions of R-13. B-14a is done: the owner accepted D-59, the answer to D-10, on 2026-09-16. B-14b, the evaluator, the file and the export refusal, is built on 2026-09-16 and T-12 passes. The owner accepted B-14b ("works") the same day. B-14c, expressions in the window, is done by the owner's word the same day, and with it B-14.

**G2 is passed, and G3 is entered for B-15 and B-16. The owner decided both on 2026-09-16** (D-60). B-15, collect and package (R-14), is done. B-15a is done: the owner accepted D-61, the packaging contract, on 2026-09-16. B-15b, collecting, the manifest, the hash and checking, is built on 2026-09-16 and T-13 passes: `verification/B-15b_package_table.md`, 30 of 30. B-15c, the window, is built on 2026-09-16 and done on 2026-09-17 by the owner's word ("B-15c playtest works"), after walking `verification/B-15c_package_playtest.md`. Current work is B-16, other formats, EXR first. B-16a is done: the owner accepted D-62, the EXR contract, on 2026-09-17. B-16b, the EXR reader and writer, is built on 2026-09-17 and T-14 passes for EXR: `verification/B-16b_exr_table.md`, 71 of 71, and OpenEXR's own check of the exported files, `verification/B-16b_exr_export_check.md`, 60 of 60. B-16c, EXR in the window, is built on 2026-09-17: `verification/B-16c_panel_table.md`, 18 of 18, and done on 2026-09-18 by the owner's word ("pass the b-16c, it worked"), after walking `verification/B-16c_exr_playtest.md`. B-16 is done for EXR. Audio (WAV) is deferred by the owner on 2026-09-17 (D-63). **Next, by the owner's decision on 2026-09-17** (D-64, D-65): foundations first, so the exposure list's overhaul and anime-specific processing wait, and G3 continues with B-17 adjustment layers, then B-18 precompositions, then B-19 the graph editor's gaps. B-17a is done: the owner accepted D-66, the adjustment layer, on 2026-09-17. B-17b, the adjustment layer in the core, is built on 2026-09-17: `verification/B-17b_adjust_table.md`, 44 of 44. B-17c, the adjustment layer in the window, is built on 2026-09-17 and done on 2026-09-18 by the owner's word ("B-17c playtest works"), after walking `verification/B-17c_adjust_playtest.md`. B-17 is done. B-18a, the precomposition specification, is done: the owner accepted D-67 on 2026-09-18 ("proceed"). B-18b, the precomposition in the core, is built on 2026-09-18: `verification/B-18b_precomp_table.md`, 70 of 70, FX-PRE-001 to 015 within 1e-6. B-18c, the window, is built on 2026-09-18: `verification/B-18c_panel_table.md`, 14 of 14, and done the same day by the owner's word ("B-18c playtest works"), after walking `verification/B-18c_precomp_playtest.md`. B-18 is done. Current work is B-19, the graph editor's gaps (D-65). The owner named its scope on 2026-09-18 ("all 10 please; along with adding animation/keyframing stopwatches to effects as well"), and document 15 splits it into B-19a to B-19g. B-19a, the six gaps that change no file, is built on 2026-09-18: `verification/B-19a_panel_table.md`, 11 of 11, and the playtest sheet `verification/B-19a_graph_playtest.md` was played by the owner on 2026-09-19 and works. B-19b, the specification for keyframed effect settings, is written on 2026-09-18: D-68 in document 14 is PROPOSED with its fixtures, FX-FXK-001 to 009, and the owner accepted it the same day ("proceed"). B-19c, keyframed effect settings in the core, is built the same day: `verification/B-19c_fxkey_table.md`, 47 of 47. B-19d, the window half, is built the same day and awaits the owner's playtest: `verification/B-19d_panel_table.md`, 21 of 21, and `verification/B-19d_fxkey_playtest.md`, played by the owner on 2026-09-19: it works. B-19f, the core for D-69, is built the same day: `verification/B-19f_keykind_table.md`, 36 of 37, and the one is a fault found in the fixture FX-KIND-005 that **awaits the owner's decision** (document 15, B-19f). B-19g, the window for D-69, is built the same day: `verification/B-19g_panel_table.md`, 24 of 24, and `verification/B-19g_keykind_playtest.md` was played by the owner on 2026-09-19 and works. That is all ten graph items and the effect stopwatches built; **B-19 is done on 2026-09-19:** all three playtests work, and the owner had FX-KIND-005 corrected, so `verification/B-19f_keykind_table.md` is 36 of 36. B-19e, the specification for separate X and Y and for auto, continuous and roving keys, is written the same day as D-69, with its fixtures, and accepted with it.

Current task is B-02 in `Markdown/15_Initial_Backlog.md`: tagged image buffers, the linear-light premultiplied float32 working space and normal-over compositing on the CPU. Its artifact is `verification/B-02_fixture_table.md`.

## Document map

00 Start here. 01 Charter. 02 Workflows. 03 Requirements. 04 Scope and roadmap. 05 Interface specification. 06 Architecture. 07 Project and media format. 08 Rendering, color and performance overview. 09 Effects, parked with R-05. 10 Legal and licensing. 11 Verification plan. 12 Development operating guide, the verification protocol. 13 Delivery and capacity. 14 Decision and risk register. 15 Backlog. 16 Production tool research. 17 Evidence register. 18 ADR index. 19 Core data model. 20 Time and animation model. 21 Rendering math and the tile contract. 22 Reference shot. 23 Feature priority matrix. 24 Command and interaction map. 25 Test fixture catalog. 26 Undo and command model. 27 Cache model, parked with R-06b. 28 Error and diagnostics catalog. 29 Build and reproducibility. 30 Competitive analysis. 31 Anime workflow research.

Root files: `AGENTS.md` enforceable agent rules, `CLAUDE.md` working instructions, `CONTEXT.md` vocabulary, `HANDOFF.md` session context, `docs/adr/` full decision records, `Schemas/` project schema, `Fixtures/` fixture data and expected values, `design/` interface design work.

## Authority

The charter governs intent. Requirements govern observable behavior. ADRs and documents 19 through 21 govern implementation contracts. Document 12 governs what counts as evidence. Document 14 records changes.

A conflict between documents is a specification defect, resolved explicitly rather than by picking whichever prose is newer or easier to implement.

Markdown is the only source. Word review copies and the checksum manifest were removed in version 0.3; git history is the change record.

## Version 0.3 acceptance

The pack now contains closed decisions, a deliverable first milestone and a verification protocol suited to its actual reviewer. Remaining open items are weekly capacity, deliberately uncommitted, and the public product name, deliberately deferred.
