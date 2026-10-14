# B-333: frame times with Dust & Scratches (D-453)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b333_dust_scratches --no-run` and run with `--ignored b333_dust_scratches_timing`: once on the card, then once with `B333_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 4% busy; 4.2 GB to 4.3 GB of its memory held by the desktop and open programs); the same after each. The processor was 3% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 201.1, processor 49.9; as added (Radius 1, Threshold 0) card 656.9, processor 92.9 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Dust & Scratches on each, every eighth frame asked for whole at Full. **Again** is the median
of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 41.3 |
| as added (Radius 1, Threshold 0) | 79.6 | 82.1 |
| Radius 3, Threshold 20 | 98.3 | 135.9 |
| Radius 5, Threshold 8, Operate on Alpha on | 232.8 | 230.8 |

First loops (empty caches): card 16.8, 82.3, 104.2, 235.5; processor 40.6, 81.0, 136.0, 229.9.

**Reading it.** Over Target P2 (4 ms or less a layer) at every setting measured. On the card three
layers add 67 to 220 ms a frame over Noise alone (79.6 to 232.8 against 12.3): about 22 ms a layer
as added, 29 at Radius 3 and 74 at Radius 5. On the processor about 14, 32 and 63 ms a layer (82.1
to 230.8 against 41.3). So the card is no quicker than the processor here, and slower as added.
The busy run's card figures measured the other tests sharing the card; these can be read.
