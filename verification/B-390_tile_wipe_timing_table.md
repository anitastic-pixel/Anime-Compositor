# B-390: frame times with Tile Wipe (D-510) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test
b390_tile_wipe` from c46d6557 (B-390's code), and run with `--ignored
b390_tile_wipe_timing`: once on the card, then once with `B390_CPU` set on the processor.
**The machine was not quiet.** Other lanes' cargo and rustc processes were running before,
during and after both runs (four cargo builds and two rustc compiles were running before the
first run; two cargo builds were still running when the second run ended). The numbers below
are recorded as they came out. They are not a clean measurement of Tile Wipe: the card's Noise
alone took 17.9 ms a frame here, against 11.8 ms on the quiet machine in B-387's table. A quiet
re-measure is owed.

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
| Noise alone | 17.9 | 57.4 |
| Noise, then Tile Wipe, completion 50 (as added otherwise) | 24.6 | 63.8 |
| Noise, then Tile Wipe, completion 50, 40 tiles, border 4, turned 30 | 22.6 | 71.9 |
| Noise, then Tile Wipe, completion 50, rectangle, reversed | 28.1 | 119.8 |

First loops (empty caches): card 21.6, 32.7, 27.1, 25.8; processor 80.9, 64.1, 71.1, 145.5.

**Reading it.** Every shot draws to the end on both, and the card ran each Tile Wipe shot
between about two and a half and four times as fast as the processor. Tile Wipe's own share on
the card (about 5 to 10 ms a frame over the Noise alone) cannot be trusted while other compiles
share the machine. Re-run the two commands above when no other cargo, rustc or test process is
running and the card is idle, and replace this table.
