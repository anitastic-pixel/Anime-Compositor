# Decision, assumption and risk register

Version 0.3 | 2026-09-04 | Accepted for baseline

## Decision procedure

Every decision record contains ID, question, recommendation, evidence basis, owner, status and date. Entries may be ACCEPTED FOR BASELINE, PROVISIONAL or OPEN. Version 0.3 closes the decisions that version 0.2 left open, on the basis of a structured design interview with the owner rather than measurement. Where a decision rests on preference rather than evidence, this document says so explicitly.

Andrew is the product decision owner, the artistic acceptance reviewer and the sole verifier. No technical or legal reviewer is engaged.

## Closed decisions

D-01 / Platform / ACCEPTED. Windows 11 x64 only for G0 through G1. Declared reference machine: Ryzen 9 9900X, 64 GB RAM, RTX 4070 Ti Super 16 GB. Cross-platform support is not claimed, planned or designed for. Platform-specific code stays behind narrow interfaces so that this remains reversible, not because a port is scheduled.

D-02 / Initial scope / ACCEPTED. Complete the 2D milestone before any 2.5D work. Version 0.3 narrows this further: see D-12.

D-03 / Distribution / ACCEPTED. Open source. This decision was taken early and deliberately in order to remove license review as an ongoing cost, since no legal reviewer is available. All dependency choices must remain compatible with it. Revenue is not a project goal.

D-04 / Capacity / PROVISIONAL. Solo development with Claude Code on a subscription, no API spend, owner as sole reviewer. Weekly hours are deliberately not committed. Consequence: document 13 remains gate-driven with no calendar dates, and no estimate in this pack may be converted into a completion date until this is closed.

D-05 / Reference shot / ACCEPTED. The owner draws it. Specification is in document 22: 1920x1080, 24 fps, 240 frames, one static background, three cel layers on 1s, 2s and 3s with a deliberate five-frame hold and a one-frame accent, mixed soft and hard and semi-transparent edges, one deliberately missing frame and one Japanese filename. Rights-cleared by construction.

D-06 / Stack / ACCEPTED, spike-confirmable. Rust core with rayon for tile parallelism, Tauri with an HTML and CSS interface. Rationale in ADR-003 and ADR-004. Selected against the constraint that the owner cannot read code, which favors a compiler that rejects broken programs and a toolchain with no configuration surface for the owner to repair.

D-07 / Color / ACCEPTED. Linear-light premultiplied float32 working space with explicit PNG interpretation, per document 21.

D-08 / Compatibility / ACCEPTED. Native features only. No AE project import, no binary plugin hosting, no expression translation. Not a research topic during G1.

D-09 / G2 geometry / ACCEPTED. Non-intersecting planes with explicit order, when G2 is reached.

D-10 / Expressions / DEFERRED. G2 concern. The likely answer within a Rust core is an embedded sandboxed scripting runtime or a small custom evaluator, but this is not decided and must not influence G1-core design.

D-11 / Name / DEFERRED. TotallyNotAfterEffects is a working title and repository name only. It is a joke, not a product name, and it references a competitor trademark, which makes it unsuitable for distribution. Choose and screen a public name before any public build.

---

## New decisions in version 0.3

D-12 / G1-core narrowing / ACCEPTED. The fifteen-Must G1 list from version 0.2 is split. G1-core is import, exposure timing, layers and transforms, undo, save and recovery, export, color and alpha handling, and offline operation. Masks and mattes, the effect stack, and the bounded preview cache are parked to G1-rest. Rationale: at the available capacity the version 0.2 list was not deliverable, and document 30 identifies exposure-and-layer-first finishing as the only structural gap against existing free tools. Masks and blur are not gaps.

D-13 / Verification model / ACCEPTED. The owner cannot read code. Human source review is removed from the quality system and replaced by fixture-and-artifact verification, specified in document 12. Fixture expected values are read-only to implementation work. This is the primary control of the project and its most important decision.

D-14 / Renderer / ACCEPTED. CPU-only for G1-core, tile-based and multithreaded. No GPU backend is built until a measured stopwatch result on a real shot justifies it. The tile contract in document 21 is designed so a later GPU dispatch is a port rather than a rewrite. Consequence: the dual-path comparison in SP-02 is removed; there is one path.

