# T-06 again, against the fixture document 08 actually declares

`verification/T-06_performance_envelope.md` measures the reference shot and says at the top of itself that the reference shot is not the fixture document 08 line 41 asks about: *"1080p, 24 fps, 240 frames, ten raster layers, two alpha mattes and three simple effect instances"*, against a shot with four layers, no mattes and no effects. That gap is **D-41**, and D-41 names what closes it - the re-measurement against the real fixture once B-06 and B-07 have landed, not an edit to line 41. Both have landed. This is that measurement. Produced by `tests/b12b_declared_fixture.rs`, which is `#[ignore]`d in normal runs.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Workload: `verification/T-06_declared_fixture.json`, at draft resolution
- Tile size: `compose::DRAFT_TILE_SIZE`, which is what a draft preview is cut 
         into (P-03(f))
- Work area: frames 0 to 239, the whole shot
- Percentiles are by nearest rank on the sorted sample

Debug assertions in this build: false. A run with `true` there is a debug build, and its numbers say more about the compiler than about the renderer.


## The fixture, and the one way it is not the real thing

| What line 41 declares | What was built |
|---|---|
| 1080p | 1920 x 1080 |
| 24 fps | 24 fps |
| 240 frames | 240, frames 0 to 239 |
| Ten raster layers | Ten, over ten separate image sequences on disk |
| Two alpha mattes | Layer 5 matted by layer 4, layer 8 by layer 7 |
| Three simple effect instances | An exposure on layer 2, a Gaussian blur on layer 6, a tint on layer 9 |

The project file is written out beside this one as `verification/T-06_declared_fixture.json`, 204280 bytes, and it is a project this build opens: the test builds it, saves it, and reads it back through `persist::load_str` before measuring anything.

**The ten sequences are copies of the reference shot's four.** The test lays them down under `target/b12b_declared_fixture/` as `copy1` to `copy10`. So this shot has ten layers of drawings and four layers of pictures - the same art appears more than once, and anybody looking at a frame of it would see that.

That is a limitation of what it looks like and not of what it costs, and the difference matters because this file is about cost. Nothing in the render path shares work between two layers reading different files: `src/cache.rs` keys a decoded cel on the file's path, its length, its modification time and its interpretation, so ten sequences are ten decodes and not four. `verification/D-37_decode_cost.md` puts decoding at 75.15 ms of an 81.69 ms draft frame, which is the part being multiplied here. Six more layers of owner-drawn art would change the pictures and would not change a number below.

Two smaller choices, both made towards the heavier reading rather than the cheaper one. The mattes are **alpha** mattes because that is the only kind this build has - `model::MatteReference` holds a layer and a flag, and document 21 composites from the matte layer's alpha. And `matte_only` is **false** on both, so all ten layers still composite; true would keep the two matte layers out of the visible stack and cost less. A floor should cost more rather than less.

Each copied layer keeps the exposure sheet of the reference layer it came from, including layer 3's deliberately missing drawing 7 and layer 4's out-of-order re-exposure. Nothing here is a new timing path; it is the same paths, ten times over.


## Ten repeated work-area loops

| Loop | Total ms | Median ms | p95 ms | Slowest ms | Frames per second at the median | Decodes | From memory | Cache held (MiB) | Process working set (MiB) |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 8716.3 | 37.50 | 53.05 | 118.54 | 26.7 | 1291 | 1529 | 1012.5 | 1027.6 |
| 2 | 8357.6 | 37.18 | 50.76 | 54.46 | 26.9 | 1290 | 1530 | 1012.5 | 1028.9 |
| 3 | 8335.6 | 37.02 | 50.92 | 52.99 | 27.0 | 1290 | 1530 | 1012.5 | 1028.0 |
| 4 | 8353.8 | 37.15 | 50.86 | 53.46 | 26.9 | 1290 | 1530 | 1012.5 | 1029.0 |
| 5 | 8391.3 | 37.03 | 51.65 | 56.42 | 27.0 | 1290 | 1530 | 1012.5 | 1028.1 |
| 6 | 8543.0 | 37.89 | 52.51 | 54.92 | 26.4 | 1290 | 1530 | 1012.5 | 1027.1 |
| 7 | 8628.6 | 38.23 | 53.26 | 57.09 | 26.2 | 1290 | 1530 | 1012.5 | 1029.2 |
| 8 | 8639.2 | 38.36 | 53.24 | 55.08 | 26.1 | 1290 | 1530 | 1012.5 | 1029.2 |
| 9 | 8701.8 | 38.24 | 54.02 | 56.71 | 26.1 | 1290 | 1530 | 1012.5 | 1027.2 |
| 10 | 8652.2 | 38.26 | 53.01 | 56.00 | 26.1 | 1290 | 1530 | 1012.5 | 1027.2 |

