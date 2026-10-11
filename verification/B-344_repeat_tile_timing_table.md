# B-344: frame times with Repeat Tile (D-464)

Measured on 2026-10-11, built with `cargo test --release --test b344_repeat_tile` from a435eef1
(B-344's code), and run with `--ignored b344_repeat_tile_timing`: once on the card, then once
with `B344_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 10.3 GB of its memory in use by another program and these build or test programs were running: anime_compositor_app-7254, b155_gpu_chain-c4cee726c1, b407_timecode-8caf3dc6b3c, cargo.exe. Between the card and processor runs, the graphics card had 9.6 GB of its memory in use by another program and these build or test programs were running: anime_compositor_app-7254, b155_gpu_chain-c4cee726c1, cargo.exe. After the processor run, the graphics card had 11.8 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-c4cee726c1, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

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
| Noise alone | 43.8 | 93.4 |
| Noise, then Repeat Tile, Unfold grown 200 each way | 55.7 | 154.6 |
| the same, Brick with Blend Borders 50 | 50.5 | 144.4 |

First loops (empty caches): card 57.2, 60.0, 69.9; processor 84.0, 137.6, 164.7.

**Reading it.** On the card, Repeat Tile costs an unknown amount a frame (the machine was busy) over the Noise alone, Unfold grown
200 each way (43.8 to 55.7, three layers), and an unknown amount as Brick with Blend
Borders 50 (50.5): one warp pass into the grown layer, and the larger layers below it.
At these settings a busy machine cannot say whether the card is quicker than the processor here.