D-15 / Build versus extend / ACCEPTED ON PREFERENCE. The comparison task in document 30, against Natron, Fusion and OpenToonz, will not be run. The owner wants to build this tool, and the comparison would not change that. This is recorded as a preference-based decision, not an evidence-based one, and document 30 is amended accordingly rather than left implying evidence that does not exist.

D-16 / Pack format / ACCEPTED. Word review copies and the checksum manifest are removed. Markdown is the only source and the repository is under git, which supplies the change record this document previously described by hand.

## New decisions in version 0.4

D-17 / sRGB transfer function / PROVISIONAL. Document 21 requires conversion between sRGB and linear light in both directions but never states which transfer function. B-02 implements IEC 61966-2-1: linear segment `c/12.92` below `c <= 0.04045`, and `((c+0.055)/1.055)^2.4` above, with the matching inverse. Evidence basis: this is the transfer function the sRGB standard defines and the one every tool the owner would compare against uses. Status is PROVISIONAL rather than ACCEPTED because it is an agent's reading of an unstated requirement, not an owner decision. Consequence if changed: every expected value in `verification/B-02_fixture_table.md` moves, and document 25's fixtures move with it.

D-18 / 8-bit quantisation rule / PROVISIONAL. Document 21 says "final integer output conversion clamps only at the declared encoding step" but does not state the rounding rule at that step. B-02 clamps to 0..1 and rounds to nearest with ties away from zero, `floor(c * 255 + 0.5)`. Evidence basis: it is the rule the G0 spikes already used, and it makes all 256 8-bit codes survive a decode and re-encode round trip exactly, which is stronger than the "at most one code value" tolerance document 25 allows. Status PROVISIONAL for the same reason as D-17. Both should be folded into document 21 as specification text rather than left as implementation comments.

## Risks

K-01 / High / Scope expansion. Effect catalogs and conveniences displace core reliability. Trigger: anything from the parked list appearing in a build. Mitigation: the admission rule in document 04, the parked list in document 23, and the agent rule forbidding parked work.

K-02 / High / Pixel errors. Alpha and color mismatch damages edges. Trigger: halos, or preview and export disagreeing. Mitigation: independent fixtures, tagged buffers, render trace.

K-03 / High / Data loss. Partial writes or migrations damage projects. Trigger: failed recovery fixture. Mitigation: atomic replacement, autosave, T-07.

K-04 / High / Unverifiable implementation. The owner cannot detect a wrong implementation that passes its fixtures, or a fixture quietly weakened. Trigger: unexplained fixture changes, or reports lacking artifacts. Mitigation: document 12, the fixture read-only rule, mandatory artifacts, mandatory not-run reporting. This risk is inherent and permanently open.

K-05 / High / Capacity. The project is large and the owner is one person without a coding background. Trigger: milestone stalls, growing unfinished subsystems. Mitigation: the D-12 narrowing, one subsystem at a time, willingness to narrow again.

K-06 / Medium / Preview transport ceiling. WebView2 frame delivery may not sustain full-resolution playback, and browser color management may alter displayed pixels. Trigger: SP-05 or SP-06 failing. Mitigation: both are preview-side only and never affect export; the fallback is a native viewer surface inside the web shell.

K-07 / Medium / Unsafe input. Malformed media exhausts resources. Trigger: unbounded parser. Mitigation: size limits and cancellation.

K-08 / Medium / Dependency drift. WebView2 is a system component updated by Microsoft outside the control of this project. Trigger: a Windows update changing viewer behavior. Mitigation: viewer color and transport fixtures run against each release candidate.

