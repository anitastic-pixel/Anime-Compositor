# P-01: where the frame goes

Document 15's P-01. A per-stage timer through one preview frame, on both fixtures, at both preview qualities, in three cache states. Produced by `tests/p01_frame_trace.rs`, which is `#[ignore]`d in normal runs and writes this file under `--release --ignored`.

**This unit optimises nothing.** It measures, and every later entry in document 15's performance section is ranked by the tables below rather than by an estimate in `Markdown/32_Performance_Architecture_Investigation.md` or `Markdown/33_Hardware_Optimization_and_Future_Architecture_Research.md`. Where a number here disagrees with one of those documents, this file is the measurement and they are the research.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: cargo release profile, `opt-level = 3`
- Tile size: `compose::DEFAULT_TILE_SIZE` at full resolution and 
         `compose::DRAFT_TILE_SIZE` at draft, which is what a draft preview is cut into (P-03(f))
- Threads rayon was given: 24
- Sample: 20 frames a row, stepping by 97 through the 240-frame work area, so the sample is spread across the shot rather than taken from one run of drawings
- Percentiles are by nearest rank on the sorted sample
- Frame budget at 24 fps: 41.667 ms

Debug assertions in this build: false. A run with `true` there is a debug build and its numbers say more about the compiler than about the renderer.


## What a frame means here, and where it stops

One frame is `preview::preview_frame_cached` followed by `WorkingBuffer::to_srgb8_straight`, which is what `app/src/main.rs` does before any byte reaches the page. It stops at a `Vec<u8>` in memory. The transport into the web view is the window's and a headless test cannot open one, which is the same boundary `verification/B-08_preview_latency.md` draws and for the same reason.

The `wait for the viewer lock` row is therefore **0.000 ms in every table below, and that zero is a property of this harness rather than of the program.** In the running window, `app/src/main.rs` takes the viewer mutex before planning and holds it through the render and the encode, so a command arriving mid-frame waits for all of it. There is no second thread here to do the waiting. **P-04 is the entry that measures that row**, in a window, and this file is not evidence that the wait is small.

## The three cache states, and what this machine could not establish

| Row | Application cache | Operating system file cache |
|---|---|---|
| application cache cold, files first read by this process | empty, `CelCache::none` | whatever Windows happened to hold |
| application cache cold, operating system file cache warm | empty, `CelCache::none` | warm: every file was read by the row above |
| everything warm | large enough to hold the pass, second time through | warm |

**The first two rows differ only in something this process cannot control.** Emptying the Windows standby list needs `SeProfileSingleProcessPrivilege` and an elevated helper, which this repository does not have and P-01 is not the unit that adds one. If the two rows come out equal, the honest reading is that **this run did not establish a cold file cache**, not that a cold disk is free. Document 33's correction to document 32 — that T-06's "cold" loop was never cold — applies to this file as well, and it is stated here rather than discovered later.

## the reference shot (4 layers), Draft resolution

### the reference shot (4 layers) — Draft, application cache cold, files first read by this process

Frame time: p50 **43.346 ms**, p95 **45.771 ms**, over 20 frames. That is 1.04x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 19.237 | 20.399 | 44.7% |
| bytes to float | 8.589 | 9.862 | 20.0% |
| transfer function and premultiply | 4.017 | 4.906 | 9.5% |
| cache lookup and its copy | 0.000 | 0.001 | 0.0% |
| cache admit and its copy | 0.002 | 0.003 | 0.0% |
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
| tile loop: sample and blend | 1.994 | 2.275 | 4.6% |
| assemble the frame from the tiles | 0.027 | 0.042 | 0.1% |
| encode for the page | 0.638 | 0.854 | 1.6% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 8.528 | 9.389 | 19.5% |

### the reference shot (4 layers) — Draft, application cache cold, operating system file cache warm

Frame time: p50 **42.386 ms**, p95 **47.048 ms**, over 20 frames. That is 1.02x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 19.286 | 20.394 | 45.1% |
| bytes to float | 8.305 | 10.260 | 19.8% |
| transfer function and premultiply | 4.030 | 4.980 | 9.6% |
| cache lookup and its copy | 0.000 | 0.001 | 0.0% |
| cache admit and its copy | 0.002 | 0.003 | 0.0% |
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
| tile loop: sample and blend | 2.026 | 2.165 | 4.7% |
| assemble the frame from the tiles | 0.027 | 0.041 | 0.1% |
| encode for the page | 0.597 | 0.871 | 1.5% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 8.280 | 8.972 | 19.1% |

