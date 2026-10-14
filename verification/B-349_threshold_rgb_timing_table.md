# B-349: frame times with Threshold RGB (D-469)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b349_threshold_rgb --no-run` and run with `--ignored b349_threshold_rgb_timing`: once on the card, then once with `B349_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 48.1, processor 94.6; Noise, then Threshold RGB as added (every level 128) card 61.8, processor 192.9 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Threshold RGB on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 41.5 |
| Noise, then Threshold RGB as added (every level 128) | 19.1 | 47.5 |
| Noise, then Threshold RGB, red 200, green 90 turned over, blue 40, blend 30 | 19.2 | 47.9 |

First loops (empty caches): card 17.2, 23.0, 22.2; processor 40.8, 47.4, 47.0.

**Reading it.** On the card, Threshold RGB adds 6.8 ms a frame over the Noise alone as added (12.3 to 19.1, three layers), about 2.3 ms a 1080p layer, and 6.9 ms in the second setting (19.2): one tone pass, each pixel read once, whatever the levels. On the processor it adds about 6 ms (41.5 to 47.5 and 47.9). The whole frame is about 2.5 times quicker on the card (19.1 against 47.5). The busy run could not say; this one can.
