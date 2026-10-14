# B-344: frame times with Repeat Tile (D-464)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b344_repeat_tile --no-run` and run with `--ignored b344_repeat_tile_timing`: once on the card, then once with `B344_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 2% busy; 4.2 GB of its memory held by the desktop and open programs); the same after each. The processor was 2% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 43.8, processor 93.4; Noise, then Repeat Tile, Unfold grown 200 each way card 55.7, processor 154.6 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Repeat Tile growing each layer 200 pixels each way, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.6 | 41.7 |
| Noise, then Repeat Tile, Unfold grown 200 each way | 14.6 | 61.2 |
| the same, Brick with Blend Borders 50 | 16.6 | 64.2 |

First loops (empty caches): card 16.3, 17.8, 20.3; processor 41.5, 60.4, 63.7.

**Reading it.** On the card, Repeat Tile, Unfold grown 200 each way, costs under 1 ms a 1080p layer
(12.6 to 14.6, three layers), and about 1.3 ms as Brick with Blend Borders 50 (16.6): one warp
pass into the grown layer, and the larger layers below it. On the processor it costs about 6.5 to
7.5 ms a layer (61.2 and 64.2 against 41.7), so the card is about six to nine times quicker. The
busy run could not say this.
