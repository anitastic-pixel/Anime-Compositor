# P-01: where the frame goes

Document 15's P-01. A per-stage timer through one preview frame, on both fixtures, at both preview qualities, in three cache states. Produced by `tests/p01_frame_trace.rs`, which is `#[ignore]`d in normal runs and writes this file under `--release --ignored`.

**This unit optimises nothing.** It measures, and every later entry in document 15's performance section is ranked by the tables below rather than by an estimate in `Markdown/32_Performance_Architecture_Investigation.md` or `Markdown/33_Hardware_Optimization_and_Future_Architecture_Research.md`. Where a number here disagrees with one of those documents, this file is the measurement and they are the research.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: cargo release profile, `opt-level = 3`
- Tile size: `compose::DEFAULT_TILE_SIZE`
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

Frame time: p50 **129.928 ms**, p95 **136.438 ms**, over 20 frames. That is 3.12x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 20.029 | 21.869 | 15.7% |
| bytes to float | 15.381 | 19.110 | 12.3% |
| transfer function and premultiply | 62.593 | 67.135 | 48.9% |
| cache lookup and its copy | 0.002 | 0.002 | 0.0% |
| cache admit and its copy | 17.160 | 18.799 | 13.4% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 0.001 | 0.002 | 0.0% |
| tile loop: sample and blend | 2.281 | 2.892 | 1.8% |
| assemble the frame from the tiles | 0.724 | 0.971 | 0.6% |
| encode for the page | 2.920 | 3.690 | 2.4% |
| **unaccounted for** | 6.307 | 7.062 | 5.0% |

### the reference shot (4 layers) — Draft, application cache cold, operating system file cache warm

Frame time: p50 **143.507 ms**, p95 **148.115 ms**, over 20 frames. That is 3.44x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 21.523 | 23.613 | 15.3% |
| bytes to float | 15.957 | 18.122 | 11.4% |
| transfer function and premultiply | 72.101 | 75.539 | 50.7% |
| cache lookup and its copy | 0.002 | 0.003 | 0.0% |
| cache admit and its copy | 18.283 | 20.246 | 12.9% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 0.002 | 0.002 | 0.0% |
| tile loop: sample and blend | 2.170 | 2.482 | 1.6% |
| assemble the frame from the tiles | 0.714 | 0.913 | 0.5% |
| encode for the page | 4.033 | 4.914 | 2.9% |
| **unaccounted for** | 6.966 | 7.901 | 4.9% |

### the reference shot (4 layers) — Draft, everything warm

Frame time: p50 **25.472 ms**, p95 **27.948 ms**, over 20 frames. That is 0.61x the 41.667 ms a 24 fps clock allows.

Cache over the measured pass: 78 hits, 46 misses in the cache's whole life, 0 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 14.467 | 15.538 | 55.6% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 0.001 | 0.002 | 0.0% |
| tile loop: sample and blend | 1.956 | 2.280 | 7.8% |
| assemble the frame from the tiles | 0.701 | 0.956 | 2.9% |
| encode for the page | 4.027 | 5.101 | 15.6% |
| **unaccounted for** | 4.610 | 5.435 | 18.2% |

## the reference shot (4 layers), Full resolution

### the reference shot (4 layers) — Full, application cache cold, files first read by this process

Frame time: p50 **217.258 ms**, p95 **229.115 ms**, over 20 frames. That is 5.21x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 21.853 | 22.751 | 10.0% |
| bytes to float | 16.207 | 17.323 | 7.3% |
| transfer function and premultiply | 74.813 | 77.939 | 34.0% |
| cache lookup and its copy | 0.001 | 0.002 | 0.0% |
| cache admit and its copy | 18.239 | 19.131 | 8.2% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 0.002 | 0.002 | 0.0% |
| tile loop: sample and blend | 7.284 | 7.860 | 3.3% |
| assemble the frame from the tiles | 11.779 | 12.871 | 5.5% |
| encode for the page | 64.302 | 68.585 | 28.5% |
| **unaccounted for** | 6.886 | 7.916 | 3.2% |

### the reference shot (4 layers) — Full, application cache cold, operating system file cache warm

Frame time: p50 **219.760 ms**, p95 **225.458 ms**, over 20 frames. That is 5.27x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 21.640 | 23.332 | 9.9% |
| bytes to float | 15.792 | 17.099 | 7.2% |
| transfer function and premultiply | 75.790 | 80.756 | 34.4% |
| cache lookup and its copy | 0.001 | 0.002 | 0.0% |
| cache admit and its copy | 17.740 | 18.636 | 8.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 0.001 | 0.002 | 0.0% |
| tile loop: sample and blend | 7.153 | 7.975 | 3.3% |
| assemble the frame from the tiles | 11.892 | 12.891 | 5.4% |
| encode for the page | 63.094 | 66.676 | 28.6% |
| **unaccounted for** | 7.186 | 7.872 | 3.2% |

