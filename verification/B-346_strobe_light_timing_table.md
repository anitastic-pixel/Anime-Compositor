# B-346: frame times with Strobe Light (D-466)

Measured on 2026-10-11, built with `cargo test --release --test b346_strobe_light` from e9b9340c
(B-346's code), and run with `--ignored b346_strobe_light_timing`: once on the card, then once
with `B346_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 11.5 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-c4cee726c1, cargo.exe. Between the card and processor runs, the graphics card had 11.3 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-c4cee726c1, cargo.exe. After the processor run, the graphics card had 11.4 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-c4cee726c1, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Strobe Light flashing all the time (Strobe Period 0.01) on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 61.6 | 98.7 |
| Noise, then Strobe Light always on (Copy, Strobe Period 0.01) | 64.4 | 136.0 |
| the same, Xor with Blend With Original 30 | 75.3 | 152.5 |

First loops (empty caches): card 94.9, 60.3, 68.5; processor 106.8, 143.3, 186.4.

**Reading it.** On the card, Strobe Light costs an unknown amount a frame (the machine was busy) over the Noise alone in the first setting
(61.6 to 64.4, three layers), and the second setting takes 75.3: one warp pass while a flash is on, each pixel read once, and nothing at all between flashes.
At these settings a busy machine cannot say whether the card is quicker than the processor here.
