# B-288: frame times with Lens Chromatic Aberration (D-409)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b288_lens_chromatic_aberration --no-run` and run with `--ignored b288_lens_chromatic_aberration_timing`: once on the card, then once with `B288_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 15.7, processor 42.3; Noise, then radial, amount 10, falloff 100 card 16.8, processor 84.5 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame on each of its first three layers, then the effect, every eighth
frame asked for whole at Full. **Again** is the median of the loops after the first (seven on
the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.2 | 41.9 |
| Noise, then radial, amount 10, falloff 100 | 17.3 | 86.5 |
| Noise, then radial, amount 10, falloff 100, fringe blur 100 (11 samples a channel) | 41.8 | 215.4 |
| Noise, then offset, amount 6 at 90 degrees | 16.2 | 74.1 |

First loops (empty caches): card 15.7, 21.9, 45.0, 19.5; processor 41.5, 79.9, 213.8, 73.4.

**Reading it.** On the card, radial and offset without fringe blur add under 2 ms a 1080p layer
(1.7 ms radial, 12.2 to 17.3 over three layers; 1.3 ms offset, 16.2), inside Target P2. Fringe blur
100 at amount 10 takes 11 samples a channel at the corners and adds about 9.9 ms a layer (41.8),
over Target P2's 4 ms; its cost grows with the amount times the fringe blur, as the number of
samples does. On the processor about 15 ms a layer radial, 11 ms offset and 58 ms with fringe blur
100. The same reading as the busy run's.
