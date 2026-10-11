# B-329: frame times with Block Load (D-449)

Measured on 2026-10-11, built with `cargo test --release --test b329_block_load` from 2c245553
(B-329's code), and run with `--ignored b329_block_load_timing`: once on the card, then once
with `B329_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 13.6 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, b155_gpu_chain-c4cee726c1, cargo.exe. Between the card and processor runs, the graphics card had 13.3 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, b155_gpu_chain-c4cee726c1, cargo.exe. After the processor run, the graphics card had 14.0 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, b155_gpu_chain-c4cee726c1, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

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
| Noise alone | 53.9 | 117.1 |
| Noise, then Block Load completion 50 | 60.4 | 216.2 |
| Noise, then Block Load completion 50, scans 8, bilinear (two blended pictures, blocks 16 and 32 wide) | 79.6 | 301.1 |

First loops (empty caches): card 113.1, 73.4, 101.9; processor 111.5, 196.8, 313.1.

**Reading it.** On the card, Block Load costs an unknown amount a frame (the machine was busy) over the Noise alone at completion 50
(53.9 to 60.4, three layers), drawn by Mosaic's `tiles` pass for each of the two
pictures (blended in its bilinear mode when Bilinear is on) and one pass of its own, `blockload`,
that lays the finer picture over the coarser down to the row the pass has reached. At these
settings a busy machine cannot say whether the card is quicker than the processor here.