### the reference shot (4 layers) — Full, everything warm

Frame time: p50 **101.465 ms**, p95 **107.898 ms**, over 20 frames. That is 2.44x the 41.667 ms a 24 fps clock allows.

Cache over the measured pass: 78 hits, 46 misses in the cache's whole life, 0 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 13.770 | 15.151 | 13.4% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 0.001 | 0.002 | 0.0% |
| tile loop: sample and blend | 7.082 | 7.697 | 6.9% |
| assemble the frame from the tiles | 11.848 | 12.576 | 11.7% |
| encode for the page | 63.484 | 71.078 | 63.2% |
| **unaccounted for** | 4.766 | 5.831 | 4.7% |

## the declared ten-layer fixture (10 layers), Draft resolution

### the declared ten-layer fixture (10 layers) — Draft, application cache cold, files first read by this process

Frame time: p50 **483.489 ms**, p95 **500.151 ms**, over 20 frames. That is 11.60x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 44.375 | 45.423 | 9.3% |
| bytes to float | 47.381 | 50.470 | 10.1% |
| transfer function and premultiply | 172.571 | 177.051 | 36.4% |
| cache lookup and its copy | 0.004 | 0.007 | 0.0% |
| cache admit and its copy | 53.248 | 54.824 | 11.2% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 135.389 | 145.981 | 26.6% |
| tile loop: sample and blend | 4.790 | 5.480 | 1.0% |
| assemble the frame from the tiles | 0.492 | 0.856 | 0.1% |
| encode for the page | 3.806 | 4.527 | 0.8% |
| **unaccounted for** | 21.261 | 22.752 | 4.5% |

### the declared ten-layer fixture (10 layers) — Draft, application cache cold, operating system file cache warm

Frame time: p50 **485.409 ms**, p95 **497.279 ms**, over 20 frames. That is 11.65x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 44.028 | 45.128 | 9.2% |
| bytes to float | 47.451 | 49.266 | 10.0% |
| transfer function and premultiply | 173.621 | 178.385 | 36.7% |
| cache lookup and its copy | 0.004 | 0.005 | 0.0% |
| cache admit and its copy | 52.907 | 54.285 | 11.1% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 138.345 | 145.987 | 26.7% |
| tile loop: sample and blend | 4.785 | 5.255 | 1.0% |
| assemble the frame from the tiles | 0.490 | 0.709 | 0.1% |
| encode for the page | 3.974 | 4.525 | 0.8% |
| **unaccounted for** | 21.025 | 21.921 | 4.4% |

### the declared ten-layer fixture (10 layers) — Draft, everything warm

Frame time: p50 **215.535 ms**, p95 **225.299 ms**, over 20 frames. That is 5.17x the 41.667 ms a 24 fps clock allows.

Cache over the measured pass: 234 hits, 136 misses in the cache's whole life, 0 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 43.754 | 46.219 | 21.6% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 144.260 | 151.848 | 65.2% |
| tile loop: sample and blend | 4.242 | 4.498 | 2.1% |
| assemble the frame from the tiles | 0.589 | 0.858 | 0.3% |
| encode for the page | 3.981 | 5.129 | 2.0% |
| **unaccounted for** | 18.666 | 20.697 | 8.8% |

## the declared ten-layer fixture (10 layers), Full resolution

### the declared ten-layer fixture (10 layers) — Full, application cache cold, files first read by this process

Frame time: p50 **579.398 ms**, p95 **592.875 ms**, over 20 frames. That is 13.91x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 43.088 | 44.573 | 7.7% |
| bytes to float | 47.120 | 49.424 | 8.4% |
| transfer function and premultiply | 175.316 | 178.983 | 30.7% |
| cache lookup and its copy | 0.004 | 0.005 | 0.0% |
| cache admit and its copy | 52.439 | 53.954 | 9.2% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 142.952 | 152.466 | 23.2% |
| tile loop: sample and blend | 19.717 | 20.586 | 3.5% |
| assemble the frame from the tiles | 10.982 | 11.542 | 1.9% |
| encode for the page | 66.009 | 69.330 | 11.8% |
| **unaccounted for** | 21.131 | 22.358 | 3.7% |

### the declared ten-layer fixture (10 layers) — Full, application cache cold, operating system file cache warm

