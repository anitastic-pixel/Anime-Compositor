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

Frame time: p50 **83.643 ms**, p95 **89.187 ms**, over 20 frames. That is 2.01x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 18.758 | 20.022 | 22.6% |
| bytes to float | 12.886 | 14.227 | 15.5% |
| transfer function and premultiply | 42.923 | 45.309 | 51.3% |
| cache lookup and its copy | 0.000 | 0.001 | 0.0% |
| cache admit and its copy | 0.002 | 0.003 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.000 | 0.000 | 0.0% |
| effect: exposure | 0.000 | 0.000 | 0.0% |
| effect: tint | 0.000 | 0.000 | 0.0% |
| effect: gaussian blur | 0.000 | 0.000 | 0.0% |
| effect result cache: lookup and admit | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 1.872 | 2.109 | 2.3% |
| assemble the frame from the tiles | 0.025 | 0.026 | 0.0% |
| encode for the page | 0.616 | 0.811 | 0.8% |
| **unaccounted for** | 6.392 | 6.757 | 7.6% |

### the reference shot (4 layers) — Draft, application cache cold, operating system file cache warm

Frame time: p50 **84.021 ms**, p95 **87.287 ms**, over 20 frames. That is 2.02x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 18.670 | 19.726 | 22.6% |
| bytes to float | 13.039 | 13.702 | 15.5% |
| transfer function and premultiply | 43.672 | 45.514 | 51.5% |
| cache lookup and its copy | 0.000 | 0.000 | 0.0% |
| cache admit and its copy | 0.002 | 0.003 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.000 | 0.000 | 0.0% |
| effect: exposure | 0.000 | 0.000 | 0.0% |
| effect: tint | 0.000 | 0.000 | 0.0% |
| effect: gaussian blur | 0.000 | 0.000 | 0.0% |
| effect result cache: lookup and admit | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 1.834 | 1.965 | 2.2% |
| assemble the frame from the tiles | 0.024 | 0.028 | 0.0% |
| encode for the page | 0.590 | 0.803 | 0.7% |
| **unaccounted for** | 6.332 | 6.665 | 7.5% |

### the reference shot (4 layers) — Draft, everything warm

Frame time: p50 **2.502 ms**, p95 **2.725 ms**, over 20 frames. That is 0.06x the 41.667 ms a 24 fps clock allows.

Cache over the measured pass: 78 hits, 46 misses in the cache's whole life, 0 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.002 | 0.002 | 0.1% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.000 | 0.000 | 0.0% |
| effect: exposure | 0.000 | 0.000 | 0.0% |
| effect: tint | 0.000 | 0.000 | 0.0% |
| effect: gaussian blur | 0.000 | 0.000 | 0.0% |
| effect result cache: lookup and admit | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 1.642 | 1.821 | 65.4% |
| assemble the frame from the tiles | 0.022 | 0.025 | 0.9% |
| encode for the page | 0.449 | 0.653 | 19.7% |
| **unaccounted for** | 0.337 | 0.409 | 13.9% |

## the reference shot (4 layers), Full resolution

### the reference shot (4 layers) — Full, application cache cold, files first read by this process

Frame time: p50 **92.514 ms**, p95 **96.371 ms**, over 20 frames. That is 2.22x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 18.360 | 20.081 | 20.3% |
| bytes to float | 12.849 | 13.624 | 13.9% |
| transfer function and premultiply | 42.748 | 44.919 | 45.9% |
| cache lookup and its copy | 0.000 | 0.000 | 0.0% |
| cache admit and its copy | 0.002 | 0.003 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.000 | 0.000 | 0.0% |
| effect: exposure | 0.000 | 0.000 | 0.0% |
| effect: tint | 0.000 | 0.000 | 0.0% |
| effect: gaussian blur | 0.000 | 0.000 | 0.0% |
| effect result cache: lookup and admit | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 8.053 | 8.557 | 8.8% |
| assemble the frame from the tiles | 0.096 | 0.119 | 0.1% |
| encode for the page | 3.704 | 4.232 | 4.1% |
| **unaccounted for** | 6.426 | 6.954 | 6.9% |

### the reference shot (4 layers) — Full, application cache cold, operating system file cache warm

Frame time: p50 **92.565 ms**, p95 **94.536 ms**, over 20 frames. That is 2.22x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 18.437 | 19.011 | 20.1% |
| bytes to float | 12.907 | 13.384 | 14.0% |
| transfer function and premultiply | 42.471 | 43.233 | 45.8% |
| cache lookup and its copy | 0.000 | 0.001 | 0.0% |
| cache admit and its copy | 0.002 | 0.003 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.000 | 0.000 | 0.0% |
| effect: exposure | 0.000 | 0.000 | 0.0% |
| effect: tint | 0.000 | 0.000 | 0.0% |
| effect: gaussian blur | 0.000 | 0.000 | 0.0% |
| effect result cache: lookup and admit | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 8.243 | 8.683 | 9.0% |
| assemble the frame from the tiles | 0.095 | 0.128 | 0.1% |
| encode for the page | 3.751 | 4.127 | 4.2% |
| **unaccounted for** | 6.329 | 6.680 | 6.8% |

