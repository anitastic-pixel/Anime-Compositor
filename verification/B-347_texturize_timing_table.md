# B-347: frame times with Texturize (D-467)

Measured on 2026-10-11, built with `cargo test --release --test b347_texturize` from 30eb56f2
(B-347's code), and run with `--ignored b347_texturize_timing`: once on the card, then once
with `B347_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 11.4 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, b155_gpu_chain-c4cee726c1, cargo.exe. Between the card and processor runs, the graphics card had 12.7 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, b155_gpu_chain-c4cee726c1, b397_twister-dd8d63ce6b2c, cargo.exe. After the processor run, the graphics card had 13.7 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, b155_gpu_chain-c4cee726c1, b397_twister-dd8d63ce6b2c, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Texturize with the fourth layer as the texture on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 32.1 | 124.0 |
| Noise, then Texturize with layer4 tiled as the texture | 122.7 | 360.6 |
| the same, stretched, Light Direction 45, Texture Contrast 2 | 144.4 | 421.2 |

First loops (empty caches): card 50.1, 148.1, 131.1; processor 123.0, 374.5, 410.5.

**Reading it.** On the card, Texturize costs an unknown amount a frame (the machine was busy) over the Noise alone in the first setting
(32.1 to 122.7, three layers), and the second setting takes 144.4: two passes, the texture's luminance then the relief, each pixel read a few times, whatever the placement.
At these settings a busy machine cannot say whether the card is quicker than the processor here.
