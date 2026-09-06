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

Amended again later the same day, and this is the part to read carefully. That revisit trigger has fired. `verification/B-08_preview_latency.md` measured the production preview path - not a spike - and recorded 12.2 frames per second in draft and 10.0 at full resolution against a 24 fps target, with about three quarters of every frame spent reading and decoding cels. Decoding costs the same at both resolutions, because a drawing is decoded at its own size before anything scales it, so the draft default cannot reach that cost. The trigger firing does **not** unpark the cache and does not make this paragraph licence to build it: it is recorded as D-37 and it is the owner's decision. G1-core is unchanged and nothing in it is unmet - D-32 already says what playback does when it cannot keep up, and now there is a number saying how often that will be.

**G0 is passed. The owner opened the gate on 2026-09-04.** The instruction was "open the gate; proceed to B-02". Production implementation is under way.

This decision was the owner's, as it had to be. An agent must not open a gate on its own reading of the evidence, and that rule still stands for G1 and G2.

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