### the reference shot (4 layers) — Draft, everything warm

Frame time: p50 **2.467 ms**, p95 **2.734 ms**, over 20 frames. That is 0.06x the 41.667 ms a 24 fps clock allows.

Cache over the measured pass: 78 hits, 46 misses in the cache's whole life, 0 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.001 | 0.002 | 0.0% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
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
| tile loop: sample and blend | 1.599 | 1.899 | 65.6% |
| assemble the frame from the tiles | 0.018 | 0.024 | 0.8% |
| encode for the page | 0.493 | 0.648 | 20.1% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 0.339 | 0.362 | 13.5% |

## the reference shot (4 layers), Full resolution

### the reference shot (4 layers) — Full, application cache cold, files first read by this process

Frame time: p50 **52.385 ms**, p95 **55.328 ms**, over 20 frames. That is 1.26x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 19.083 | 20.898 | 37.0% |
| bytes to float | 8.347 | 9.893 | 16.0% |
| transfer function and premultiply | 3.936 | 4.677 | 7.6% |
| cache lookup and its copy | 0.000 | 0.001 | 0.0% |
| cache admit and its copy | 0.002 | 0.003 | 0.0% |
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
| tile loop: sample and blend | 7.861 | 8.480 | 14.9% |
| assemble the frame from the tiles | 0.109 | 0.159 | 0.2% |
| encode for the page | 4.006 | 4.506 | 7.7% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 8.585 | 10.473 | 16.6% |

### the reference shot (4 layers) — Full, application cache cold, operating system file cache warm

Frame time: p50 **54.532 ms**, p95 **57.940 ms**, over 20 frames. That is 1.31x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 21.316 | 22.266 | 38.5% |
| bytes to float | 8.368 | 9.570 | 15.4% |
| transfer function and premultiply | 4.184 | 4.407 | 7.5% |
| cache lookup and its copy | 0.000 | 0.000 | 0.0% |
| cache admit and its copy | 0.002 | 0.004 | 0.0% |
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
| tile loop: sample and blend | 7.874 | 8.829 | 14.4% |
| assemble the frame from the tiles | 0.106 | 0.158 | 0.2% |
| encode for the page | 4.108 | 4.799 | 7.6% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 8.926 | 10.033 | 16.4% |

### the reference shot (4 layers) — Full, everything warm

Frame time: p50 **12.719 ms**, p95 **13.448 ms**, over 20 frames. That is 0.31x the 41.667 ms a 24 fps clock allows.

Cache over the measured pass: 78 hits, 46 misses in the cache's whole life, 0 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.002 | 0.003 | 0.0% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
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
| tile loop: sample and blend | 7.704 | 8.336 | 60.4% |
| assemble the frame from the tiles | 0.124 | 0.165 | 1.0% |
| encode for the page | 4.223 | 4.827 | 34.5% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 0.526 | 0.586 | 4.1% |

## the declared ten-layer fixture (10 layers), Draft resolution

### the declared ten-layer fixture (10 layers) — Draft, application cache cold, files first read by this process

Frame time: p50 **111.526 ms**, p95 **117.697 ms**, over 20 frames. That is 2.68x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 41.632 | 44.958 | 37.4% |
| bytes to float | 24.133 | 25.581 | 21.8% |
| transfer function and premultiply | 12.057 | 13.431 | 10.8% |
| cache lookup and its copy | 0.001 | 0.001 | 0.0% |
| cache admit and its copy | 0.007 | 0.008 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.001 | 0.001 | 0.0% |
| effect: exposure | 0.090 | 0.108 | 0.1% |
| effect: tint | 0.051 | 0.067 | 0.0% |
| effect: gaussian blur | 1.061 | 1.401 | 0.9% |
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
| tile loop: sample and blend | 5.538 | 5.909 | 4.9% |
| assemble the frame from the tiles | 0.098 | 0.119 | 0.1% |
| encode for the page | 0.619 | 0.779 | 0.6% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 26.244 | 28.787 | 23.3% |

### the declared ten-layer fixture (10 layers) — Draft, application cache cold, operating system file cache warm

