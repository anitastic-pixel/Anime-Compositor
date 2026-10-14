# B-462: frame times with Contour Paint, Match to Nearest Custom Color (D-584)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b462_contour_custom --no-run` and run with `--ignored b462_contour_custom_timing`: once on the card, then once with `B462_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 1% busy; 3.7 GB of its memory held by the desktop and open programs); the same after each. The processor was 6% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 12.1, processor 47.9; Noise, then Contour Paint, fills Over Smeared Image matched to 2 custom colours card 101.5, processor 705.1 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Contour Paint filling Over Smeared Image matched to custom colours on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.1 | 40.9 |
| Noise, then Contour Paint, fills Over Smeared Image matched to 2 custom colours | 101.1 | 644.2 |
| the same matched to 8 custom colours | 101.5 | 598.4 |

First loops (empty caches): card 15.8, 105.2, 104.7; processor 40.9, 596.2, 596.9.

**Reading it.** On the card, once warm, Contour Paint, Match to Nearest Custom Color adds about 89 ms a frame over the Noise alone with 2 custom colours (12.1 to 101.1, three layers, about 30 ms a 1080p layer), and the same with 8 (101.5): the same passes as K-Means' fills, with the palette set by hand instead of found, so the number of colours costs nothing measurable. On the processor it costs about 600 to 645 ms a frame (598.4 and 644.2 against 40.9), so the card is about six times quicker. The busy run could not say that; the quiet run can.
