# B-345: frame times with Scatter (D-465)

Measured on 2026-10-11, built with `cargo test --release --test b345_scatter` from 4f0859d9
(B-345's code), and run with `--ignored b345_scatter_timing`: once on the card, then once
with `B345_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 14.0 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-c4cee726c1, cargo.exe. Between the card and processor runs, the graphics card had 11.2 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-c4cee726c1, cargo.exe. After the processor run, the graphics card had 14.1 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-c4cee726c1, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Scatter on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 71.2 | 121.0 |
| Noise, then Scatter as added (amount 10) | 72.3 | 189.5 |
| the same, amount 127, Randomize Every Frame on | 79.7 | 363.0 |

First loops (empty caches): card 132.6, 100.9, 102.7; processor 100.9, 160.5, 210.4.

**Reading it.** On the card, Scatter costs an unknown amount a frame (the machine was busy) over the Noise alone as added
(71.2 to 72.3, three layers), and an unknown amount at amount 127 with a new jumble each
frame (79.7): one warp pass, each pixel read once, whatever the amount.
At these settings a busy machine cannot say whether the card is quicker than the processor here.