Frame time: p50 **578.628 ms**, p95 **585.801 ms**, over 20 frames. That is 13.89x the 41.667 ms a 24 fps clock allows.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 43.648 | 45.051 | 7.7% |
| bytes to float | 46.935 | 48.428 | 8.3% |
| transfer function and premultiply | 174.357 | 180.772 | 30.9% |
| cache lookup and its copy | 0.004 | 0.005 | 0.0% |
| cache admit and its copy | 52.433 | 54.077 | 9.3% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 143.296 | 148.740 | 23.3% |
| tile loop: sample and blend | 19.545 | 20.690 | 3.5% |
| assemble the frame from the tiles | 9.045 | 10.392 | 1.7% |
| encode for the page | 65.093 | 69.255 | 11.6% |
| **unaccounted for** | 20.763 | 22.871 | 3.7% |

### the declared ten-layer fixture (10 layers) — Full, everything warm

Frame time: p50 **292.281 ms**, p95 **301.591 ms**, over 20 frames. That is 7.01x the 41.667 ms a 24 fps clock allows.

Cache over the measured pass: 234 hits, 136 misses in the cache's whole life, 0 evictions.

| Stage | p50 ms | p95 ms | share of the frame |
|---|---|---|---|
| wait for the viewer lock | 0.000 | 0.000 | 0.0% |
| open and read the cel file | 0.000 | 0.000 | 0.0% |
| bytes to float | 0.000 | 0.000 | 0.0% |
| transfer function and premultiply | 0.000 | 0.000 | 0.0% |
| cache lookup and its copy | 41.343 | 43.995 | 14.6% |
| cache admit and its copy | 0.000 | 0.000 | 0.0% |
| layer mask | 0.000 | 0.000 | 0.0% |
| effect stack | 141.617 | 148.794 | 46.2% |
| tile loop: sample and blend | 18.825 | 20.420 | 6.8% |
| assemble the frame from the tiles | 9.148 | 11.050 | 3.4% |
| encode for the page | 65.022 | 70.659 | 23.7% |
| **unaccounted for** | 14.503 | 16.905 | 5.2% |

## What this table ranks

The three stages that cost the most in each row, by share, largest first. This is the ordering document 15's P-01 entry says every later entry is ranked by, and it is generated from the tables above rather than typed, so a re-run cannot leave it stale.

| Workload | Quality | Cache state | First | Second | Third |
|---|---|---|---|---|---|
| the reference shot (4 layers) | Draft | application cache cold, files first read by this process | transfer function and premultiply — 48.9% | open and read the cel file — 15.7% | cache admit and its copy — 13.4% |
| the reference shot (4 layers) | Draft | application cache cold, operating system file cache warm | transfer function and premultiply — 50.7% | open and read the cel file — 15.3% | cache admit and its copy — 12.9% |
| the reference shot (4 layers) | Draft | everything warm | cache lookup and its copy — 55.6% | encode for the page — 15.6% | tile loop: sample and blend — 7.8% |
| the reference shot (4 layers) | Full | application cache cold, files first read by this process | transfer function and premultiply — 34.0% | encode for the page — 28.5% | open and read the cel file — 10.0% |
| the reference shot (4 layers) | Full | application cache cold, operating system file cache warm | transfer function and premultiply — 34.4% | encode for the page — 28.6% | open and read the cel file — 9.9% |
| the reference shot (4 layers) | Full | everything warm | encode for the page — 63.2% | cache lookup and its copy — 13.4% | assemble the frame from the tiles — 11.7% |
| the declared ten-layer fixture (10 layers) | Draft | application cache cold, files first read by this process | transfer function and premultiply — 36.4% | effect stack — 26.6% | cache admit and its copy — 11.2% |
| the declared ten-layer fixture (10 layers) | Draft | application cache cold, operating system file cache warm | transfer function and premultiply — 36.7% | effect stack — 26.7% | cache admit and its copy — 11.1% |
| the declared ten-layer fixture (10 layers) | Draft | everything warm | effect stack — 65.2% | cache lookup and its copy — 21.6% | tile loop: sample and blend — 2.1% |
| the declared ten-layer fixture (10 layers) | Full | application cache cold, files first read by this process | transfer function and premultiply — 30.7% | effect stack — 23.2% | encode for the page — 11.8% |
| the declared ten-layer fixture (10 layers) | Full | application cache cold, operating system file cache warm | transfer function and premultiply — 30.9% | effect stack — 23.3% | encode for the page — 11.6% |
| the declared ten-layer fixture (10 layers) | Full | everything warm | effect stack — 46.2% | encode for the page — 23.7% | cache lookup and its copy — 14.6% |

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
