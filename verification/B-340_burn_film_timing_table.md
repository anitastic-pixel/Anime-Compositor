# B-340: frame times with Burn Film (D-460)

Measured on 2026-10-11, built with `cargo test --release --test b340_burn_film` from c15736fc
(B-340's code), and run with `--ignored b340_burn_film_timing`: once on the card, then once
with `B340_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 15.2 GB of its memory in use by another program and these build or test programs were running: b420_screen_key-34fcb24a2, cargo.exe, rustc.exe. Between the card and processor runs, the graphics card had 7.2 GB of its memory in use by another program and these build or test programs were running: anime_compositor_app-5586, b420_screen_key-34fcb24a2, cargo.exe, rustc.exe. After the processor run, the graphics card had 14.2 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, b389_glass_melt-605181782, b420_screen_key-34fcb24a2, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Burn Film, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 76.7 | 59.5 |
| Noise, then Burn Film burn 50 | 166.9 | 213.2 |

First loops (empty caches): card 284.5, 93.3; processor 57.4, 176.9.

**Reading it.** On the card, Burn Film costs an unknown amount a frame (the machine was busy) over the Noise alone at burn 50
(76.7 to 166.9, three layers), drawn by one pass of its own, `burnfilm`, that works
each pixel's burn time (the nearest burn point and six octaves of fractal noise) in double
precision. At this setting a busy machine cannot say whether the card is quicker than the processor here.
