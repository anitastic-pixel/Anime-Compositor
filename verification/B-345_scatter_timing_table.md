# B-345: frame times with Scatter (D-465)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b345_scatter --no-run` and run with `--ignored b345_scatter_timing`: once on the card, then once with `B345_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 2% busy; 4.2 GB of its memory held by the desktop and open programs); the same after each. The processor was 2% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 71.2, processor 121.0; Noise, then Scatter as added (amount 10) card 72.3, processor 189.5 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Scatter on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 41.5 |
| Noise, then Scatter as added (amount 10) | 14.0 | 60.6 |
| the same, amount 127, Randomize Every Frame on | 14.2 | 62.0 |

First loops (empty caches): card 16.1, 18.1, 17.7; processor 41.0, 55.7, 61.3.

**Reading it.** On the card, Scatter costs under 1 ms a 1080p layer as added and at amount 127 with
a new jumble each frame (14.0 and 14.2 against 12.3, three layers): one warp pass, each pixel read
once, whatever the amount. On the processor it costs about 6.5 ms a layer (60.6 and 62.0 against
41.5), so the card is about ten times quicker. The busy run could not say this.
