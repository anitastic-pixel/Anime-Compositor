# B-429: frame times with Reduce Interlace Flicker (D-549)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b429_reduce_interlace_flicker --no-run` and run with `--ignored b429_reduce_interlace_flicker_timing`: once on the card, then once with `B429_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 3.6 GB of its memory held by the desktop and open programs); the same after each. The processor was 2% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 30.2, processor 63.3; Noise, then Reduce Interlace Flicker as added (softness 1) card 108.8, processor 112.1 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Reduce Interlace Flicker on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 41.0 |
| Noise, then Reduce Interlace Flicker as added (softness 1) | 71.2 | 63.6 |
| Noise, then Reduce Interlace Flicker softness 2.5 | 74.2 | 64.4 |

First loops (empty caches): card 15.9, 64.9, 89.0; processor 40.0, 63.5, 64.8.

**Reading it.** Reduce Interlace Flicker is Gaussian Blur's one-way blur, which is drawn on the processor even on the card's path, so the card's lead over the Noise alone (12.3 against 41.0) is lost once it is added. Quiet, the processor is now a little quicker than the card: 63.6 against 71.2 ms a frame at softness 1, and 64.4 against 74.2 at 2.5, three layers. The busy run had the two about equal at softness 1 and the card quicker at 2.5; that does not hold. It has no measured stage of its own: in the perf report it is counted as Gaussian Blur's.
