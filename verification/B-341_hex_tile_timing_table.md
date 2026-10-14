# B-341: frame times with Hex Tile (D-461)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b341_hex_tile --no-run` and run with `--ignored b341_hex_tile_timing`: once on the card, then once with `B341_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (2 and 3% busy; 4.2 GB of its memory held by the desktop and open programs); the same after each. The processor was 1% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 69.3, processor 97.0; Noise, then Hex Tile radius 50 card 57.6, processor 156.0 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Hex Tile, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.4 | 41.9 |
| Noise, then Hex Tile radius 50 | 17.0 | 59.4 |
| the same, Fold Seamlessly, rotate 20, smearing 30 | 17.3 | 62.2 |

First loops (empty caches): card 15.6, 20.9, 20.4; processor 41.1, 58.6, 61.3.

**Reading it.** On the card, Hex Tile at radius 50 costs about 1.5 ms a 1080p layer (12.4 to 17.0,
three layers), and the same with Fold Seamlessly, a turn and smearing (17.3), drawn by the card's
warp pass (mode 100), one bilinear read a pixel. On the processor it costs about 6 to 7 ms a layer
(59.4 and 62.2 against 41.9), so the card is about four times quicker. The busy run could not say
this.
