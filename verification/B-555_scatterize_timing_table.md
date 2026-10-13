# B-555: frame times with Scatterize (D-676)

Measured on 2026-10-13, built with `cargo test --release --test b555_scatterize` from lane C's
commit b089f9c0, which holds b089f9c0 (B-555's code), and run with `--ignored b555_scatterize_timing`: once on the card, then once
with `B555_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 4.1 GB of its memory in use by another program and these build or test programs were running: b532_warp_stabilizer-b6a2, cargo.exe. Between the card and processor runs, the graphics card had 6.0 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-bee64f3aec, cargo.exe. After the processor run, the graphics card had 6.2 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-bee64f3aec, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Scatterize, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 16.3 | 51.5 |
| Noise, then Scatterize as added | 584.3 | 620.9 |
| Noise, then Scatterize, Amount 300, the right edge edge-on | 615.8 | 808.7 |
| Noise, then Scatterize, Amount keyed 0 to 600, Add | 655.4 | 603.4 |

First loops (empty caches): card 22.9, 591.8, 596.5, 492.3; processor 58.9, 538.7, 848.5, 629.8.

**Reading it.** On the card, Scatterize costs an unknown amount a frame (the machine was busy) over the Noise alone as added
(16.3 to 584.3, three layers); the cost grows with the layer's size (one dot per pixel). Here a busy machine cannot say whether the card is quicker.
