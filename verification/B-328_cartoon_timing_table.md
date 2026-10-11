# B-328: frame times with Cartoon (D-448)

Measured on 2026-10-10, built with `cargo test --release --test b328_cartoon` from bdf258e4
(B-328's code), and run with `--ignored b328_cartoon_timing`: once on the card, then once
with `B328_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, nothing else was running. Between the card and processor runs, nothing else was running. After the processor run, the graphics card had 10.7 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Cartoon, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.6 | 44.8 |
| Noise, then Cartoon as added | 48.1 | 665.1 |
| Noise, then Cartoon detail radius 30, edge width 8, enhancement 50 (the widest reach) | 323.0 | 6731.9 |

First loops (empty caches): card 17.9, 51.8, 301.5; processor 44.5, 589.5, 6192.6.

**Reading it.** On the card, Cartoon costs an unknown amount a frame (the machine was busy) over the Noise alone as added
(12.6 to 48.1, three layers), drawn by Bilateral Blur's two passes and two of its
own, `cartoonedge` and `cartoon`; the cost grows with Detail Radius (the smoothing) and Edge
Width (the line). At these settings a busy machine cannot say whether the card is quicker than the processor here.
