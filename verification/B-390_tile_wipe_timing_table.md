# B-390: frame times with Tile Wipe (D-510) — PROVISIONAL

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b390_tile_wipe --no-run` and run with `--ignored b390_tile_wipe_timing`: once on the card, then once with `B390_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (2 and 7% busy; 4.3 GB of its memory held by the desktop and open programs); the same after each. The processor was 2% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 17.9, processor 57.4; Noise, then Tile Wipe, completion 50 (as added otherwise) card 24.6, processor 63.8 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Tile Wipe,
every eighth frame asked for whole at Full. **Again** is the median of the loops after the first
(seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 41.8 |
| Noise, then Tile Wipe, completion 50 (as added otherwise) | 15.4 | 48.2 |
| Noise, then Tile Wipe, completion 50, 40 tiles, border 4, turned 30 | 15.6 | 48.4 |
| Noise, then Tile Wipe, completion 50, rectangle, reversed | 15.1 | 48.1 |

First loops (empty caches): card 15.7, 19.6, 19.0, 18.3; processor 41.7, 47.3, 48.3, 47.9.

**Reading it.** Every shot draws to the end on both, and the card ran each Tile Wipe shot about
three times as fast as the processor (15.1 to 15.6 against 48.1 to 48.4 a frame). Tile Wipe's own
share is about 1 ms a 1080p layer on the card (2.8 to 3.3 ms a frame over the Noise alone, three
layers), within Target P1's 1 ms or just at it, and about 2 ms a layer on the processor. The busy
run's 5 to 10 ms a frame on the card were the other compiles.
