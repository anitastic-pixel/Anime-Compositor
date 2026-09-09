# T-06: document 08's performance envelope, measured

Document 08 line 43 says of its own figures: *"These are validation targets, not measured capabilities."* This is the measurement that turns as many of them as this build can into measured capabilities, and names the ones it cannot. Produced by `tests/t06_envelope.rs`, which is `#[ignore]`d in normal runs.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Workload: `verification/B-08a_project.json`, the reference shot, at draft resolution unless a row says otherwise
- Tile size: `compose::DEFAULT_TILE_SIZE`
- Work area: frames 0 to 239, which is the whole shot and what B-10 exports
- Percentiles are by nearest rank on the sorted sample

Debug assertions in this build: false. A run with `true` there is a debug build, and its numbers say more about the compiler than about the renderer.


## The fixture this file could not use, and the one that has since been built

Document 08 line 41 declares the reference fixture as *"1080p, 24 fps, 240 frames, ten raster layers, two alpha mattes and three simple effect instances"*. The reference shot is 1080p, 24 fps and 240 frames exactly, and it has **four raster layers, no mattes and no effects**, because mattes were B-06 and effects were B-07 and both were PARKED under D-12 when this was measured.

Nothing below is a measurement of the declared fixture. Every figure here is a **floor** for it rather than an estimate of it: six more layers cost more, and the two parked features are the two document 08 itself says need bounds expansion and a second evaluation of alpha. That gap is registered as **D-41**.

**B-06 and B-07 have both since landed, and the declared fixture has been built and measured.** It is `verification/T-06_declared_fixture.md`, and it is the file to read for what document 08's own fixture costs - it is about nine times the frame. This file stays what it is, the reference shot's own numbers, because the reference shot is what the viewer actually opens.

## Ten repeated work-area loops

| Loop | Total ms | Median ms | p95 ms | Slowest ms | Frames per second at the median | Decodes | From memory | Cache held (MiB) | Process working set (MiB) |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 10362.0 | 43.29 | 62.04 | 99.87 | 23.1 | 430 | 510 | 1012.5 | 1019.5 |
| 2 | 10656.0 | 45.35 | 62.80 | 69.02 | 22.1 | 429 | 511 | 1012.5 | 1019.5 |
| 3 | 10629.6 | 44.91 | 62.80 | 66.17 | 22.3 | 429 | 511 | 1012.5 | 1021.4 |
| 4 | 10287.1 | 44.03 | 59.84 | 68.60 | 22.7 | 429 | 511 | 1012.5 | 1021.4 |
| 5 | 10247.5 | 44.12 | 59.97 | 64.74 | 22.7 | 429 | 511 | 1012.5 | 1021.9 |
| 6 | 10230.3 | 43.86 | 58.80 | 64.12 | 22.8 | 429 | 511 | 1012.5 | 1019.8 |
| 7 | 10237.0 | 43.59 | 59.17 | 62.69 | 22.9 | 429 | 511 | 1012.5 | 1021.7 |
| 8 | 10275.2 | 43.90 | 60.23 | 65.28 | 22.8 | 429 | 511 | 1012.5 | 1021.9 |
| 9 | 10276.5 | 44.07 | 59.50 | 65.09 | 22.7 | 429 | 511 | 1012.5 | 1021.8 |
| 10 | 10267.4 | 44.39 | 59.27 | 64.94 | 22.5 | 429 | 511 | 1012.5 | 1021.8 |

**Cold-render throughput** is the first row: 10362.0 ms in total for 240 frames, a median of 43.29 ms and 23.1 frames per second. It is the only loop that pays for reading every drawing off the disk for the first time.

**Memory across the ten loops.** The process held 1019.5 MiB at the end of the second loop and 1021.8 MiB at the end of the tenth, a difference of 2.3 MiB across 1,920 renders. `tests/t06_envelope.rs` fails if that difference exceeds one cel, 33,177,600 bytes; the bound is argued in the test's own header rather than picked. Peak working set across the ten loops was 1180.1 MiB. That figure is read before the seek section below, which deliberately allocates a cache sixteen times the viewer's own; folding that experiment into a peak-RAM number would describe the test rather than the program.

**Peak VRAM is not reported, and not because it was forgotten.** There is no GPU path in this build; a VRAM figure would be the desktop's.

**Dropped frames** are D-32's, and they are counted end to end by photographing a running window in `verification/B-08_window_shell.md`, not here. What this table can say is how many frames of the tenth loop cost more than the 41.7 ms a 24 fps frame is allowed: **148 of 240**. Those are the frames the viewer would drop rather than run the shot slow.

**Warm playback sits on the 24 fps deadline, and which side of it is not settled by this file.** A 24 fps clock allows this 240-frame work area 10000.0 ms. On the run that wrote this artifact, **0 of the ten loops came in under that**, the fastest at 10230.3 ms and the slowest at 10656.0 ms - a margin of +2.3% to +6.6% against the deadline.

That margin is small enough that the verdict changes with what else the machine is doing. Run repeatedly on 2026-09-06 on an otherwise ordinary desktop, the count has come back as ten of the ten loops under the deadline, as none of the ten, and as the first six under it with the last four about a fifth behind. Nothing in the build changed between them. **Read the row below as *at the deadline*, not as a pass or a failure**, and read the margin rather than the count: a measurement that flips between runs is a measurement of the margin, and the margin is a few per cent.

