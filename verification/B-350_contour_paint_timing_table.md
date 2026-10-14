# B-350: frame times with Contour Paint (D-470)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b350_contour_paint --no-run` and run with `--ignored b350_contour_paint_timing`: once on the card, then once with `B350_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 4.2 GB to 4.3 GB of its memory held by the desktop and open programs); after a run no other build or test was running, but the card was 12% busy with another program (the timing test had ended). The processor was 14% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 17.6, processor 42.8; Noise, then Contour Paint as added (three smears) card 97.7, processor 674.1 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Contour Paint on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.6 | 41.6 |
| Noise, then Contour Paint as added (three smears) | 94.2 | 519.5 |
| the same with fills Over Smeared Image, K-Means, 8 colours | 126.2 | 631.5 |

First loops (empty caches): card 16.8, 97.5, 129.2; processor 41.6, 502.0, 632.1.

**Reading it.** On the card, Contour Paint as added (three smears) costs about 27 ms a 1080p layer
(12.6 to 94.2, three layers), and about 38 ms with fills Over Smeared Image, K-Means, 8 colours
(126.2): per smear pass a luminance read, the tensor, its two Gaussian passes, the tangent and the
smear, each pixel walking 2 ceil(2 s) steps. On the processor it costs about 159 and 197 ms a
layer (519.5 and 631.5 against 41.6), so the card is about five to six times quicker. The busy run
could not say this.
