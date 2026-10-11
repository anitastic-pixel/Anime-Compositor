# B-393: frame times with Glow Wipe (D-513) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b393_glow_wipe` from
4d0cc1e4 (B-393's code), and run with `--ignored b393_glow_wipe_timing`: once on the card,
then once with `B393_CPU` set on the processor.
**The machine was not quiet.** Four other cargo processes (other lanes' tests and builds) and a
compiler were running before, during and after both runs. The numbers below are recorded as they
came out. They are not a clean measurement of Glow Wipe: the card's Noise alone took 97.5 ms a
frame here, against 11.8 ms on the quiet machine in B-387's table, and the processor's 125.7 ms
against 57.4 ms in B-390's busy table. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Glow Wipe,
every eighth frame asked for whole at Full. **Again** is the median of the loops after the first
(seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 97.5 | 125.7 |
| Noise, then Glow Wipe, completion 50 (as added otherwise) | 139.5 | 156.3 |
| Noise, then Glow Wipe, completion 50, doors, direction 30, Color from Source | 94.7 | 202.0 |
| Noise, then Glow Wipe, completion 50, rectangular, reversed, #ff8000 at 300 | 50.5 | 295.8 |

First loops (empty caches): card 187.7, 146.9, 145.8, 79.2; processor 109.7, 167.3, 165.2, 354.9.

**Reading it.** Every shot draws to the end on both. On the card the busiest settings came out
faster than the Noise alone, which shows how much the other tests' use of the card moved these
numbers; the wipe is one pass of the existing `wipe` shader, no blur. Re-run the two commands
above when no other cargo, rustc or test process is running and the card is idle, and replace
this table.
