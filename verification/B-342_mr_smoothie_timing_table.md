# B-342: frame times with Mr. Smoothie (D-462)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b342_mr_smoothie --no-run` and run with `--ignored b342_mr_smoothie_timing`: once on the card, then once with `B342_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 4% busy; 4.2 GB of its memory held by the desktop and open programs); the same after each. The processor was 5% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 17.7, processor 72.5; Noise, then Mr. Smoothie as added card 26.5, processor 109.0 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Mr. Smoothie, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.5 | 43.7 |
| Noise, then Mr. Smoothie as added | 17.5 | 98.9 |
| the same, Hue, smoothness 30, BAB, phase 90 | 20.4 | 105.6 |

First loops (empty caches): card 16.3, 21.9, 23.6; processor 43.6, 97.9, 103.9.

**Reading it.** On the card, Mr. Smoothie as added costs about 1.7 ms a 1080p layer (12.5 to 17.5,
three layers, smoothness 5), and about 2.6 ms at Hue, smoothness 30 (20.4): the channel read, the
Gaussian's two passes, then one bilinear read a pixel. On the processor it costs about 18 and 21
ms a layer (98.9 and 105.6 against 43.7), so the card is about eight to ten times quicker. The busy
run could not say this.
