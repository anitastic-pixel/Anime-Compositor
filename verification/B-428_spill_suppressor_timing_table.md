# B-428: frame times with Spill Suppressor (D-548)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b428_spill_suppressor --no-run` and run with `--ignored b428_spill_suppressor_timing`: once on the card, then once with `B428_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 1% busy; 3.6 GB of its memory held by the desktop and open programs); the same after each. The processor was 2% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 23.4, processor 42.6; Noise, then Spill Suppressor as added (blue, 100) card 37.2, processor 70.9 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Spill Suppressor on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 41.3 |
| Noise, then Spill Suppressor as added (blue, 100) | 19.7 | 64.2 |
| Noise, then Spill Suppressor #28b43c at 60 | 19.6 | 64.4 |

First loops (empty caches): card 16.2, 24.2, 23.5; processor 40.5, 64.0, 63.9.

**Reading it.** On the card Spill Suppressor is Screen Key's tone pass (mode 90), fused after the Noise, each pixel read once; quiet, it adds about 7 ms a frame over the Noise alone (12.3 to 19.7 and 19.6, three layers, about 2.5 ms a 1080p layer), not the 10 to 14 the busy run gave. On the processor it adds about 23 ms a frame (41.3 to 64.2 and 64.4), not 26 to 28, so the card is about three times quicker. Spill Suppressor has no measured stage of its own: in the perf report it is counted as Screen Key's.
