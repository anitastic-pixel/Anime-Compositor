# P-14: the blur while its radius is being dragged

**Dragging the blur's radius costs 34.753 ms a frame at Draft and 52.758 ms at Full at the median, and the worst single frame of the drag is 39.967 ms and 59.720 ms - 0.96x and 1.43x the 41.667 ms a 24 fps clock allows. The evaluated-effect cache cannot help: the blur is re-evaluated on every one of the 60 frames, and it is 64.1% of the frame at Draft and 41.4% at Full.**

Document 15's P-14. D-51 asked for one measurement before any GPU path is started, and this is it. **Nothing here is an optimisation and nothing here changes a pixel.**

The run is a drag the window would send: the playhead held on one frame, and one `SetEffectParameters` per pointer move at the page's own 0.1 step, sixty of them, which is about a second of dragging. Every one of those frames is a new radius, so every one of them is a miss in P-11's evaluated-effect cache *for the blur*, by construction. The hits reported below are the fixture's exposure and tint, which the drag is not holding and whose results stand; the blur is evaluated on every one of the sixty frames, and the harness asserts that rather than assuming it. The cel cache is the viewer's own 1 GiB (D-40) and is warm, because the person has been looking at this frame.

p50, p95 and the worst single frame are all reported, because what makes a drag unpleasant is its worst frame and not its average.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Pool: rayon's default, one thread per hardware thread
- Workload: `verification/T-06_declared_fixture.json`, the declared ten-layer fixture, frame 0
- Harness: `tests/p01_frame_trace.rs`, the `p14_blur_drag` measurement
- The drag: sigma 4.1 to 10.0 source pixels in 60 steps of 0.1


## Draft

Frame time across the drag: p50 **34.753 ms**, p95 **38.960 ms**, worst single frame **39.967 ms**, over 60 frames. Against the 41.667 ms a 24 fps clock allows, that is 0.83x, 0.94x and 0.96x.

Cels over the drag: 720 hits, 0 misses, 0 evictions. Effect results over the drag: 120 hits (the exposure and the tint, which are not being dragged), 60 evaluations, 50 dropped.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.005 | 0.008 | 0.0% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 3.790 | 4.445 | 10.9% |
| effect: exposure | 0.000 | 0.000 | 0.0% |
| effect: tint | 0.000 | 0.000 | 0.0% |
| effect: gaussian blur | 21.434 | 26.122 | 64.1% |
| effect result cache: lookup and admit | 2.614 | 3.607 | 6.7% |
| tile loop: sample and blend | 4.368 | 4.854 | 12.9% |
| assemble the frame from the tiles | 0.021 | 0.033 | 0.1% |
| encode for the page | 0.560 | 0.718 | 1.7% |
| **unaccounted for** | 1.236 | 1.575 | 3.7% |

In the order the pointer moved, the first frame of the drag - sigma 4.1 - cost 29.068 ms and the last - sigma 10.0 - cost 37.146 ms. The blur's cost grows with its radius, so a drag that kept going would keep getting slower, and these two numbers are the slope of that rather than the whole of it.


## Full

Frame time across the drag: p50 **52.758 ms**, p95 **57.644 ms**, worst single frame **59.720 ms**, over 60 frames. Against the 41.667 ms a 24 fps clock allows, that is 1.27x, 1.38x and 1.43x.

Cels over the drag: 720 hits, 0 misses, 0 evictions. Effect results over the drag: 120 hits (the exposure and the tint, which are not being dragged), 60 evaluations, 50 dropped.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.006 | 0.009 | 0.0% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 3.788 | 4.314 | 7.3% |
| effect: exposure | 0.000 | 0.000 | 0.0% |
| effect: tint | 0.000 | 0.000 | 0.0% |
| effect: gaussian blur | 21.961 | 27.718 | 41.4% |
| effect result cache: lookup and admit | 2.397 | 3.351 | 4.1% |
| tile loop: sample and blend | 18.796 | 20.673 | 36.1% |
| assemble the frame from the tiles | 0.138 | 0.180 | 0.3% |
| encode for the page | 4.188 | 5.051 | 8.2% |
| **unaccounted for** | 1.406 | 1.651 | 2.7% |

In the order the pointer moved, the first frame of the drag - sigma 4.1 - cost 41.668 ms and the last - sigma 10.0 - cost 56.231 ms. The blur's cost grows with its radius, so a drag that kept going would keep getting slower, and these two numbers are the slope of that rather than the whole of it.


## Would a person dragging that slider find it usable

The plain sentence document 15 asks this unit for, and the numbers it is drawn from.

| Quality | p50 | worst frame | what the picture does while the pointer moves |
|---|---|---|---|
| Draft | 34.753 ms | 39.967 ms | about 25 updates a second at its worst, and it keeps up with the pointer |
| Full | 52.758 ms | 59.720 ms | about 17 updates a second at its worst, and it follows the pointer a step behind |

## What this does not say

- **Nothing about a wider radius.** The drag measured ends at sigma 10 source pixels. A blur's cost grows with the pixels it touches, so a drag that went further would cost more, and the two-ended figure under each table is the only slope this page has.
- **Nothing about a second machine.** One machine, one build, recorded above.
- **Nothing about run-to-run spread.** This page is one run. Five runs of this harness on this machine while it was being written put the Draft median between 30.9 and 35.0 ms and its worst frame between 38.6 and 41.3, and the Full median between 49.0 and 54.8 with its worst frame between 58.0 and 68.4. The reading above is the same at either end of those ranges, which is why one run is reported rather than three.
- **Nothing about the window's own overhead.** The clone `update_drag` takes and the transport into the web view are outside the measured span, for the reasons the harness gives. Both are small beside a blur; neither is measured here.
- **Nothing about making it faster.** Document 15 puts that out of scope for this unit in the strongest terms it has, and names the two CPU answers that come first if the answer is over budget.
