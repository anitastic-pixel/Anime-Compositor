# B-340: frame times with Burn Film (D-460)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b340_burn_film --no-run` and run with `--ignored b340_burn_film_timing`: once on the card, then once with `B340_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 4.2 GB of its memory held by the desktop and open programs); the same after each. The processor was 3% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 76.7, processor 59.5; Noise, then Burn Film burn 50 card 166.9, processor 213.2 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Burn Film, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 41.7 |
| Noise, then Burn Film burn 50 | 31.2 | 134.6 |

First loops (empty caches): card 15.9, 34.8; processor 41.3, 135.1.

**Reading it.** On the card, Burn Film at burn 50 costs about 6.3 ms a 1080p layer (12.3 to 31.2,
three layers), drawn by one pass of its own, `burnfilm`, that works each pixel's burn time (the
nearest burn point and six octaves of fractal noise) in double precision. On the processor it
costs about 31 ms a layer (134.6 against 41.7), so the card is about five times quicker. The busy
run could not say this.