The paragraph above is the other half of the same picture: inside the tenth loop, individual frames ran over the 41.7 ms one frame is allowed, and D-32 drops those rather than running the shot slow. Both figures are four layers at draft resolution, and both are a floor for the fixture document 08 declares, which has six more layers, two mattes and three effects. End to end, on a real window, playback is counted in `verification/B-08_window_shell.md`.


## Seeking

A seek is a jump to a frame that is not the next one, which is what scrubbing is and what nothing measured before this had ever done. The walk used here steps through the work area by 97 frames at a time; 97 is coprime with 240, so the walk reaches every frame exactly once in an order that is nowhere near sequential, and it is the same order every run.

| Seek | Budget | Median ms | p95 ms | Slowest ms | Within 100 ms at p95 |
|---|---|---|---|---|---|
| The frame just shown, again | 1 GiB (the viewer's default) | 25.45 | 28.32 | 33.99 | yes |
| A scattered walk of the whole shot | 1 GiB (the viewer's default) | 61.00 | 85.37 | 127.17 | yes |
| The same walk, second pass | 2 GB (room for every drawing) | 22.50 | 38.41 | 47.96 | yes |
| The same walk at full resolution, second pass | 2 GB (room for every drawing) | 35.75 | 38.58 | 39.91 | yes |

### How to read the seek table

**A scrub is the workload the budget decides, and this shot is the small one.** One cel of this composition costs 33,177,600 bytes to hold and every frame of this shot needs four of them, so the 1 GiB default holds several frames of it and repeating a frame is a full hit. Jumping far enough away still evicts and decodes, which is the second row, and a cold decode is what `verification/B-08_preview_latency.md` already measured. The third row is the whole shot in memory: the cache filled to 1771.9 MiB in 56 cels, and a second walk over the same 240 frames decoded 0 further drawings.

The default is what it is because of a heavier shot than this one. `verification/B-08b_cache_budget.md` measured 512 MB against 128 MB on sequential playback of *this* shot and found a fraction of a millisecond between them, which is why the default sat at 128 MB until 2026-09-08. What moved it was `verification/T-06_declared_fixture.md`: on the ten-layer fixture document 08 line 41 actually declares, one frame is 316.4 MiB and 128 MB could not hold it. Four layers were never the shot the number had to be right for.

**Document 08's "p95 cached seek-to-display at or below 100 ms" is met in every row of the table above, the default budget included**, at 85.37 ms for the scattered walk that re-decodes on almost every jump; the slowest single seek of those 240 was 127.17 ms. The default budget is therefore not a failure against that target on this machine. What it is, is a cost, and the cost shows as a comparison rather than as a breach: 61.00 ms at the median re-decoding against 22.50 ms with the drawings already in memory. Whether that trade is the right one is a decision for the owner rather than a change an agent should make, because raising the budget spends memory the machine may not have. Registered as **D-40**. Two things bound the headroom above: every figure here is four layers rather than the ten document 08 declares, and it is one machine.

**Every figure in this table is seek-to-buffer, not seek-to-display.** It stops at a finished picture in memory. Getting that picture onto the screen is the transport, and `verification/B-08_window_shell.md` is the artifact that watches a real window do it. Document 08's phrase is "seek-to-display"; this measures the part of it that a headless test can reach, and the difference is not small — `verification/B-08_preview_latency.md` records that the transport was never measured on this build at all.


## What document 08 line 41 asked for, and what came back

| Asked for | Answer |
|---|---|
| Warm-cache playback at 24 fps | **At the deadline**: 0 of the ten loops of this run came in under the 10000.0 ms a 24 fps clock allows 240 frames, and the count flips between runs. Read the margin above, at draft on four layers |
| p95 cached seek-to-display at or below 100 ms | **Met**, as seek-to-buffer, in every seek measured, the default budget included; worst p95 85.37 ms. What the default costs is D-40 |
| No unbounded memory growth after ten repeated work-area loops | Measured against the operating system's working set, and asserted rather than reported |
| Dropped frames | Counted end to end by photograph in `B-08_window_shell.md`; the frames over budget in the tenth loop are counted here |
| Cold-render throughput | The first loop above |
| Peak RAM | Peak working set above |
| Peak VRAM | **Not measurable**: there is no GPU path in this build |
| The ten-layer, two-matte, three-effect fixture | **Not this file's workload**: it was parked under D-12 when this was measured. Built and measured since, in `verification/T-06_declared_fixture.md`. Every figure here is a floor for it, not an estimate. D-41 |

## Two checks on the cache that every number above rests on

Every figure in this file is read out of the cache, so a fault in the cache's own accounting would move all of them at once without failing anything. A mutation pass found that the budget check believed that accounting, and these two were added because of it. Both are asserted rather than reported, and the run that wrote this file passed them.

- **The held bytes are the cel count times the cel size.** With room for the whole shot the cache held 1771.9 MiB in 56 cels, checked against 56 cels of 33177600 bytes - the same figure derived a second way rather than taken on trust.
- **A budget below one cel refuses rather than churns.** Given 33177599 bytes, one byte less than a cel, the cache held nothing and evicted 0. A cache that admits what it cannot hold and throws it straight back out also ends up empty; the eviction count is the only thing that tells the two apart.

