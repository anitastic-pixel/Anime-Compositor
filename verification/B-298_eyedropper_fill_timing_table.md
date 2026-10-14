# B-298: frame times with Eyedropper Fill (D-419)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b298_eyedropper_fill --no-run` and run with `--ignored b298_eyedropper_fill_timing`: once on the card, then once with `B298_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 4.2 GB of its memory held by the desktop and open programs); the same after each. The processor was 2% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 133.2, processor 102.3; Noise, then Eyedropper Fill as added (one pixel) card 136.8, processor 158.9 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then an Eyedropper Fill, every eighth frame asked for whole at Full. **Again** is the median of
the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.6 | 42.4 |
| Noise, then Eyedropper Fill as added (one pixel) | 14.5 | 45.4 |
| Noise, then Eyedropper Fill, radius 200, All | 15.7 | 46.0 |
| Noise, then Eyedropper Fill, radius 1000, Including Alpha, alpha kept | 18.9 | 47.2 |

First loops (empty caches): card 16.6, 18.3, 18.4, 22.7; processor 41.9, 45.5, 45.3, 50.1.

**Reading it.** Eyedropper Fill is cheap: about 0.6 ms a 1080p layer on the card for one pixel,
1 ms for a radius of 200 and 2 ms for a radius of 1000 (14.5, 15.7 and 18.9 against 12.6, over
three layers), and 1 to 1.6 ms on the processor (45.4 to 47.2 against 42.4). The cost grows with
the area sampled, as it should, and stays within Target P1's 1 ms a layer up to a radius of about
200 on the card. The busy run could not be read; this quiet one can.