Frame time: p50 **112.006 ms**, p95 **115.593 ms**, over 20 frames. That is 2.69x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 41.627 | 43.197 | 37.3% |
| bytes to float | 23.747 | 25.458 | 21.3% |
| transfer function and premultiply | 12.271 | 12.926 | 11.0% |
| cache lookup and its copy | 0.001 | 0.001 | 0.0% |
| cache admit and its copy | 0.007 | 0.008 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.001 | 0.001 | 0.0% |
| effect: exposure | 0.093 | 0.121 | 0.1% |
| effect: tint | 0.053 | 0.069 | 0.0% |
| effect: gaussian blur | 1.012 | 1.508 | 0.9% |
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
| tile loop: sample and blend | 5.616 | 5.790 | 4.9% |
| assemble the frame from the tiles | 0.084 | 0.099 | 0.1% |
| encode for the page | 0.601 | 1.061 | 0.6% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 26.624 | 28.086 | 23.7% |

### the declared ten-layer fixture (10 layers) — Draft, everything warm

Frame time: p50 **9.710 ms**, p95 **10.240 ms**, over 20 frames. That is 0.23x the 41.667 ms a 24 fps clock allows.

Cache over the measured pass: 234 hits, 136 misses in the cache's whole life, 0 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.016 | 0.027 | 0.2% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.001 | 0.002 | 0.0% |
| effect: exposure | 0.103 | 0.149 | 1.2% |
| effect: tint | 0.058 | 0.073 | 0.6% |
| effect: gaussian blur | 1.131 | 1.371 | 11.6% |
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
| tile loop: sample and blend | 5.486 | 5.823 | 56.8% |
| assemble the frame from the tiles | 0.086 | 0.106 | 0.9% |
| encode for the page | 0.549 | 0.674 | 6.1% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 2.181 | 2.474 | 22.6% |

## the declared ten-layer fixture (10 layers), Full resolution

### the declared ten-layer fixture (10 layers) — Full, application cache cold, files first read by this process

Frame time: p50 **147.712 ms**, p95 **161.514 ms**, over 20 frames. That is 3.55x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 44.782 | 50.031 | 30.9% |
| bytes to float | 26.691 | 29.796 | 18.1% |
| transfer function and premultiply | 12.578 | 14.306 | 8.7% |
| cache lookup and its copy | 0.001 | 0.001 | 0.0% |
| cache admit and its copy | 0.008 | 0.010 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.001 | 0.002 | 0.0% |
| effect: exposure | 1.139 | 1.595 | 0.8% |
| effect: tint | 1.048 | 1.632 | 0.7% |
| effect: gaussian blur | 7.035 | 8.666 | 4.6% |
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
| tile loop: sample and blend | 20.956 | 21.907 | 14.1% |
| assemble the frame from the tiles | 0.081 | 0.130 | 0.1% |
| encode for the page | 3.985 | 4.655 | 3.1% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 27.996 | 31.136 | 19.0% |

### the declared ten-layer fixture (10 layers) — Full, application cache cold, operating system file cache warm

Frame time: p50 **144.392 ms**, p95 **151.320 ms**, over 20 frames. That is 3.47x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 44.142 | 46.964 | 31.0% |
| bytes to float | 25.538 | 28.365 | 17.9% |
| transfer function and premultiply | 12.518 | 12.970 | 8.7% |
| cache lookup and its copy | 0.001 | 0.001 | 0.0% |
| cache admit and its copy | 0.009 | 0.011 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.001 | 0.002 | 0.0% |
| effect: exposure | 1.126 | 1.729 | 0.9% |
| effect: tint | 1.150 | 1.750 | 0.8% |
| effect: gaussian blur | 6.940 | 8.870 | 4.7% |
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
| tile loop: sample and blend | 20.721 | 21.469 | 14.2% |
| assemble the frame from the tiles | 0.116 | 0.207 | 0.1% |
| encode for the page | 3.941 | 4.222 | 2.8% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 26.863 | 30.704 | 19.0% |

### the declared ten-layer fixture (10 layers) — Full, everything warm

Frame time: p50 **48.224 ms**, p95 **53.468 ms**, over 20 frames. That is 1.16x the 41.667 ms a 24 fps clock allows.

