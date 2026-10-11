# B-348: frame times with Channel Threshold (D-468)

Measured on 2026-10-11, built with `cargo test --release --test b348_channel_threshold` from 2414ed7a
(B-348's code), and run with `--ignored b348_channel_threshold_timing`: once on the card, then once
with `B348_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 10.9 GB of its memory in use by another program and these build or test programs were running: anime_compositor_app-7254, b155_gpu_chain-0784c04478, b221_gpu_mix-7be658513acc, cargo.exe. Between the card and processor runs, the graphics card had 9.2 GB of its memory in use by another program and these build or test programs were running: anime_compositor_app-7254, b155_gpu_chain-0784c04478, b221_gpu_mix-7be658513acc, cargo.exe. After the processor run, the graphics card had 9.3 GB of its memory in use by another program and these build or test programs were running: anime_compositor_app-7254, b155_gpu_chain-0784c04478, b221_gpu_mix-7be658513acc, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Channel Threshold on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 29.5 | 123.4 |
| Noise, then Channel Threshold as added (luminance at 128) | 47.0 | 98.7 |
| Noise, then Channel Threshold, RGB at 200 turned over at blend 30 | 31.0 | 73.8 |

First loops (empty caches): card 96.1, 72.2, 56.6; processor 61.2, 89.2, 89.2.

**Reading it.** On the card, Channel Threshold costs an unknown amount a frame (the machine was busy) over the Noise alone in the first setting
(29.5 to 47.0, three layers), and the second setting takes 31.0: one tone pass, each pixel read once, whatever the channel.
At these settings a busy machine cannot say whether the card is quicker than the processor here.
