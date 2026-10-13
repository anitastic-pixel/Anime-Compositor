# B-553: frame times with Mercury (D-674)

Measured on 2026-10-13, built with `cargo test --release --test b553_mercury` from lane C's
commit be08421f, which holds be08421f (B-553's code), and run with `--ignored b553_mercury_timing`: once on the card, then once
with `B553_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 8.7 GB of its memory in use by another program and these build or test programs were running: anime_compositor_app-466e, b155_gpu_chain-e334744fb6, b532_warp_stabilizer-b6a2, cargo.exe. Between the card and processor runs, the graphics card had 9.5 GB of its memory in use by another program and these build or test programs were running: anime_compositor_app-466e, b155_gpu_chain-e334744fb6, b532_warp_stabilizer-b6a2, cargo.exe. After the processor run, the graphics card had 5.1 GB of its memory in use by another program and these build or test programs were running: b532_warp_stabilizer-b6a2, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Mercury, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 31.8 | 48.8 |
| Noise, then Mercury as added | 36.1 | 70.0 |
| Noise, then Mercury, forty big drops a second running together, a point light | 38.6 | 74.0 |
| Noise, then Mercury, birth rate keyed 0 to 40 (the settings read back to the in point) | 48.7 | 73.8 |

First loops (empty caches): card 42.6, 42.7, 47.6, 42.4; processor 47.8, 69.5, 72.8, 73.0.

**Reading it.** On the card, Mercury costs an unknown amount a frame (the machine was busy) over the Noise alone as added
(31.8 to 36.1, three layers); the cost grows with the number of drops alive (birth rate times longevity), their size, and, when the birth rate is keyed, the frames since the in point. Here a busy machine cannot say whether the card is quicker.
