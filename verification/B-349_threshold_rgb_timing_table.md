# B-349: frame times with Threshold RGB (D-469)

Measured on 2026-10-11, built with `cargo test --release --test b349_threshold_rgb` from 8ee864d4
(B-349's code), and run with `--ignored b349_threshold_rgb_timing`: once on the card, then once
with `B349_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 10.1 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-c4cee726c1, b221_gpu_mix-7be658513acc, cargo.exe. Between the card and processor runs, the graphics card had 10.2 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-c4cee726c1, b221_gpu_mix-7be658513acc, cargo.exe. After the processor run, the graphics card had 6.6 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-c4cee726c1, b221_gpu_mix-7be658513acc, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Threshold RGB on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 48.1 | 94.6 |
| Noise, then Threshold RGB as added (every level 128) | 61.8 | 192.9 |
| Noise, then Threshold RGB, red 200, green 90 turned over, blue 40, blend 30 | 23.8 | 178.6 |

First loops (empty caches): card 92.7, 81.8, 38.4; processor 81.5, 206.7, 202.7.

**Reading it.** On the card, Threshold RGB costs an unknown amount a frame (the machine was busy) over the Noise alone in the first setting
(48.1 to 61.8, three layers), and the second setting takes 23.8: one tone pass, each pixel read once, whatever the levels.
At these settings a busy machine cannot say whether the card is quicker than the processor here.
