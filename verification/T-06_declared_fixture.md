# T-06 again, against the fixture document 08 actually declares

`verification/T-06_performance_envelope.md` measures the reference shot and says at the top of itself that the reference shot is not the fixture document 08 line 41 asks about: *"1080p, 24 fps, 240 frames, ten raster layers, two alpha mattes and three simple effect instances"*, against a shot with four layers, no mattes and no effects. That gap is **D-41**, and D-41 names what closes it - the re-measurement against the real fixture once B-06 and B-07 have landed, not an edit to line 41. Both have landed. This is that measurement. Produced by `tests/b12b_declared_fixture.rs`, which is `#[ignore]`d in normal runs.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Workload: `verification/T-06_declared_fixture.json`, at draft resolution
- Tile size: `compose::DEFAULT_TILE_SIZE`
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

The project file is written out beside this one as `verification/T-06_declared_fixture.json`, 204520 bytes, and it is a project this build opens: the test builds it, saves it, and reads it back through `persist::load_str` before measuring anything.

**The ten sequences are copies of the reference shot's four.** The test lays them down under `target/b12b_declared_fixture/` as `copy1` to `copy10`. So this shot has ten layers of drawings and four layers of pictures - the same art appears more than once, and anybody looking at a frame of it would see that.

That is a limitation of what it looks like and not of what it costs, and the difference matters because this file is about cost. Nothing in the render path shares work between two layers reading different files: `src/cache.rs` keys a decoded cel on the file's path, its length, its modification time and its interpretation, so ten sequences are ten decodes and not four. `verification/D-37_decode_cost.md` puts decoding at 75.15 ms of an 81.69 ms draft frame, which is the part being multiplied here. Six more layers of owner-drawn art would change the pictures and would not change a number below.

Two smaller choices, both made towards the heavier reading rather than the cheaper one. The mattes are **alpha** mattes because that is the only kind this build has - `model::MatteReference` holds a layer and a flag, and document 21 composites from the matte layer's alpha. And `matte_only` is **false** on both, so all ten layers still composite; true would keep the two matte layers out of the visible stack and cost less. A floor should cost more rather than less.

Each copied layer keeps the exposure sheet of the reference layer it came from, including layer 3's deliberately missing drawing 7 and layer 4's out-of-order re-exposure. Nothing here is a new timing path; it is the same paths, ten times over.


## Ten repeated work-area loops

| Loop | Total ms | Median ms | p95 ms | Slowest ms | Frames per second at the median | Decodes | From memory | Cache held (MiB) | Process working set (MiB) |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 79287.5 | 343.10 | 365.62 | 377.30 | 2.9 | 2340 | 480 | 126.6 | 136.7 |
| 2 | 81112.0 | 346.10 | 382.05 | 400.29 | 2.9 | 2340 | 480 | 126.6 | 136.7 |
| 3 | 86440.2 | 373.61 | 398.84 | 423.92 | 2.7 | 2340 | 480 | 126.6 | 136.8 |
| 4 | 85798.6 | 372.02 | 389.20 | 411.63 | 2.7 | 2340 | 480 | 126.6 | 136.9 |
| 5 | 87282.4 | 376.85 | 405.08 | 423.73 | 2.7 | 2340 | 480 | 126.6 | 136.9 |
| 6 | 86110.5 | 372.87 | 394.47 | 413.43 | 2.7 | 2340 | 480 | 126.6 | 137.1 |
| 7 | 86852.0 | 375.61 | 397.19 | 431.69 | 2.7 | 2340 | 480 | 126.6 | 136.8 |
| 8 | 86915.0 | 376.01 | 394.64 | 433.72 | 2.7 | 2340 | 480 | 126.6 | 136.8 |
| 9 | 86254.1 | 374.39 | 392.32 | 424.45 | 2.7 | 2340 | 480 | 126.6 | 134.8 |
| 10 | 85712.3 | 371.08 | 388.81 | 398.76 | 2.7 | 2340 | 480 | 126.6 | 136.9 |

**Cold-render throughput** is the first row: 79287.5 ms for 240 frames, a median of 343.10 ms and 2.9 frames per second. It is the only loop that pays for reading every drawing off the disk for the first time.

**Warm playback at 24 fps.** A 24 fps clock allows this 240-frame work area 10000.0 ms, and **0 of the ten loops came in under it**; the tenth loop's median frame cost 371.08 ms against the 41.7 ms one frame is allowed, and **240 of its 240 frames** cost more than that. Those are the frames D-32 drops rather than running the shot slow. End to end, on a real window, playback is counted in `verification/B-08_window_shell.md`.

**Memory across the ten loops.** 136.7 MiB at the end of the second loop, 136.9 MiB at the end of the tenth, a difference of 0.2 MiB across 1,920 renders. The test fails if that exceeds one cel, 33,177,600 bytes. Peak working set across the ten loops was 549.7 MiB, read before the seek section below so that the large cache the seek table allocates does not get reported as the program's appetite.

**Peak VRAM is not reported.** There is no GPU path in this build; a VRAM figure would be the desktop's.


## Seeking

A seek is a jump to a frame that is not the next one. The walk steps through the work area 97 frames at a time; 97 is coprime with 240, so it reaches every frame exactly once in an order nowhere near sequential, and it is the same order every run.

