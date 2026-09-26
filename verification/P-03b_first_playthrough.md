# P-03(b): a first playthrough, before and after the parallel decode

Every frame below is a cache miss on every layer, with the viewer's own 1 GiB budget (D-40) rather than P-01's 6 GiB. That is what opening a project and pressing play costs on the first pass through the shot, and it is the only cache state a decode done ahead of the layer loop can change: an export and P-01's two cold rows hold `CelCache::none`, which may not keep what it decodes and so is never decoded ahead for, and a warm cache decodes nothing.

Same machine, build and harness as `verification/P-01_frame_trace.md`, and the same twenty frames spread across the shot.

## the reference shot (4 layers) — Draft

Frame time: p50 **15.585 ms**, p95 **22.086 ms**, over 20 frames. That is 0.37x the 41.667 ms a 24 fps clock allows.

Cache over the pass: 28 hits, 50 misses, 32 evictions.

Effect results over the pass: 0 hits, 0 evaluations, 0 dropped to stay inside the budget.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 9.694 | 13.912 | 64.6% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.003 | 0.009 | 0.0% |
| cache admit and its copy | 2.785 | 6.585 | 15.6% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.000 | 0.000 | 0.0% |
| effect: exposure | 0.000 | 0.000 | 0.0% |
| effect: tint | 0.000 | 0.000 | 0.0% |
| effect: gaussian blur | 0.000 | 0.000 | 0.0% |
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
| effect result cache: lookup and admit | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 2.021 | 2.255 | 12.0% |
| assemble the frame from the tiles | 0.023 | 0.035 | 0.1% |
| encode for the page | 0.498 | 0.650 | 3.1% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 0.736 | 1.133 | 4.6% |

## the reference shot (4 layers) — Full

Frame time: p50 **26.063 ms**, p95 **33.089 ms**, over 20 frames. That is 0.63x the 41.667 ms a 24 fps clock allows.

Cache over the pass: 28 hits, 50 misses, 32 evictions.

Effect results over the pass: 0 hits, 0 evaluations, 0 dropped to stay inside the budget.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 9.626 | 13.560 | 41.4% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.003 | 0.006 | 0.0% |
| cache admit and its copy | 2.909 | 5.713 | 9.8% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.000 | 0.000 | 0.0% |
| effect: exposure | 0.000 | 0.000 | 0.0% |
| effect: tint | 0.000 | 0.000 | 0.0% |
| effect: gaussian blur | 0.000 | 0.000 | 0.0% |
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
| effect result cache: lookup and admit | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 8.035 | 8.511 | 29.7% |
| assemble the frame from the tiles | 0.100 | 0.145 | 0.4% |
| encode for the page | 4.170 | 4.855 | 15.8% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 0.801 | 1.056 | 3.0% |

## the declared ten-layer fixture (10 layers) — Draft

Frame time: p50 **44.955 ms**, p95 **61.409 ms**, over 20 frames. That is 1.08x the 41.667 ms a 24 fps clock allows.

Cache over the pass: 86 hits, 148 misses, 130 evictions.

Effect results over the pass: 18 hits, 38 evaluations, 0 dropped to stay inside the budget.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 23.657 | 32.973 | 57.6% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.013 | 0.035 | 0.0% |
| cache admit and its copy | 9.402 | 15.558 | 20.2% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.001 | 0.002 | 0.0% |
| effect: exposure | 0.105 | 0.142 | 0.2% |
| effect: tint | 0.000 | 0.094 | 0.1% |
| effect: gaussian blur | 0.000 | 2.004 | 1.4% |
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
| effect result cache: lookup and admit | 0.009 | 0.038 | 0.0% |
| tile loop: sample and blend | 5.860 | 6.710 | 12.7% |
| assemble the frame from the tiles | 0.099 | 0.166 | 0.2% |
| encode for the page | 0.582 | 1.194 | 1.4% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 2.849 | 3.517 | 6.0% |

## the declared ten-layer fixture (10 layers) — Full

Frame time: p50 **64.162 ms**, p95 **96.970 ms**, over 20 frames. That is 1.54x the 41.667 ms a 24 fps clock allows.

Cache over the pass: 86 hits, 148 misses, 130 evictions.

Effect results over the pass: 18 hits, 38 evaluations, 24 dropped to stay inside the budget.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 22.883 | 33.011 | 34.8% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.010 | 0.018 | 0.0% |
| cache admit and its copy | 8.155 | 12.727 | 11.4% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 4.357 | 13.315 | 9.8% |
| effect: exposure | 1.578 | 2.470 | 2.2% |
| effect: tint | 0.000 | 1.295 | 0.6% |
| effect: gaussian blur | 0.000 | 8.133 | 4.3% |
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
| effect result cache: lookup and admit | 1.197 | 5.553 | 2.7% |
| tile loop: sample and blend | 19.103 | 22.808 | 25.3% |
| assemble the frame from the tiles | 0.119 | 0.229 | 0.2% |
| encode for the page | 4.260 | 4.661 | 5.6% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 2.136 | 2.994 | 2.9% |

