# B-343: frame times with Plastic (D-463)

Measured on 2026-10-11, built with `cargo test --release --test b343_plastic` from 3e880fc1
(B-343's code), and run with `--ignored b343_plastic_timing`: once on the card, then once
with `B343_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 6.1 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-c4cee726c1, cargo.exe. Between the card and processor runs, the graphics card had 6.4 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-c4cee726c1, cargo.exe. After the processor run, the graphics card had 6.1 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-c4cee726c1, b406_numbers-2fdba1cdda51, cargo.exe, rustc.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Plastic, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 15.7 | 57.4 |
| Noise, then Plastic as added | 21.5 | 138.4 |
| the same, Luminance, softness 30, a point light, metal 100 | 25.6 | 175.2 |

First loops (empty caches): card 17.5, 31.4, 28.4; processor 57.5, 128.4, 175.4.

**Reading it.** On the card, Plastic costs an unknown amount a frame (the machine was busy) over the Noise alone as added
(15.7 to 21.5, three layers, softness 10), and an unknown amount at Luminance, softness 30,
a point light, metal 100 (25.6): the channel read, the Gaussian's two passes, then one
lighting pass. At these settings a busy machine cannot say whether the card is quicker than the processor here.
