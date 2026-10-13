# B-552: frame times with Hair (D-673)

Measured on 2026-10-13, built with `cargo test --release --test b552_hair` from lane C's
commit a96fb7b5, which holds a96fb7b5 (B-552's code), and run with `--ignored b552_hair_timing`: once on the card, then once
with `B552_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 14.6 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, b155_gpu_chain-e334744fb6, b530_pixel_motion_blur-70, cargo.exe, rustc.exe. Between the card and processor runs, the graphics card had 6.3 GB of its memory in use by another program and these build or test programs were running: anime_compositor_app-7f89, b531_timewarp-f9686d1f554, cargo.exe, rustc.exe. After the processor run, the graphics card had 7.4 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-e334744fb6, b531_timewarp-f9686d1f554, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Hair, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 53.7 | 54.1 |
| Noise, then Hair as added | 290.7 | 518.2 |
| Noise, then Hair, a long fur: length 200, thickness 6, density 60 | 3100.4 | 4496.6 |
| Noise, then Hair, weight keyed -100 to 200, constant mass | 336.5 | 523.0 |

First loops (empty caches): card 63.1, 311.1, 3286.2, 466.0; processor 51.9, 307.6, 3740.9, 944.6.

**Reading it.** On the card, Hair costs an unknown amount a frame (the machine was busy) over the Noise alone as added
(53.7 to 290.7, three layers); the cost grows with the number of strands, their length and their thickness. Here a busy machine cannot say whether the card is quicker.
