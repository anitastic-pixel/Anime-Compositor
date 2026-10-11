# B-327: frame times with Brush Strokes (D-447)

Measured on 2026-10-10, built with `cargo test --release --test b327_brush_strokes` from afa19012
(B-327's code, rebased onto Remove Grain as c8bf38f5), and run with `--ignored b327_brush_strokes_timing`: once on the card, then once
with `B327_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 15.0 GB of its memory in use by another program and these build or test programs were running: b335_remove_grain-b5afb87, cargo.exe. Between the card and processor runs, the graphics card had 11.3 GB of its memory in use by another program and these build or test programs were running: cargo.exe, rustc.exe. After the processor run, the graphics card had 14.3 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Brush Strokes, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 250.9 | 61.7 |
| Noise, then Brush Strokes as added | 498.1 | 222.9 |
| Noise, then Brush Strokes brush 6, length 30, density 4, randomness 2 (many strokes a pixel) | 842.5 | 1048.9 |

First loops (empty caches): card 768.3, 617.2, 492.2; processor 45.4, 236.8, 1040.0.

**Reading it.** On the card, Brush Strokes costs an unknown amount a frame (the machine was busy) over the Noise alone as
added (250.9 to 498.1, three layers), drawn by one pass of its own, `brush`, each
pixel visiting the cells whose strokes can reach it, so the cost grows with Brush Size, Stroke
Length and Stroke Density. At these settings a busy machine cannot say whether the card is quicker than the processor here.
