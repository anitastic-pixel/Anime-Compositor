# B-225: frame times before and after, five generators on the card

Measured on 2026-10-08. No other cargo or rustc process was running: checked before the runs and
after every run, none. The timing test is `b225_gpu_generators_timing` in
`tests/b225_gpu_generators.rs`. It was built with `cargo test --release --test
b225_gpu_generators` on two builds, each in its own folder. They were taken turn about for three
rounds, with the order turned each round:

- **before**: a09564a, the commit before B-225, with B-225's test file. A layer with Beam,
  4-Color Gradient, CC Light Sweep, Advanced Lightning or Bevel Edges is drawn by the CPU from that
  effect on.
- **after**: B-225. The card draws all five. Radio Waves stays on the CPU (D-345) and is not
  timed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed**, B-224's way:

- **The shots.** The reference shot, every eighth frame of 240, at Full, with Draw on: GPU. Each
  frame is asked for whole, in milliseconds, the way the viewer asks for it. A Noise that changes
  every frame comes first, then the effect, on the shot's first three layers; the second layer
  has a Drop Shadow before them.
  - Beam from (20, 30) to (80, 70) per cent, length 40, time 50, thickness 6 to 30, softness 40.
  - 4-Color Gradient, four points and colours, blend 100, opacity 60, Screen.
  - CC Light Sweep, centre (40, 50), direction -30, width 80, sweep 50, edge 100, edge
    thickness 6, smooth, Add.
  - Advanced Lightning from (20, 10) to (80, 90) per cent, Strike, detail 6, branches 40, width 3,
    glow 20, seed 7.
  - Bevel Edges, thickness 0.1, light from -60 degrees, intensity 0.4.
- **First** is the median of the first loop's 30 frames, started with empty caches. It shows what
  a frame costs the first time it is seen.
- **Again** is the median of the next seven loops' 210 frames.
- **Left to the card** is how many effects the card draws on each of the first three layers at
  frame 100.

Each figure in the table below is the median of the three rounds.

| Shot | Quality | Left to the card, before | Left to the card, after | First, before | First, after | Again, before | Again, after |
|---|---|---|---|---:|---:|---:|---:|
| Noise, then Beam | Full | 0 / 0 / 0 | 2 / 3 / 2 | 67.0 | 19.3 | 74.9 | 16.1 |
| Noise, then 4-Color Gradient | Full | 0 / 0 / 0 | 2 / 3 / 2 | 92.1 | 19.9 | 77.0 | 16.4 |
| Noise, then CC Light Sweep | Full | 0 / 0 / 0 | 2 / 3 / 2 | 100.7 | 18.3 | 92.8 | 15.6 |
| Noise, then Advanced Lightning | Full | 0 / 0 / 0 | 2 / 3 / 2 | 87.7 | 48.5 | 72.7 | 46.2 |
| Noise, then Bevel Edges | Full | 0 / 0 / 0 | 2 / 3 / 2 | 85.7 | 18.0 | 70.2 | 15.0 |

The three rounds, first / again, in ms:

| Shot | Quality | before 1 | before 2 | before 3 | after 1 | after 2 | after 3 |
|---|---|---|---|---|---|---|---|
| Noise, then Beam | Full | 70.5 / 74.9 | 66.7 / 74.1 | 67.0 / 75.0 | 19.1 / 16.1 | 20.0 / 16.1 | 19.3 / 16.3 |
| Noise, then 4-Color Gradient | Full | 91.4 / 76.4 | 105.3 / 82.3 | 92.1 / 77.0 | 19.9 / 16.4 | 19.1 / 16.4 | 21.8 / 18.3 |
| Noise, then CC Light Sweep | Full | 100.7 / 94.2 | 100.3 / 84.4 | 102.7 / 92.8 | 17.9 / 15.6 | 18.3 / 15.2 | 19.9 / 17.1 |
| Noise, then Advanced Lightning | Full | 93.8 / 73.1 | 87.1 / 72.7 | 87.7 / 72.7 | 48.5 / 47.4 | 48.2 / 45.6 | 48.8 / 46.2 |
| Noise, then Bevel Edges | Full | 84.7 / 69.9 | 94.6 / 76.0 | 85.7 / 70.2 | 19.1 / 16.1 | 18.0 / 14.9 | 17.9 / 15.0 |

**What it shows.**

- **Played again**, about 4.5 to 6 times as fast for four of the five:
  - Beam: from 74.9 to 16.1 ms a frame.
  - 4-Color Gradient: from 77.0 to 16.4 ms.
  - CC Light Sweep: from 92.8 to 15.6 ms.
  - Bevel Edges: from 70.2 to 15.0 ms.
  - First sight falls the same way (67.0 to 19.3, 92.1 to 19.9, 100.7 to 18.3, 85.7 to 18.0 ms).
- **Advanced Lightning gains less**: from 72.7 to 46.2 ms played again, 87.7 to 48.5 at first
  sight. The bolt's segments are still worked out by the CPU every frame (so its random choices
  stay the CPU's, P0-23) and the card tests every pixel against every segment's box. Which of the
  two costs the 30 ms over the others was not measured here.
