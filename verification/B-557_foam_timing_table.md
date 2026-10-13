# B-557: frame times with Foam (D-678)

Measured on 2026-10-13, built with `cargo test --release --test b557_foam` from lane C's
commit dd9c6fce, which holds dd9c6fce (B-557's code), and run with `--ignored b557_foam_timing`: once on the card, then once
with `B557_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 9.8 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-bee64f3aec, b532_warp_stabilizer-b6a2, cargo.exe. Between the card and processor runs, the graphics card had 9.7 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-bee64f3aec, b446_turbulent_noise_type, b532_warp_stabilizer-b6a2, cargo.exe. After the processor run, the graphics card had 10.1 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-bee64f3aec, b532_warp_stabilizer-b6a2, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Foam, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 34.1 | 70.8 |
| Noise, then Foam as added (the Draft view) | 102.8 | 169.1 |
| Noise, then Foam, Rendered, bigger bubbles, the Layer texture with a reflection | 66.2 | 193.3 |
| Noise, then Foam, Rendered, production rate keyed 0 to 2 (the settings read back to the in point) | 64.5 | 151.6 |

First loops (empty caches): card 34.1, 116.0, 81.3, 74.9; processor 63.3, 150.3, 205.7, 137.2.

**Reading it.** On the card, Foam costs an unknown amount a frame (the machine was busy) over the Noise alone as added
(34.1 to 102.8, three layers); the cost grows with the number of bubbles alive (production rate and lifespan) and their size. Here a busy machine cannot say whether the card is quicker.