| Seek | Budget | Median ms | p95 ms | Slowest ms | Within 100 ms at p95 |
|---|---|---|---|---|---|
| The frame just shown, again | 128 MB (the viewer's default) | 375.13 | 398.01 | 432.19 | **no** |
| A scattered walk of the whole shot | 128 MB (the viewer's default) | 372.22 | 390.40 | 433.63 | **no** |
| The frame just shown, again | 332 MB (room for one frame) | 195.20 | 210.77 | 234.45 | **no** |
| The same walk, second pass | 332 MB (room for one frame) | 297.84 | 337.21 | 375.81 | **no** |

### How to read the seek table

**At ten layers the viewer's default budget cannot hold a single frame, and that is arithmetic rather than a defect.** One cel of this composition costs 33,177,600 bytes to hold and every frame of this fixture needs ten of them, which is 316.4 MiB. The default budget is 128 MB. So the first row - asking for the frame that was just shown - is not a warm seek at this fixture at all: showing the frame evicts the cels that made it. The third row is the same request at a budget with room for one frame, and the difference between the two rows is the whole of what the budget decides.

The fourth row is the scattered walk with that same one-frame budget, walked twice, the second pass measured. It still re-decodes on every jump - one frame of headroom cannot hold a neighbourhood - so it is a lower bound on the cost of scrubbing this shot and not a picture of a warm cache. Holding every distinct drawing of this fixture is 166 cels, about 5.5 GB, and this file will not allocate that to fill in a table row. The cache in the measured row held 316.4 MiB in 10 cels.

**Every figure here is seek-to-buffer, not seek-to-display.** It stops at a finished picture in memory; the transport into the window is the window's, and `verification/B-08_window_shell.md` is where a real one is watched.


## What this settles, and what it hands back to the owner

**D-41 - "the performance fixture document 08 declares cannot be built in this build".** It can now, and it has been. The entry says what closes it: the re-measurement against the real fixture after B-07. This file is it, and D-41 can be closed. What stays true and should be written into the closing note is the sentence above about the art: ten sequences of drawings, four sequences of pictures. Every figure in this file is a real ten-layer cost and no figure in it is a picture of a ten-layer shot.

**D-40 - the 128 MB preview cache budget.** The owner decided on 2026-09-06 to leave it where it is, and attached a condition: *"if B-06 and B-07 make the reference shot heavy enough that the measured p95 crosses 100 ms, that is a new measurement and reopens this entry rather than contradicting it."* Here is that measurement, on the declared fixture, at the default budget, scrubbing:

| | |
|---|---|
| p95 of a scattered seek, default budget | **390.40 ms** |
| Document 08 line 41's target | 100 ms |
| Crossed | **yes** |

So the condition D-40 attached **has fired**, and D-40 reopens. What reopens it is a measurement and not a disagreement: the owner's reason was that the target was met at this budget, and on the declared fixture it is not. The three answers the entry lists are unchanged - leave the default and let scrubbing pay for itself, raise it to hold a working neighbourhood, or make it a setting with a stated cost - and the third row of the seek table above is what raising it buys.

**And a third thing, which is not in the register at all.** D-40 is about the cache budget and D-41 was about the fixture not existing. Neither of them is the biggest number on this page. Document 08 line 41 also asks for *warm-cache playback at 24 fps*, and on the fixture it declares this build renders **371.08 ms a frame at draft against the 41.7 ms a 24 fps clock allows** - 0 of the ten loops came in under the deadline, and 240 of the tenth loop's 240 frames were over it. That is a factor of about 9, and a factor is not a margin.

Read next to `verification/T-06_performance_envelope.md`, which has the four-layer reference shot sitting *on* the 24 fps deadline and flipping either side of it between runs, this says something specific and worth saying plainly: **nothing here is a regression, and the shot document 08 declares is simply more work than this build does in real time.** Two and a half times the layers costs about nine times the frame, which is more than the layer count alone accounts for and is not explained by anything measured here. `verification/D-37_decode_cost.md` is where the obvious suspect is - decoding was 75.15 ms of an 81.69 ms four-cel draft frame - and the loop table above is consistent with it: every loop decodes 2340 cels and only 480 come from memory, because ten cels of frame do not fit a 128 MB cache and never will.

What that costs the person using it is already decided and needs no new decision: D-32 says the viewer holds real time and drops the frames it cannot make, so this shot plays at the right speed and shows fewer frames rather than playing slowly. What is not decided is whether a 24 fps target for a ten-layer shot is one this project keeps, lowers, or reaches by doing the decoding differently. That is a register entry somebody has to open, and opening it is the owner's.

This file does not act on any of it. Reopening or opening a decision is a note in `Markdown/14_Decisions_Risks.md`, and the register is the owner's.


## What document 08 line 41 asked for, and what came back

| Asked for | Answer |
|---|---|
| The ten-layer, two-matte, three-effect fixture | **Built**, and measured here. D-41 |
| Warm-cache playback at 24 fps | 0 of the ten loops came in under the 10000.0 ms a 24 fps clock allows 240 frames |
| p95 cached seek-to-display at or below 100 ms | **Not met** at the viewer's default budget, as seek-to-buffer: 390.40 ms. D-40 |
| No unbounded memory growth after ten repeated work-area loops | Measured against the operating system's working set, and asserted rather than reported |
| Dropped frames | 240 of the tenth loop's 240 frames cost more than one frame's budget; counted end to end by photograph in `B-08_window_shell.md` |
| Cold-render throughput | The first loop above |
| Peak RAM | 549.7 MiB peak working set |
| Peak VRAM | **Not measurable**: there is no GPU path in this build |

## Two checks on the cache that every number above rests on

Every figure in this file is read out of the cache, so a fault in the cache's own accounting would move all of them at once without failing anything. Both of these are asserted, and the run that wrote this file passed them.

- **The held bytes are the cel count times the cel size.** The cache held 316.4 MiB in 10 cels, checked against 10 cels of 33177600 bytes - the same figure derived a second way.
- **A budget below one cel refuses rather than churns.** Given 33177599 bytes, one less than a cel, the cache held nothing and evicted 0. A cache that admits what it cannot hold and throws it straight back out also ends up empty; the eviction count is the only thing that tells the two apart.