Cache over the measured pass: 234 hits, 136 misses in the cache's whole life, 0 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.020 | 0.025 | 0.0% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 10.221 | 11.054 | 20.3% |
| effect: exposure | 1.095 | 1.380 | 2.4% |
| effect: tint | 0.852 | 1.019 | 1.7% |
| effect: gaussian blur | 7.007 | 7.404 | 13.5% |
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
| tile loop: sample and blend | 18.508 | 21.311 | 39.7% |
| assemble the frame from the tiles | 0.074 | 0.121 | 0.2% |
| encode for the page | 3.880 | 4.451 | 8.4% |
| GPU: send drawings to the card | 0.000 | 0.000 | 0.0% |
| GPU: draw, encode and bring the picture back | 0.000 | 0.000 | 0.0% |
| GPU: paint the picture into the window | 0.000 | 0.000 | 0.0% |
| **unaccounted for** | 6.795 | 7.340 | 13.8% |

## What this table ranks

The three stages that cost the most in each row, by share, largest first. This is the ordering document 15's P-01 entry says every later entry is ranked by, and it is generated from the tables above rather than typed, so a re-run cannot leave it stale.

| Workload | Quality | Cache state | First | Second | Third |
|---|---|---|---|---|---|
| the reference shot (4 layers) | Draft | application cache cold, files first read by this process | open and read the cel file — 44.7% | bytes to float — 20.0% | transfer function and premultiply — 9.5% |
| the reference shot (4 layers) | Draft | application cache cold, operating system file cache warm | open and read the cel file — 45.1% | bytes to float — 19.8% | transfer function and premultiply — 9.6% |
| the reference shot (4 layers) | Draft | everything warm | tile loop: sample and blend — 65.6% | encode for the page — 20.1% | assemble the frame from the tiles — 0.8% |
| the reference shot (4 layers) | Full | application cache cold, files first read by this process | open and read the cel file — 37.0% | bytes to float — 16.0% | tile loop: sample and blend — 14.9% |
| the reference shot (4 layers) | Full | application cache cold, operating system file cache warm | open and read the cel file — 38.5% | bytes to float — 15.4% | tile loop: sample and blend — 14.4% |
| the reference shot (4 layers) | Full | everything warm | tile loop: sample and blend — 60.4% | encode for the page — 34.5% | assemble the frame from the tiles — 1.0% |
| the declared ten-layer fixture (10 layers) | Draft | application cache cold, files first read by this process | open and read the cel file — 37.4% | bytes to float — 21.8% | transfer function and premultiply — 10.8% |
| the declared ten-layer fixture (10 layers) | Draft | application cache cold, operating system file cache warm | open and read the cel file — 37.3% | bytes to float — 21.3% | transfer function and premultiply — 11.0% |
| the declared ten-layer fixture (10 layers) | Draft | everything warm | tile loop: sample and blend — 56.8% | effect: gaussian blur — 11.6% | encode for the page — 6.1% |
| the declared ten-layer fixture (10 layers) | Full | application cache cold, files first read by this process | open and read the cel file — 30.9% | bytes to float — 18.1% | tile loop: sample and blend — 14.1% |
| the declared ten-layer fixture (10 layers) | Full | application cache cold, operating system file cache warm | open and read the cel file — 31.0% | bytes to float — 17.9% | tile loop: sample and blend — 14.2% |
| the declared ten-layer fixture (10 layers) | Full | everything warm | tile loop: sample and blend — 39.7% | effect stack: the copy it writes into — 20.3% | effect: gaussian blur — 13.5% |

## How to read these tables

**The share column is the one that adds up.** A median is not additive — the median of a sum is not the sum of the medians — so the p50 column does not total the frame time and is not meant to. The share column is every frame's nanoseconds in that stage over every frame's nanoseconds altogether, which is exact, and it sums to a hundred with the unaccounted-for row.

**Unaccounted for is real work, not measurement error.** It is the frame minus every named stage: resolving each layer's exposure to a path, checking the file is where the project says, evaluating the animated properties, allocating and dropping the buffers, and the plan structure itself. A large figure there is a finding and names the next thing to instrument; it is not a licence to guess.

**The stages are disjoint and the harness checks it.** Every row asserts that its stages sum to no more than the frame they were measured inside, and that the residual is not negative. A nested pair of timers would fail both.

## What was not measured

- **The viewer lock.** Zero here by construction; P-04 measures it.
- **A cold operating system file cache.** Not establishable from an unprivileged process; see the section above.
- **Export.** ADR-015 keeps the cel cache off the export path, and the export path is not what D-47 is about.
- **Anything about a graphics card.** ADR-006 is the owner's and P-07 is the entry that would put one number in front of it.

No expected value in `Fixtures/` or document 25 was read or written by this file.
