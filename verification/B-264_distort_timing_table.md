# B-264..B-266: frame times with Flow Motion, Griddler and Fisheye

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b264_flow_motion_griddler_fisheye --no-run` and run with `--ignored b264_distort_timing`: once on the card, then once with `B264_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-09, on a machine that was not quiet), again: Noise alone: card, round 1 14.8, card, round 2 21.7, card, round 3 11.8, processor 61.0; Noise, then Flow Motion as it starts (10 and -10, low): card, round 1 29.0, card, round 2 32.1, card, round 3 17.1, processor 109.5 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-256: the reference shot `verification/B-08a_project.json`, 1920 by 1080,
each of its first three layers a Noise that changes every frame and then the effect, every eighth
frame asked for whole at Full, with Draw on: GPU (`preview_frame_srgb8`; the test checks no frame
fell back to the processor) or by the processor alone (`preview_frame_cached`). **First** is the
median of the first loop of 30 frames, from empty caches; **Again** the median of the loops after
it, seven on the card, two on the processor. Milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone | 16.2 | 12.2 | 41.6 | 42.4 |
| Noise, then Flow Motion as it starts (10 and -10, low) | 21.5 | 18.0 | 68.5 | 69.3 |
| Noise, then Flow Motion with high antialiasing | 73.2 | 69.6 | 229.7 | 229.7 |
| Noise, then Griddler as it starts (tiles of 10, 80 per cent) | 20.6 | 17.0 | 62.5 | 62.8 |
| Noise, then Fisheye as it starts (size 50, convergence 50) | 19.1 | 16.4 | 64.0 | 64.3 |

**Reading it.** Per 1080p layer, from the "again" figures over three layers:

| Effect | Card | Card against its target | Processor |
|---|---|---|---|
| Flow Motion, Low | about 1.9 ms (18.0 against 12.2) | within Target P2's 4 ms | about 9.0 ms (69.3 against 42.4) |
| Flow Motion, High | about 19.1 ms (69.6 against 12.2) | over Target P2's 4 ms; sixteen points a pixel, an option the user chooses | about 62.4 ms (229.7 against 42.4) |
| Griddler | about 1.6 ms (17.0 against 12.2) | within Target P2's 4 ms | about 6.8 ms (62.8 against 42.4) |
| Fisheye | about 1.4 ms (16.4 against 12.2) | within Target P2's 4 ms | about 7.3 ms (64.3 against 42.4) |

The build measured includes 668d623, the change to the card's Flow Motion (whole-number taps skip a slow double-precision division; every picture in the D-385 table byte-for-byte the same), so it compares with the busy run's rounds 2 and 3. The reading is the same as the busy run's round 3: Flow Motion at Low, Griddler and Fisheye within Target P2 on the card, and High, sixteen points a pixel, over it. What the quiet run settles: Griddler is readable (about 1.6 ms a layer), and the processor's Fisheye is about 7.3 ms a layer, not "between 3 and 12"; the processor figures are all lower than the busy run's (Flow Motion Low 9.0 against about 16 ms a layer, High 62 against 78).