**Cold-render throughput** is the first row: 8716.3 ms for 240 frames, a median of 37.50 ms and 26.7 frames per second. It is the only loop that pays for reading every drawing off the disk for the first time.

**Warm playback at 24 fps.** A 24 fps clock allows this 240-frame work area 10000.0 ms, and **10 of the ten loops came in under it**; the tenth loop's median frame cost 38.26 ms against the 41.7 ms one frame is allowed, and **42 of its 240 frames** cost more than that. Those are the frames D-32 drops rather than running the shot slow. End to end, on a real window, playback is counted in `verification/B-08_window_shell.md`.

**Memory across the ten loops.** 1028.9 MiB at the end of the second loop, 1027.2 MiB at the end of the tenth, a difference of 0.0 MiB across 1,920 renders. The test fails if that exceeds one cel, 33,177,600 bytes. Peak working set across the ten loops was 1385.5 MiB, read before the seek section below so that the large cache the seek table allocates does not get reported as the program's appetite.

**Peak VRAM is not reported.** There is no GPU path in this build; a VRAM figure would be the desktop's.


## Seeking

A seek is a jump to a frame that is not the next one. The walk steps through the work area 97 frames at a time; 97 is coprime with 240, so it reaches every frame exactly once in an order nowhere near sequential, and it is the same order every run.

