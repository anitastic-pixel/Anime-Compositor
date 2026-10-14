# B-329: frame times with Block Load (D-449)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b329_block_load --no-run` and run with `--ignored b329_block_load_timing`: once on the card, then once with `B329_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 2% busy; 4.3 GB of its memory held by the desktop and open programs); the same after each. The processor was 4% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 53.9, processor 117.1; Noise, then Block Load completion 50 card 60.4, processor 216.2 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Block Load, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 13.0 | 42.0 |
| Noise, then Block Load completion 50 | 15.1 | 63.6 |
| Noise, then Block Load completion 50, scans 8, bilinear (two blended pictures, blocks 16 and 32 wide) | 20.0 | 92.1 |

First loops (empty caches): card 17.1, 19.1, 23.4; processor 41.7, 62.8, 92.8.

**Reading it.** On the card, Block Load at completion 50 costs under 1 ms a 1080p layer (13.0 to
15.1, three layers), and with scans 8 and Bilinear (two blended pictures) about 2.3 ms (20.0). It is
drawn by Mosaic's `tiles` pass for each of the two pictures (blended in its bilinear mode when
Bilinear is on) and one pass of its own, `blockload`, that lays the finer picture over the coarser
down to the row the pass has reached. On the processor the same shots cost about 7 and 17 ms a
layer (63.6 and 92.1 against 42.0), so the card is about seven to ten times quicker, and the plain
case is within Target P1's 1 ms a layer on the card. The busy run could not say this.
