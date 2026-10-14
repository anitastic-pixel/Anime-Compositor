# B-296: frame times with Grid (D-417)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b296_grid --no-run` and run with `--ignored b296_grid_timing`: once on the card, then once with `B296_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 4.2 GB of its memory held by the desktop and open programs); the same after each. The processor was 6% busy or less before each run. Before that the machine was busy (the card 16 to 16% busy, from programs outside this work), and the runs waited up to 61 s for it to be quiet. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 12.0, processor 44.6; Noise, then Grid as added (border 2, None) card 17.2, processor 69.2 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Grid, every eighth frame asked for whole at Full. **Again** is the median of the loops
after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.6 | 44.3 |
| Noise, then Grid as added (border 2, None) | 17.2 | 50.4 |
| Noise, then Grid, Width Slider 100, border 4, orange at 70, Normal | 17.3 | 51.6 |
| Noise, then Grid, 120 by 80, border 10, feathers 6 and 12, Multiply | 17.3 | 52.7 |

First loops (empty caches): card 16.6, 21.2, 20.1, 20.4; processor 43.2, 49.2, 50.6, 52.4.

**Reading it.** A grid costs about 1.6 ms a 1080p layer on the card (17.2 to 17.3 against 12.6,
over three layers), whatever its settings, and about 2 to 3 ms on the processor (50.4 to 52.7
against 44.3). The card figure is what the busy run gave; the processor one is lower and no longer
spread, so that spread was the other programs.
