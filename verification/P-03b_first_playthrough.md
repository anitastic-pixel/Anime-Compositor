# P-03(b): a first playthrough, before and after the parallel decode

Every frame below is a cache miss on every layer, with the viewer's own 1 GiB budget (D-40) rather than P-01's 6 GiB. That is what opening a project and pressing play costs on the first pass through the shot, and it is the only cache state a decode done ahead of the layer loop can change: an export and P-01's two cold rows hold `CelCache::none`, which may not keep what it decodes and so is never decoded ahead for, and a warm cache decodes nothing.

Same machine, build and harness as `verification/P-01_frame_trace.md`, and the same twenty frames spread across the shot.

## the reference shot (4 layers) — Draft

Frame time: p50 **26.262 ms**, p95 **35.855 ms**, over 20 frames. That is 0.63x the 41.667 ms a 24 fps clock allows.

Cache over the pass: 31 hits, 47 misses, 15 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 19.419 | 20.705 | 65.0% |
| open and read the cel file | 0.000 | 2.862 | 1.5% |
| bytes to float | 0.000 | 3.495 | 2.0% |
| transfer function and premultiply | 0.000 | 11.307 | 6.0% |
| cache lookup and its copy | 0.002 | 0.004 | 0.0% |
| cache admit and its copy | 0.001 | 2.951 | 3.4% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 1.953 | 2.234 | 7.0% |
| assemble the frame from the tiles | 0.696 | 0.904 | 2.5% |
| encode for the page | 2.872 | 3.343 | 10.7% |
| **unaccounted for** | 0.478 | 0.788 | 1.9% |

## the reference shot (4 layers) — Full

Frame time: p50 **83.750 ms**, p95 **93.074 ms**, over 20 frames. That is 2.01x the 41.667 ms a 24 fps clock allows.

Cache over the pass: 31 hits, 47 misses, 15 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 19.638 | 20.997 | 21.5% |
| open and read the cel file | 0.000 | 3.105 | 0.5% |
| bytes to float | 0.000 | 3.512 | 0.6% |
| transfer function and premultiply | 0.000 | 10.662 | 1.9% |
| cache lookup and its copy | 0.002 | 0.005 | 0.0% |
| cache admit and its copy | 0.001 | 2.574 | 0.8% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 6.470 | 7.116 | 7.7% |
| assemble the frame from the tiles | 10.784 | 11.947 | 12.7% |
| encode for the page | 44.908 | 47.706 | 53.6% |
| **unaccounted for** | 0.451 | 0.811 | 0.6% |

## the declared ten-layer fixture (10 layers) — Draft

Frame time: p50 **188.541 ms**, p95 **206.113 ms**, over 20 frames. That is 4.52x the 41.667 ms a 24 fps clock allows.

Cache over the pass: 86 hits, 148 misses, 116 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 30.728 | 40.884 | 18.9% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.014 | 0.020 | 0.0% |
| cache admit and its copy | 6.717 | 9.603 | 3.5% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 132.974 | 150.084 | 69.5% |
| tile loop: sample and blend | 4.550 | 4.911 | 2.7% |
| assemble the frame from the tiles | 0.742 | 1.013 | 0.4% |
| encode for the page | 4.326 | 5.358 | 2.4% |
| **unaccounted for** | 4.708 | 5.452 | 2.6% |

## the declared ten-layer fixture (10 layers) — Full

Frame time: p50 **257.287 ms**, p95 **285.162 ms**, over 20 frames. That is 6.17x the 41.667 ms a 24 fps clock allows.

Cache over the pass: 86 hits, 148 misses, 116 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 30.071 | 37.082 | 13.1% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.014 | 0.018 | 0.0% |
| cache admit and its copy | 5.894 | 9.087 | 2.3% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 130.985 | 143.230 | 48.4% |
| tile loop: sample and blend | 17.559 | 19.696 | 7.0% |
| assemble the frame from the tiles | 12.768 | 13.498 | 5.0% |
| encode for the page | 54.082 | 65.779 | 22.4% |
| **unaccounted for** | 4.701 | 5.206 | 1.8% |

