# Feature priority matrix

Version 0.3 | 2026-09-04 | Accepted for baseline

## Scoring method

Scores are 1 low through 5 high. Workflow value estimates usefulness to the intended anime finishing workflow; complexity includes implementation, testing, interface and migration burden; risk covers correctness and long-term maintenance. Scores are planning estimates, not measured evidence.

Version 0.3 changes the tiering, not the scores. Under D-12, G1 splits into G1-core and G1-rest. A feature moving to G1-rest is parked: still specified, deliberately not built, with a written revisit trigger. Parked is not rejected.

## G1-core

| Feature | Priority | Value | Freq | Complexity | Risk | Requirement |
|---|---|---:|---:|---:|---:|---|
| PNG sequence import | Must | 5 | 5 | 2 | 2 | R-01 |
| Explicit cel exposure holds | Must | 5 | 5 | 3 | 2 | R-02 |
| Raster layer stack | Must | 5 | 5 | 3 | 2 | R-03 |
| 2D transforms, hold and linear keys | Must | 5 | 5 | 3 | 3 | R-03 |
| Normal, multiply, screen, add blending | Must | 4 | 4 | 3 | 4 | R-03, R-10 |
| Viewer and frame stepping | Must | 5 | 5 | 3 | 2 | R-06a |
| Undo and redo command model | Must | 5 | 5 | 4 | 4 | R-07 |
| Safe save, reopen, relink, recovery | Must | 5 | 5 | 4 | 5 | R-08 |
| PNG sequence export | Must | 5 | 5 | 3 | 3 | R-09 |
| Explicit color and alpha handling | Must | 5 | 5 | 4 | 5 | R-10 |
| Offline operation | Must | 5 | 5 | 2 | 2 | R-11 |
| Tile-based multithreaded CPU render | Must | 5 | 5 | 3 | 3 | ADR-011 |
| Render trace diagnostics | Must | 3 | 2 | 2 | 1 | ADR-012 |

Render trace scores low on workflow value and is still a Must, because its value is to the verification model rather than to the artist. Under document 12 it is how a wrong image gets investigated at all.

## G1-rest: parked

| Feature | Value | Complexity | Risk | Requirement | Revisit trigger |
|---|---:|---:|---:|---|---|
| Polygon mask | 4 | 3 | 3 | R-04 | A real shot cannot be finished without one |
| Alpha matte | 5 | 3 | 4 | R-04 | Same |
| Exposure, tint, Gaussian blur effects | 4 | 3 | 3 | R-05 | Repeated manual effort in real shots |

Both remaining entries are parked on triggers worded as things that happen in a real shot, and neither has happened. They are still specified, still not built.

## Unparked on 2026-09-05: the bounded preview cache

| Feature | Value | Complexity | Risk | Requirement | Status |
|---|---:|---:|---:|---|---|
| Bounded preview cache | 5 | 4 | 4 | R-06b | Trigger fired 2026-09-05, unparked the same day - D-37, ADR-015, built as B-08b |

The cache is the most instructive entry in this document, and it is worth reading as a whole sequence rather than as a row. It scored value 5, was a version 0.2 Must, and was parked anyway, because it is a performance optimization for a workflow that had never been run. Its trigger was written as a measurement rather than an opinion, precisely so that neither enthusiasm nor reluctance could move it.

The measurement was taken on 2026-09-05. `verification/B-08_preview_latency.md` records the production preview path at 12.2 frames per second in draft and 10.0 at full resolution against a 24 fps target, with three quarters of every frame spent decoding cels that the resolution choice cannot make cheaper, and `verification/D-37_decode_cost.md` records what a cache would recover: nothing at size one, 54% at size four, 94% at 473 MB. The trigger firing did not unpark it - a fired trigger is a reason to ask the owner, not permission to build - so the ask was recorded as D-37, and the owner answered it the same day.

What the discipline bought is visible in the answer's shape. The cache that was approved is a cache of decoded cels rather than of finished frames, because the measurement said decoding was 75.15 ms of an 81.69 ms frame and rendering was 6.53 ms. A cache built when it first looked attractive would have cached frames.

---

## G2 and beyond

| Feature | Stage | Priority | Value | Complexity | Risk | Requirement |
|---|---|---|---:|---:|---:|---|
| 2.5D flat planes and camera | G2 | Must-next | 5 | 5 | 5 | R-12 |
| Parenting | G2 | Must-next | 4 | 3 | 3 | R-12 |
| Native bounded expressions | G2 | Should-next | 4 | 5 | 5 | R-13 |
| GPU render path | G2+ | Trigger-gated | 4 | 4 | 4 | ADR-006 |
| Precompositions | G3 | Validate | 5 | 5 | 5 | - |
| Adjustment layers | G3 | Validate | 4 | 4 | 4 | - |
| Curve and graph editor | G3 | Validate | 3 | 4 | 3 | - |
| EXR import and export | G3 | Validate | 4 | 4 | 4 | R-15 |
| WAV reference audio | G3 | Validate | 3 | 3 | 2 | R-15 |
| Collect and package project | G3 | Should | 4 | 3 | 3 | R-14 |
| Anime line smoothing | G3 | Research | 5 | 4 | 4 | - |
| Line recolor and color key | G3 | Research | 5 | 3 | 3 | - |
| Directional cel blur | G3 | Research | 4 | 3 | 3 | - |
| Particle system | Parked | Defer | 2 | 5 | 5 | - |
| Tracking and roto automation | Parked | Defer | 3 | 5 | 5 | - |
| Full 3D renderer | Excluded | Reject | 1 | 5 | 5 | - |
| Drawing and vector animation | Excluded | Reject | 2 | 5 | 5 | - |
| AEP importer | Excluded | Reject | 3 | 5 | 5 | - |
| AE binary plugin host | Excluded | Reject | 2 | 5 | 5 | - |

The GPU path is deliberately listed as trigger-gated rather than scheduled. Its trigger is a stopwatch reading on the reference shot, per ADR-006.

## Admission rule

A feature does not enter a build because its value score is high. It must identify a blocked or repeated workflow, its smallest useful behavior, its dependencies, its fixture and its milestone impact.

Implementation agents may not build anything on the parked or excluded lists. Promotion is an owner decision recorded in document 14, never a judgment call made inside an implementation task.

## Review cadence

Re-score after the owner finishes real shots with the tool. Replace estimated value and frequency with observed evidence. Do not adjust scores to justify a preferred feature.

Related documents: 03, 04, 12, 14, 15 and 30.