D-19 / Four import diagnostic identifiers / PROVISIONAL. Document 28's catalog defines `MEDIA_SEQUENCE_GAP`, `MEDIA_UNSUPPORTED_FORMAT` and `MEDIA_DECODE_FAILED`, which cover most of B-03. Test T-01 also requires behaviour for mismatched dimensions within one sequence, and the reference shot forces two more cases, and document 28 names none of them. B-03 adds `MEDIA_SEQUENCE_DIMENSION_MISMATCH` (WARNING, the sequence is treated as the majority size and the odd drawings are still imported), `MEDIA_SEQUENCE_DUPLICATE_NUMBER` (ERROR, two selected files claim one drawing number, the first in sort order is kept), `MEDIA_SEQUENCE_UNNUMBERED` (ERROR, a selected file has no number in its name and is excluded) and `MEDIA_SEQUENCE_NAME_VARIANT` (INFO, a file carries a clear number under a name the inferred pattern does not generate, and is imported under its own name). Evidence basis: document 28 requires stable machine IDs and forbids library error strings as user-facing identifiers, so silence was not an option and reusing an unrelated identifier would have been a silent reinterpretation of the catalog. Status PROVISIONAL because the names and severities are an agent's proposal, not the owner's decision. Consequence if changed: the identifiers in `verification/B-03_import_table.md` change, and document 28's catalog gains four entries.

D-20 / Frame numbers come from the file list, not the pattern / PROVISIONAL. Document 19 line 17 requires a sequence asset to store a numeric pattern and a frame-number-to-file map "so missing numbers remain missing rather than being silently compacted", but does not say which of the two decides membership. B-03 makes the map authoritative and the pattern descriptive: a frame number is the last run of ASCII digits in a file's stem, and a file whose name the pattern does not generate is still imported under its own name. Evidence basis: the reference shot requires it. Drawing 13 of layer 2 is `layer2_桜_013.png`, which `layer2_%03d.png` does not generate; a pattern-driven importer reports a false gap and drops a drawing the owner can see in their own folder. Consequence if changed: layer 2 would report a gap it does not have.

D-21 / Three command diagnostic identifiers / PROVISIONAL. Document 26 requires that a command validate intent and that "a rejected command does not change document revision, dirty state, undo stack or caches", and document 28 forbids surfacing a library error string as a user-facing identifier, but document 28's catalog names no identifier for a command that fails validation. B-05 adds `COMMAND_TARGET_MISSING` (ERROR, the command named a composition, layer or keyframe that is not there), `COMMAND_INVALID_VALUE` (ERROR, the command carried a value the model cannot hold: the wrong value kind for the property, a non-finite number, or an index outside the layer stack) and `COMMAND_LAYER_LOCKED` (ERROR, the command would have changed a locked layer). B-05 also raises `MATTE_REFERENCE_MISSING` and `MATTE_CYCLE`, which document 28 does list; it raises them as ERROR from the command that would create the bad reference, where the catalog's WARNING wording describes one found while loading a project. Evidence basis: rejection has to be reportable to be verifiable, and the rows in `verification/B-05_model_table.md` that prove a rejected command changed nothing are keyed on these identifiers. Status PROVISIONAL because the names and severities are an agent's proposal, not the owner's decision. Consequence if changed: those rows change and document 28's catalog gains three entries.

## Assumptions and change log

A-01: solo development is permanent for planning purposes. A-02: artistic acceptance requires the owner using the tool on a real shot, and cannot be replaced by fixtures. A-03: owner verification is genuine and unhurried; the protocol in document 12 fails if artifacts go unread.

Version 0.3 supersedes 0.2. It closes D-01, D-03, D-05, D-06 and D-13 through D-16, and adds D-12. Further changes are recorded in git history and in `docs/adr/`.

Version 0.6 adds D-21, from B-05: document 28's catalog names no identifier for a command rejected at validation, and document 26 requires that rejection be reported without changing the document. The model half of T-03 is no longer NOT RUN; its results are in `verification/B-05_model_table.md`. The render half of T-03, the four FX-XF transform fixtures, remains NOT RUN and belongs to B-05a. B-04 raised no new decision: `verification/B-04_exposure_table.md` records T-02 passing under the rules documents 20 and 25 already state.

Version 0.5 adds D-19 and D-20, both from B-03: document 28's diagnostic catalog has no identifier for the cases T-01 requires, and document 19 does not say whether the pattern or the frame map decides which drawings exist. T-01 is no longer NOT RUN; its results are in `verification/B-03_import_table.md`.

Version 0.4 records the owner passing G0 on 2026-09-04 and adds D-17 and D-18, two gaps in document 21 that B-02 could not implement around. T-04 and T-09 are no longer NOT RUN; their results are in `verification/B-02_fixture_table.md`. Every other implementation test remains NOT RUN.

Related documents: 00, 04, 12, 13, 18, 23 and 30.
