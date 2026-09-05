# Anime compositing workflow research agenda

Version 0.2 | 2026-09-04 | Proposed baseline

## Purpose and evidence boundary

This document turns anime-production references into testable product hypotheses. It does not claim one universal Japanese studio workflow. Vendor/studio sources demonstrate that particular tools/capabilities exist; actual priority for this application must come from representative shot work and artist validation.

## Confirmed reference signals

OLM publishes OpenTools including cel-oriented blur, smoothing, color-key, highlight/glow and distance-gradation tools, and describes some of these tools as developed/used in its production context [S-01]. This is direct evidence that edge treatment, cel blur, selective color operations and stylized highlight tools are meaningful enough to exist in an anime production pipeline; it is not evidence that every studio uses them.

PSOFT currently documents multiple AE-oriented products, including anti-aliasing, CelFX and CelMX, and its anti-aliasing manual documents direct use as an After Effects effect plus render-engine behavior [S-07, S-17]. These are capability references only; implementation/source reuse requires separate licensing review.

OpenToonz documents Xsheet/Timeline exposure of animation levels and an effects workflow [S-09, S-16]. This supports treating drawing exposure/holds as a first-class timeline concept rather than forcing every drawing to be represented as an ordinary video clip.

CELSYS describes RETAS STUDIO as commercial animation-production software and states broad adoption among Japanese animation companies [S-15]. This is a vendor claim and should be treated as biased evidence of historical workflow relevance, not an independently verified current adoption percentage.

## Workflow map to investigate

### 1. Cel/image sequence ingest

Needs: numeric sequence detection, drawing-number preservation, mixed holds, gaps, revised drawings and transparent edges. G1 R-01/R-02 already covers the minimum. Research question: whether artists need Xsheet-style vertical exposure entry, traditional timesheet import or both.

### 2. Revision and retake handling

Needs: replace one drawing without retiming the shot, detect missing/revised frames, preserve layer/effect setup, relink moved folders and compare revisions. W-02 should be instrumented because revision handling may be a stronger differentiator than adding more effects.

### 3. Line/edge treatment

Candidate operations: line smoothing/anti-aliasing, line thinning/thickening (morphology), line recolor, edge expansion and alpha cleanup. Evaluation must include thin diagonals, colored lines, transparent boundaries and temporal stability across changing drawings. Avoid a generic beauty filter that softens intended line character.

### 4. Cel-oriented blur and camera treatment

Candidate operations: directional blur, path/motion-like cel blur, controlled softness, camera shake, multiplane parallax and simple depth treatment. Promote only after G1 and use actual held-frame sequences to detect flicker/edge contamination.

### 5. Lighting and stylized finishing

Candidate operations: thresholded highlight extraction, glow/kira effects, light wrap, exposure flicker, gradients from alpha boundaries and color-selective treatment. These are later conveniences, not core compositor correctness.

### 6. Shot organization and handoff

Investigate cut/shot naming, frame numbering, work-area/render ranges, batch output, notes/markers, collect/package, reference audio, EXR/WAV and layered-artwork handoff. Do not build production-management infrastructure until repeated local workflow evidence exists.

## Research backlog

| Research ID | Question | Prototype/fixture | Promotion criterion |
|---|---|---|---|
| AR-01 | Is Xsheet-style exposure editing faster/clearer than layer-only timing? | RSH-01 mixed holds | fewer timing errors/repetitions in observed task |
| AR-02 | Which line-smoothing behavior preserves cel character? | thin/diagonal line sequence | no obvious blur/halo; stable across frames |
| AR-03 | Is line recolor used often enough for native support? | color-trace style fixtures | repeated need across representative shots |
| AR-04 | Which morphology/edge expansion cases are needed for mattes? | transparent line/paint edges | solves observed fringe/matte problem |
| AR-05 | Which blur variants matter beyond Gaussian? | directional/cel movement shot | clear workflow gain over generic blur |
| AR-06 | Are highlight/glow helpers worth dedicated effects? | bright highlight sequence | repeated setup reduced without artifacts |
| AR-07 | What revision metadata matters? | replacement drawing workflow | fewer accidental stale frames |
| AR-08 | Is traditional timesheet import required? | anonymized/original timesheet samples | repeated real handoff need + parseable conventions |
| AR-09 | Which exchange formats are actually required? | EXR/WAV/PSD/CSP samples | observed handoff need + legal/dependency path |
| AR-10 | Do artists need batch shot rendering before G3? | multi-cut folder set | repeated manual export burden |

## Quality-of-life hypotheses

High-value candidates to validate after G1: search/command palette, exposure-aware navigation to previous/next drawing, reveal source frame, replace drawing in place, stale-media indicator, render-range presets, per-layer cache status, effect preset favorites and clear missing-media diagnostics.

These should be judged by interaction reduction and error prevention, not by resemblance to After Effects.

## Legal/ethical boundary

Study observed behavior and public documentation; do not copy proprietary plugin code, branding, UI artwork or undocumented implementation details. Recreate generic image-processing ideas only through independently specified math/tests and license-compatible dependencies. Compatibility claims must be narrow and evidence-backed.

## Next evidence to collect

1. Two or more rights-cleared anime-style shots with distinct line/lighting needs.
2. At least one traditional/digital exposure sheet or equivalent created/owned by the user.
3. Side-by-side W-01/W-02 trials in existing tools.
4. Short interviews/observations with other animators/compositors when available.
5. Exact license/source review before adopting any third-party implementation.

