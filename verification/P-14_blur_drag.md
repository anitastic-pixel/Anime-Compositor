# P-14: the blur while its radius is being dragged

**Dragging the blur's radius costs 8.912 ms a frame at Draft and 38.701 ms at Full at the median, and the worst single frame of the drag is 10.482 ms and 45.245 ms - 0.25x and 1.09x the 41.667 ms a 24 fps clock allows. The evaluated-effect cache cannot help: the blur is re-evaluated on every one of the 60 frames, and it is 13.5% of the frame at Draft and 18.1% at Full.**

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

Frame time across the drag: p50 **8.912 ms**, p95 **9.465 ms**, worst single frame **10.482 ms**, over 60 frames. Against the 41.667 ms a 24 fps clock allows, that is 0.21x, 0.23x and 0.25x.

Cels over the drag: 720 hits, 0 misses, 0 evictions. Effect results over the drag: 120 hits (the exposure and the tint, which are not being dragged), 60 evaluations, 0 dropped.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.005 | 0.007 | 0.1% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.000 | 0.001 | 0.0% |
| effect: exposure | 0.000 | 0.000 | 0.0% |
| effect: tint | 0.000 | 0.000 | 0.0% |
| effect: gaussian blur | 1.188 | 1.415 | 13.5% |
| effect: line smoothing | 0.000 | 0.000 | 0.0% |
| effect: selective colour blur | 0.000 | 0.000 | 0.0% |
| effect: glow | 0.000 | 0.000 | 0.0% |
| effect: line recolour | 0.000 | 0.000 | 0.0% |
| effect: directional blur | 0.000 | 0.000 | 0.0% |
| effect: select colour | 0.000 | 0.000 | 0.0% |
| effect: line width | 0.000 | 0.000 | 0.0% |
| effect: radial blur | 0.000 | 0.000 | 0.0% |
| effect: bloom | 0.000 | 0.000 | 0.0% |
| effect: colour key | 0.000 | 0.000 | 0.0% |
| effect result cache: lookup and admit | 0.015 | 0.024 | 0.2% |
| tile loop: sample and blend | 5.283 | 5.653 | 59.3% |
| assemble the frame from the tiles | 0.072 | 0.089 | 0.8% |
| encode for the page | 0.476 | 0.689 | 5.6% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 1.830 | 2.170 | 20.5% |

In the order the pointer moved, the first frame of the drag - sigma 4.1 - cost 9.164 ms and the last - sigma 10.0 - cost 8.845 ms. The blur's cost grows with its radius, so a drag that kept going would keep getting slower, and these two numbers are the slope of that rather than the whole of it.


## Full

Frame time across the drag: p50 **38.701 ms**, p95 **42.610 ms**, worst single frame **45.245 ms**, over 60 frames. Against the 41.667 ms a 24 fps clock allows, that is 0.93x, 1.02x and 1.09x.

Cels over the drag: 720 hits, 0 misses, 0 evictions. Effect results over the drag: 120 hits (the exposure and the tint, which are not being dragged), 60 evaluations, 50 dropped.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.006 | 0.008 | 0.0% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 3.430 | 3.836 | 8.9% |
| effect: exposure | 0.000 | 0.000 | 0.0% |
| effect: tint | 0.000 | 0.000 | 0.0% |
| effect: gaussian blur | 6.952 | 7.933 | 18.1% |
| effect: line smoothing | 0.000 | 0.000 | 0.0% |
| effect: selective colour blur | 0.000 | 0.000 | 0.0% |
| effect: glow | 0.000 | 0.000 | 0.0% |
| effect: line recolour | 0.000 | 0.000 | 0.0% |
| effect: directional blur | 0.000 | 0.000 | 0.0% |
| effect: select colour | 0.000 | 0.000 | 0.0% |
| effect: line width | 0.000 | 0.000 | 0.0% |
| effect: radial blur | 0.000 | 0.000 | 0.0% |
| effect: bloom | 0.000 | 0.000 | 0.0% |
| effect: colour key | 0.000 | 0.000 | 0.0% |
| effect result cache: lookup and admit | 2.913 | 4.142 | 6.7% |
| tile loop: sample and blend | 19.806 | 20.844 | 51.2% |
| assemble the frame from the tiles | 0.108 | 0.132 | 0.3% |
| encode for the page | 4.249 | 4.756 | 11.0% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 1.434 | 1.813 | 3.8% |

In the order the pointer moved, the first frame of the drag - sigma 4.1 - cost 36.085 ms and the last - sigma 10.0 - cost 38.405 ms. The blur's cost grows with its radius, so a drag that kept going would keep getting slower, and these two numbers are the slope of that rather than the whole of it.


## Would a person dragging that slider find it usable

The plain sentence document 15 asks this unit for, and the numbers it is drawn from.

| Quality | p50 | worst frame | what the picture does while the pointer moves |
|---|---|---|---|
| Draft | 8.912 ms | 10.482 ms | about 95 updates a second at its worst, and it keeps up with the pointer |
| Full | 38.701 ms | 45.245 ms | about 22 updates a second at its worst, and it follows the pointer a step behind |

## What this does not say

- **Nothing about a wider radius.** The drag measured ends at sigma 10 source pixels. A blur's cost grows with the pixels it touches, so a drag that went further would cost more, and the two-ended figure under each table is the only slope this page has.
- **Nothing about a second machine.** One machine, one build, recorded above.
- **Nothing about run-to-run spread.** This page is one run. Five runs of this harness on this machine while it was being written put the Draft median between 30.9 and 35.0 ms and its worst frame between 38.6 and 41.3, and the Full median between 49.0 and 54.8 with its worst frame between 58.0 and 68.4. The reading above is the same at either end of those ranges, which is why one run is reported rather than three.
- **Nothing about the window's own overhead.** The clone `update_drag` takes and the transport into the web view are outside the measured span, for the reasons the harness gives. Both are small beside a blur; neither is measured here.
- **Nothing about making it faster.** Document 15 puts that out of scope for this unit in the strongest terms it has, and names the two CPU answers that come first if the answer is over budget.
