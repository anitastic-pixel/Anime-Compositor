# B-226: frame times before and after, the last four of P0-2 on the card

Measured on 2026-10-08. No other cargo or rustc process was running: checked before the runs and
after every run, none. The owner's app was not open. The timing test is
`b226_gpu_last_four_timing` in `tests/b226_gpu_last_four.rs`. It was built with `cargo test
--release --test b226_gpu_last_four` on two builds, each in its own folder. They were taken turn
about for three rounds, with the order turned each round (before, after; after, before; before,
after):

- **before**: 72981b9, the commit before B-226, with B-226's test file. A layer with Block
  Dissolve, Gradient Wipe, Line Smoothing or Line Width is drawn by the CPU from that effect on.
- **after**: B-226. The card draws all four.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed**, B-225's way:

- **The shots.** The reference shot, every eighth frame of 240, at Full, with Draw on: GPU. Each
  frame is asked for whole, in milliseconds, the way the viewer asks for it. A Noise that changes
  every frame comes first, then the effect, on the shot's first three layers; the second layer
  has a Drop Shadow before them.
  - Block Dissolve, completion 60, blocks 24 by 8, feather 6.
  - Gradient Wipe with layer-4 as the gradient, stretched, completion 30, softness 25, inverted.
  - Line Smoothing, softness 50, threshold 10.
  - Line Width, 3 pixels thicker, based on the shape.
- **First** is the median of the first loop's 30 frames, started with empty caches. It shows what
  a frame costs the first time it is seen.
- **Again** is the median of the next seven loops' 210 frames.
- **Left to the card** is how many effects the card draws on each of the first three layers at
  frame 100. Line Smoothing and Line Width begin their run (they decide by exact comparisons), so
  the Noise before them stays on the CPU and the card draws one effect on each layer.

Each figure in the table below is the median of the three rounds.

| Shot | Quality | Left to the card, before | Left to the card, after | First, before | First, after | Again, before | Again, after |
|---|---|---|---|---:|---:|---:|---:|
| Noise, then Block Dissolve | Full | 0 / 0 / 0 | 2 / 3 / 2 | 68.4 | 22.9 | 76.6 | 20.5 |
| Noise, then Gradient Wipe | Full | 0 / 0 / 0 | 2 / 3 / 2 | 100.4 | 42.2 | 102.3 | 43.7 |
| Noise, then Line Smoothing | Full | 0 / 0 / 0 | 1 / 1 / 1 | 153.0 | 110.6 | 115.1 | 61.0 |
| Noise, then Line Width | Full | 0 / 0 / 0 | 1 / 1 / 1 | 106.2 | 72.4 | 89.9 | 71.6 |

The three rounds, first / again, in ms:

| Shot | Quality | before 1 | before 2 | before 3 | after 1 | after 2 | after 3 |
|---|---|---|---|---|---|---|---|
| Noise, then Block Dissolve | Full | 68.4 / 76.6 | 73.9 / 77.0 | 67.7 / 76.5 | 22.0 / 20.5 | 22.9 / 20.7 | 22.9 / 20.4 |
| Noise, then Gradient Wipe | Full | 100.4 / 102.5 | 100.4 / 98.7 | 113.3 / 102.3 | 42.9 / 43.7 | 42.1 / 43.7 | 42.2 / 43.9 |
| Noise, then Line Smoothing | Full | 153.0 / 115.5 | 151.9 / 115.1 | 154.3 / 114.8 | 110.6 / 61.2 | 110.2 / 60.8 | 110.9 / 61.0 |
| Noise, then Line Width | Full | 117.4 / 88.9 | 105.7 / 89.9 | 106.2 / 99.8 | 80.9 / 71.6 | 72.2 / 67.1 | 72.4 / 72.5 |

**What it shows.**

- **Block Dissolve**: from 76.6 to 20.5 ms a frame played again, about 3.7 times as fast; 68.4 to
  22.9 ms at first sight. The CPU still chooses the blocks (P0-23) and hands the card a small
  table.
- **Gradient Wipe**: from 102.3 to 43.7 ms played again, about 2.3 times as fast; 100.4 to 42.2
  at first sight. The gradient layer is drawn and handed to the card every frame, as Displacement
  Map's is (B-223); how much of the 43.7 ms that is was not measured here.
- **Line Smoothing**: from 115.1 to 61.0 ms played again, 153.0 to 110.6 at first sight. It begins
  its run, so the Noise before it is still the CPU's, and the CPU encodes the picture for it.
- **Line Width**: gains least, from 89.9 to 71.6 ms played again, 106.2 to 72.4 at first sight.
  It too begins its run, so the Noise before it stays on the CPU.