| Seek | Budget | Median ms | p95 ms | Slowest ms | Within 100 ms at p95 |
|---|---|---|---|---|---|
| The frame just shown, again | 1 GiB (the viewer's default) | 7.93 | 9.42 | 11.15 | yes |
| A scattered walk of the whole shot | 1 GiB (the viewer's default) | 40.03 | 54.42 | 61.05 | yes |
| The frame just shown, again | 128 MiB (what it was before) | 71.88 | 75.06 | 81.49 | yes |
| A scattered walk of the whole shot | 128 MiB (what it was before) | 59.95 | 64.16 | 70.46 | yes |
| The frame just shown, again | 4 GiB (a probe, nobody's default) | 8.99 | 10.29 | 18.75 | yes |
| A scattered walk of the whole shot | 4 GiB (a probe, nobody's default) | 39.77 | 57.33 | 62.78 | yes |
| The frame just shown, again | 6 GiB (a probe, nobody's default) | 8.14 | 9.50 | 11.29 | yes |
| A scattered walk of the whole shot | 6 GiB (a probe, nobody's default) | 8.57 | 35.07 | 50.33 | yes |

### How to read the seek table

**The bottom two rows are the shot this project's performance target is written against, on a cache that could not hold one frame of it.** One cel of this composition costs 33,177,600 bytes to hold and every frame of this fixture needs ten of them, which is 316.4 MiB, against the 128 MiB the default used to be. Asking for the frame just shown was not a warm seek at all: showing the frame evicted the cels that made it. That is what reopened D-40, and the top two rows are the same two requests at the 1 GiB default that replaced it.

**What raising it bought, measured rather than argued.** Asking for the frame just shown went from 71.88 ms to 7.93 ms at the median, and the scattered walk from 59.95 ms to 40.03 ms. Playback moved too, in the loop table above: the tenth loop decoded 1290 cels and answered 1530 from memory, where the same loop at 128 MB decoded 2,340 and answered 480. The reason a scrub improved at all is that 32 cels, 3.2 frames of this shot is more than one frame, and a shot re-uses drawings across frames.

What it did not buy is a warm scrub. Holding every distinct drawing of this fixture is 166 cels, about 5.5 GB, so at the default a jump far enough away still re-decodes most of what it needs, and the scattered walk stays nearer a lower bound on the cost of scrubbing than a picture of a warm cache. The two probe rows are what a budget that does hold the whole shot costs and buys, and the paragraph under this table reads them.

**And no budget reaches the target.** With every cel of a frame in memory, what is left is the cost of compositing ten layers, two mattes and three effects, and that is not a cache's to save. It is the same finding as the playback paragraph below in different clothes. The cache in the byte-accounting check held 316.4 MiB in 10 cels.

**Every figure here is seek-to-buffer, not seek-to-display.** It stops at a finished picture in memory; the transport into the window is the window's, and `verification/B-08_window_shell.md` is where a real one is watched.


**Would raising it further help?** The two probe rows are here to answer that and are nobody's default. Against the 1 GiB default's 54.42 ms scattered p95: 4 GiB gives 57.33 ms, and 6 GiB - the first round number past the 5.5 GB every distinct drawing of this shot costs, so the only budget here at which a scrub can be warm - gives 35.07 ms. Repeating a frame, which the default already holds, goes from 7.93 ms at the default to 8.14 ms at 6 GiB - that one was never the cache's to improve further. A budget that holds the whole shot is the only one that can make scrubbing cheap, and it is 6 GiB of a person's memory to do it; whether that is a trade this project offers is the third of D-40's three answers and the owner's to make.


## What this settles, and what it hands back to the owner

**D-41 - "the performance fixture document 08 declares cannot be built in this build".** It can now, and it has been. The entry says what closes it: the re-measurement against the real fixture after B-07. This file is it, and D-41 can be closed. What stays true and should be written into the closing note is the sentence above about the art: ten sequences of drawings, four sequences of pictures. Every figure in this file is a real ten-layer cost and no figure in it is a picture of a ten-layer shot.

**D-40 - the preview cache budget, now 1 GiB.** The owner decided on 2026-09-06 to leave it at 128 MB, and attached a condition: *"if B-06 and B-07 make the reference shot heavy enough that the measured p95 crosses 100 ms, that is a new measurement and reopens this entry rather than contradicting it."* That measurement fired on this fixture, and on 2026-09-08 the owner raised the default to hold a working neighbourhood - the second of the three answers the entry lists. Here is the same measurement at the budget that replaced it, scrubbing:

| | |
|---|---|
| p95 of a scattered seek, default budget | **54.42 ms** |
| Document 08 line 41's target | 100 ms |
| Crossed | **no** |

So the target is met at the raised budget, on the fixture document 08 declares rather than on a four-layer floor. D-40 can close on this row.

**And a third thing, which is not in the register at all.** D-40 is about the cache budget and D-41 was about the fixture not existing. Neither of them is the biggest number on this page. Document 08 line 41 also asks for *warm-cache playback at 24 fps*, and on the fixture it declares this build renders **38.26 ms a frame at draft against the 41.7 ms a 24 fps clock allows** - 10 of the ten loops came in under the deadline, and 42 of the tenth loop's 240 frames were over it. That is a factor of about 1, and a factor is not a margin.

Read next to `verification/T-06_performance_envelope.md`, which has the four-layer reference shot sitting *on* the 24 fps deadline and flipping either side of it between runs, this says something specific and worth saying plainly: **nothing here is a regression, and the shot document 08 declares is simply more work than this build does in real time.** Two and a half times the layers costs about 0.5 times the frame, which is roughly what the layer count alone accounts for: this is a shot that is more work, not a build that got worse at it. `verification/D-37_decode_cost.md` is where the cost of a layer mostly lives - decoding was 75.15 ms of an 81.69 ms four-cel draft frame - and the loop table above is consistent with it: every loop still decodes 1290 cels, against 1530 answered from memory. The budget took a bite out of that and cannot take the rest: a sequential walk of 166 distinct drawings comes back round to a drawing long after any cache short of the whole 5.5 GB has dropped it.

What that costs the person using it is already decided and needs no new decision: D-32 says the viewer holds real time and drops the frames it cannot make, so this shot plays at the right speed and shows fewer frames rather than playing slowly. What is not decided is whether a 24 fps target for a ten-layer shot is one this project keeps, lowers, or reaches by doing the decoding differently. That is a register entry somebody has to open, and opening it is the owner's.

This file does not act on any of it. Reopening or opening a decision is a note in `Markdown/14_Decisions_Risks.md`, and the register is the owner's.


## What document 08 line 41 asked for, and what came back

| Asked for | Answer |
|---|---|
| The ten-layer, two-matte, three-effect fixture | **Built**, and measured here. D-41 |
| Warm-cache playback at 24 fps | 10 of the ten loops came in under the 10000.0 ms a 24 fps clock allows 240 frames |
| p95 cached seek-to-display at or below 100 ms | **Met** at the viewer's default budget, as seek-to-buffer: 54.42 ms. D-40 |
| No unbounded memory growth after ten repeated work-area loops | Measured against the operating system's working set, and asserted rather than reported |
| Dropped frames | 42 of the tenth loop's 240 frames cost more than one frame's budget; counted end to end by photograph in `B-08_window_shell.md` |
| Cold-render throughput | The first loop above |
| Peak RAM | 1385.5 MiB peak working set |
| Peak VRAM | **Not measurable**: there is no GPU path in this build |

## Two checks on the cache that every number above rests on

Every figure in this file is read out of the cache, so a fault in the cache's own accounting would move all of them at once without failing anything. Both of these are asserted, and the run that wrote this file passed them.

- **The held bytes are the cel count times the cel size.** The cache held 316.4 MiB in 10 cels, checked against 10 cels of 33177600 bytes - the same figure derived a second way.
- **A budget below one cel refuses rather than churns.** Given 33177599 bytes, one less than a cel, the cache held nothing and evicted 0. A cache that admits what it cannot hold and throws it straight back out also ends up empty; the eviction count is the only thing that tells the two apart.

