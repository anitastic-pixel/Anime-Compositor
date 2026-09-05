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

## Risks

K-01 / High / Scope expansion. Effect catalogs and conveniences displace core reliability. Trigger: anything from the parked list appearing in a build. Mitigation: the admission rule in document 04, the parked list in document 23, and the agent rule forbidding parked work.

K-02 / High / Pixel errors. Alpha and color mismatch damages edges. Trigger: halos, or preview and export disagreeing. Mitigation: independent fixtures, tagged buffers, render trace.

K-03 / High / Data loss. Partial writes or migrations damage projects. Trigger: failed recovery fixture. Mitigation: atomic replacement, autosave, T-07.

K-04 / High / Unverifiable implementation. The owner cannot detect a wrong implementation that passes its fixtures, or a fixture quietly weakened. Trigger: unexplained fixture changes, or reports lacking artifacts. Mitigation: document 12, the fixture read-only rule, mandatory artifacts, mandatory not-run reporting. This risk is inherent and permanently open.

K-05 / High / Capacity. The project is large and the owner is one person without a coding background. Trigger: milestone stalls, growing unfinished subsystems. Mitigation: the D-12 narrowing, one subsystem at a time, willingness to narrow again.

K-06 / Medium / Preview transport ceiling. WebView2 frame delivery may not sustain full-resolution playback, and browser color management may alter displayed pixels. Trigger: SP-05 or SP-06 failing. Mitigation: both are preview-side only and never affect export; the fallback is a native viewer surface inside the web shell.

K-07 / Medium / Unsafe input. Malformed media exhausts resources. Trigger: unbounded parser. Mitigation: size limits and cancellation.

K-08 / Medium / Dependency drift. WebView2 is a system component updated by Microsoft outside the control of this project. Trigger: a Windows update changing viewer behavior. Mitigation: viewer color and transport fixtures run against each release candidate.

## Assumptions and change log

A-01: solo development is permanent for planning purposes. A-02: artistic acceptance requires the owner using the tool on a real shot, and cannot be replaced by fixtures. A-03: owner verification is genuine and unhurried; the protocol in document 12 fails if artifacts go unread.

Version 0.3 supersedes 0.2. It closes D-01, D-03, D-05, D-06 and D-13 through D-16, and adds D-12. All implementation tests remain NOT RUN. Further changes are recorded in git history and in `docs/adr/`.

Related documents: 00, 04, 12, 13, 18, 23 and 30.
