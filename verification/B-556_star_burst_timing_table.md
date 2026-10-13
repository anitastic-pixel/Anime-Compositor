# B-556: frame times with Star Burst (D-677)

Measured on 2026-10-13, built with `cargo test --release --test b556_star_burst` from lane C's
commit 6f0825ee, which holds 6f0825ee (B-556's code), and run with `--ignored b556_star_burst_timing`: once on the card, then once
with `B556_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, nothing else was running. Between the card and processor runs, these build or test programs were running: b155_gpu_chain-bee64f3aec, cargo.exe. After the processor run, the graphics card had 6.2 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-bee64f3aec, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Star Burst, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 14.1 | 61.5 |
| Noise, then Star Burst as added | 25.8 | 97.0 |
| Noise, then Star Burst, Grid Spacing 0.5 (about half a million stars) | 320.3 | 516.0 |
| Noise, then Star Burst, Speed keyed 0 to 5, Blend w. Original 30 | 91.6 | 102.5 |

First loops (empty caches): card 17.2, 26.2, 312.8, 94.1; processor 50.3, 109.4, 585.1, 124.8.

**Reading it.** On the card, Star Burst costs an unknown amount a frame (the machine was busy) over the Noise alone as added
(14.1 to 25.8, three layers); the cost grows with the grid's star count (the layer's size over Grid Spacing) and the stars' size. Here a busy machine cannot say whether the card is quicker.
