# B-341: frame times with Hex Tile (D-461)

Measured on 2026-10-11, built with `cargo test --release --test b341_hex_tile` from 2b01e006
(B-341's code), and run with `--ignored b341_hex_tile_timing`: once on the card, then once
with `B341_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 7.8 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, b155_gpu_chain-c4cee726c1, cargo.exe, rustc.exe. Between the card and processor runs, the graphics card had 8.6 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, b155_gpu_chain-c4cee726c1, cargo.exe. After the processor run, the graphics card had 10.0 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, b155_gpu_chain-c4cee726c1, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

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
| Noise alone | 69.3 | 97.0 |
| Noise, then Hex Tile radius 50 | 57.6 | 156.0 |
| the same, Fold Seamlessly, rotate 20, smearing 30 | 61.0 | 129.4 |

First loops (empty caches): card 68.3, 78.2, 69.9; processor 86.7, 159.3, 116.8.

**Reading it.** On the card, Hex Tile costs an unknown amount a frame (the machine was busy) over the Noise alone at radius 50
(69.3 to 57.6, three layers), and an unknown amount with Fold Seamlessly, a turn and smearing
(61.0), drawn by the card's warp pass (mode 100), one bilinear read a pixel.
At these settings a busy machine cannot say whether the card is quicker than the processor here.