### the reference shot (4 layers) — Full, everything warm

Frame time: p50 **13.540 ms**, p95 **14.129 ms**, over 20 frames. That is 0.32x the 41.667 ms a 24 fps clock allows.

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
| effect result cache: lookup and admit | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 8.708 | 9.049 | 63.5% |
| assemble the frame from the tiles | 0.100 | 0.111 | 0.7% |
| encode for the page | 4.404 | 4.743 | 32.7% |
| **unaccounted for** | 0.401 | 0.492 | 3.0% |

## the declared ten-layer fixture (10 layers), Draft resolution

### the declared ten-layer fixture (10 layers) — Draft, application cache cold, files first read by this process

Frame time: p50 **250.073 ms**, p95 **256.908 ms**, over 20 frames. That is 6.00x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 38.700 | 39.885 | 15.7% |
| bytes to float | 39.195 | 41.070 | 15.8% |
| transfer function and premultiply | 130.164 | 134.799 | 52.5% |
| cache lookup and its copy | 0.001 | 0.001 | 0.0% |
| cache admit and its copy | 0.008 | 0.010 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.001 | 0.001 | 0.0% |
| effect: exposure | 0.909 | 1.296 | 0.4% |
| effect: tint | 0.746 | 1.083 | 0.3% |
| effect: gaussian blur | 13.871 | 15.463 | 5.2% |
| effect result cache: lookup and admit | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 4.251 | 4.667 | 1.7% |
| assemble the frame from the tiles | 0.022 | 0.030 | 0.0% |
| encode for the page | 0.622 | 0.885 | 0.3% |
| **unaccounted for** | 19.922 | 21.604 | 8.1% |

### the declared ten-layer fixture (10 layers) — Draft, application cache cold, operating system file cache warm

Frame time: p50 **250.039 ms**, p95 **256.201 ms**, over 20 frames. That is 6.00x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 39.007 | 41.074 | 15.9% |
| bytes to float | 39.617 | 41.071 | 16.1% |
| transfer function and premultiply | 129.487 | 133.504 | 51.9% |
| cache lookup and its copy | 0.001 | 0.001 | 0.0% |
| cache admit and its copy | 0.009 | 0.010 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.001 | 0.001 | 0.0% |
| effect: exposure | 0.894 | 1.103 | 0.4% |
| effect: tint | 0.774 | 1.027 | 0.3% |
| effect: gaussian blur | 13.716 | 14.255 | 5.1% |
| effect result cache: lookup and admit | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 4.388 | 4.546 | 1.8% |
| assemble the frame from the tiles | 0.018 | 0.034 | 0.0% |
| encode for the page | 0.665 | 0.926 | 0.3% |
| **unaccounted for** | 20.718 | 21.754 | 8.3% |

### the declared ten-layer fixture (10 layers) — Draft, everything warm

Frame time: p50 **44.164 ms**, p95 **48.483 ms**, over 20 frames. That is 1.06x the 41.667 ms a 24 fps clock allows.

Cache over the measured pass: 234 hits, 136 misses in the cache's whole life, 0 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.021 | 0.027 | 0.1% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 12.376 | 14.995 | 29.6% |
| effect: exposure | 1.297 | 1.485 | 3.1% |
| effect: tint | 0.966 | 1.161 | 2.1% |
| effect: gaussian blur | 15.858 | 17.288 | 35.1% |
| effect result cache: lookup and admit | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 4.347 | 4.641 | 10.3% |
| assemble the frame from the tiles | 0.027 | 0.039 | 0.1% |
| encode for the page | 0.711 | 0.952 | 1.8% |
| **unaccounted for** | 7.626 | 9.408 | 17.9% |

## the declared ten-layer fixture (10 layers), Full resolution

### the declared ten-layer fixture (10 layers) — Full, application cache cold, files first read by this process

Frame time: p50 **271.533 ms**, p95 **297.015 ms**, over 20 frames. That is 6.52x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 39.927 | 44.049 | 14.8% |
| bytes to float | 40.406 | 47.476 | 15.1% |
| transfer function and premultiply | 129.821 | 140.798 | 47.9% |
| cache lookup and its copy | 0.001 | 0.001 | 0.0% |
| cache admit and its copy | 0.008 | 0.010 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.001 | 0.001 | 0.0% |
| effect: exposure | 1.018 | 1.201 | 0.4% |
| effect: tint | 0.766 | 0.945 | 0.3% |
| effect: gaussian blur | 13.691 | 14.205 | 4.6% |
| effect result cache: lookup and admit | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 21.228 | 24.572 | 8.0% |
| assemble the frame from the tiles | 0.069 | 0.149 | 0.0% |
| encode for the page | 4.092 | 5.039 | 1.5% |
| **unaccounted for** | 19.955 | 21.155 | 7.3% |

