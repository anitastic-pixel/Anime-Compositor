# B-450: frame times with Color Grade's third unit (D-571)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b450_color_grade_3 --no-run` and run with `--ignored b450_color_grade_3_timing`: once on the card, then once with `B450_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 5% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-12, on a machine that was not quiet), again: Noise alone card 107.4, processor 111.0; Noise, then Color Grade: the four new curves and three secondary wheels card 136.4, processor 259.9 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Color Grade on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 42.4 |
| Noise, then Color Grade: the four new curves and three secondary wheels | 34.5 | 105.3 |
| Noise, then Color Grade: an HSL secondary, its key denoised 12 and blurred 8 (drawn by the processor either way) | 129.4 | 141.9 |

First loops (empty caches): card 15.6, 38.5, 124.9; processor 42.0, 106.4, 141.5.

**Reading it.** The four curves and the three wheels add about 22 ms a frame on the card over the Noise alone (12.3 to 34.5, three layers), about 7.4 ms a 1080p layer, against about 21 ms a layer on the processor (42.4 to 105.3). A key with Denoise or Blur is worked out by the processor (D-571): on the processor's own path the whole secondary with its refined key adds about 33 ms a layer (141.9); on the card's path, with the hand-over to the processor and back for each such layer, the frame takes 129.4 ms, a little quicker than the processor's path. This changes the busy run's reading, where the card's path came out at 737.1 ms against 348.6 and looked slower than having the card off: on a quiet machine it is not. Teaching the card the key's median and blur (the `ponytail:` note in `src/compose.rs`) would still save most of that 129.4, but nothing here makes it urgent.