### the declared ten-layer fixture (10 layers) — Full, application cache cold, operating system file cache warm

Frame time: p50 **288.209 ms**, p95 **315.062 ms**, over 20 frames. That is 6.92x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 43.650 | 46.331 | 15.1% |
| bytes to float | 44.098 | 47.172 | 15.1% |
| transfer function and premultiply | 136.731 | 141.898 | 46.8% |
| cache lookup and its copy | 0.001 | 0.002 | 0.0% |
| cache admit and its copy | 0.009 | 0.010 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 0.001 | 0.001 | 0.0% |
| effect: exposure | 1.228 | 1.373 | 0.4% |
| effect: tint | 0.834 | 1.238 | 0.3% |
| effect: gaussian blur | 13.629 | 16.684 | 4.8% |
| effect result cache: lookup and admit | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 21.679 | 36.207 | 8.4% |
| assemble the frame from the tiles | 0.130 | 0.152 | 0.0% |
| encode for the page | 4.136 | 5.134 | 1.5% |
| **unaccounted for** | 22.013 | 23.538 | 7.6% |

### the declared ten-layer fixture (10 layers) — Full, everything warm

Frame time: p50 **59.790 ms**, p95 **66.582 ms**, over 20 frames. That is 1.43x the 41.667 ms a 24 fps clock allows.

Cache over the measured pass: 234 hits, 136 misses in the cache's whole life, 0 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| decode this frame's cels in parallel | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 0.021 | 0.030 | 0.0% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack: the copy it writes into | 10.697 | 12.874 | 17.8% |
| effect: exposure | 1.238 | 1.548 | 2.1% |
| effect: tint | 0.913 | 0.971 | 1.4% |
| effect: gaussian blur | 14.939 | 17.081 | 23.5% |
| effect result cache: lookup and admit | 0.000 | 0.000 | 0.0% |
| tile loop: sample and blend | 21.745 | 23.106 | 36.9% |
| assemble the frame from the tiles | 0.135 | 0.171 | 0.2% |
| encode for the page | 4.198 | 4.483 | 7.3% |
| **unaccounted for** | 6.365 | 8.247 | 10.7% |

## What this table ranks

The three stages that cost the most in each row, by share, largest first. This is the ordering document 15's P-01 entry says every later entry is ranked by, and it is generated from the tables above rather than typed, so a re-run cannot leave it stale.

| Workload | Quality | Cache state | First | Second | Third |
|---|---|---|---|---|---|
| the reference shot (4 layers) | Draft | application cache cold, files first read by this process | transfer function and premultiply — 51.3% | open and read the cel file — 22.6% | bytes to float — 15.5% |
| the reference shot (4 layers) | Draft | application cache cold, operating system file cache warm | transfer function and premultiply — 51.5% | open and read the cel file — 22.6% | bytes to float — 15.5% |
| the reference shot (4 layers) | Draft | everything warm | tile loop: sample and blend — 65.4% | encode for the page — 19.7% | assemble the frame from the tiles — 0.9% |
| the reference shot (4 layers) | Full | application cache cold, files first read by this process | transfer function and premultiply — 45.9% | open and read the cel file — 20.3% | bytes to float — 13.9% |
| the reference shot (4 layers) | Full | application cache cold, operating system file cache warm | transfer function and premultiply — 45.8% | open and read the cel file — 20.1% | bytes to float — 14.0% |
| the reference shot (4 layers) | Full | everything warm | tile loop: sample and blend — 63.5% | encode for the page — 32.7% | assemble the frame from the tiles — 0.7% |
| the declared ten-layer fixture (10 layers) | Draft | application cache cold, files first read by this process | transfer function and premultiply — 52.5% | bytes to float — 15.8% | open and read the cel file — 15.7% |
| the declared ten-layer fixture (10 layers) | Draft | application cache cold, operating system file cache warm | transfer function and premultiply — 51.9% | bytes to float — 16.1% | open and read the cel file — 15.9% |
| the declared ten-layer fixture (10 layers) | Draft | everything warm | effect: gaussian blur — 35.1% | effect stack: the copy it writes into — 29.6% | tile loop: sample and blend — 10.3% |
| the declared ten-layer fixture (10 layers) | Full | application cache cold, files first read by this process | transfer function and premultiply — 47.9% | bytes to float — 15.1% | open and read the cel file — 14.8% |
| the declared ten-layer fixture (10 layers) | Full | application cache cold, operating system file cache warm | transfer function and premultiply — 46.8% | bytes to float — 15.1% | open and read the cel file — 15.1% |
| the declared ten-layer fixture (10 layers) | Full | everything warm | tile loop: sample and blend — 36.9% | effect: gaussian blur — 23.5% | effect stack: the copy it writes into — 17.8% |

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
