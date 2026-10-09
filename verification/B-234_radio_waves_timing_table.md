# B-234: frame times before and after, Radio Waves on the card

Measured on 2026-10-08. No other cargo or rustc process was running: checked before every run and
after the last, none. The timing test is `b225_gpu_generators_timing` in
`tests/b225_gpu_generators.rs`. It was built with `cargo test --release --test
b225_gpu_generators` on two builds, each in its own folder. They were taken turn about for three
rounds, the order turned in the second:

- **before**: e6b3747, the commit before B-234, with B-234's test file. A layer with Radio Waves
  is drawn by the CPU from that effect on.
- **after**: B-234. The card draws Radio Waves too.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed**, B-225's way: the reference shot, every eighth frame of 240, at Full, with Draw
on: GPU, each frame asked for whole, in milliseconds. A Noise that changes every frame comes first,
then the effect, on the shot's first three layers; the second layer has a Drop Shadow before them.
Radio Waves: producer at (50, 50) per cent, 6 sides, interval 12, expansion 8, spin 1, lifespan 96,
fade-out 48, width 10 to 20, sine profile, white. **First** is the median of the first loop's 30
frames, from empty caches; **again** the median of the next seven loops' 210 frames. **Left to the
card** is how many effects the card draws on each of the first three layers at frame 100. Each
figure below is the median of the three rounds. The other five generators were on the card in both
builds and are shown only as a control.

| Shot | Quality | Left to the card, before | Left to the card, after | First, before | First, after | Again, before | Again, after |
|---|---|---|---|---:|---:|---:|---:|
| Noise, then Radio Waves | Full | 0 / 0 / 0 | 2 / 3 / 2 | 76.2 | 24.0 | 80.5 | 20.5 |
| Noise, then Beam | Full | 2 / 3 / 2 | 2 / 3 / 2 | 19.4 | 19.8 | 16.0 | 15.9 |
| Noise, then 4-Color Gradient | Full | 2 / 3 / 2 | 2 / 3 / 2 | 19.3 | 19.4 | 16.0 | 16.0 |
| Noise, then CC Light Sweep | Full | 2 / 3 / 2 | 2 / 3 / 2 | 17.8 | 17.7 | 14.9 | 14.8 |
| Noise, then Advanced Lightning | Full | 2 / 3 / 2 | 2 / 3 / 2 | 47.9 | 47.9 | 44.9 | 44.8 |
| Noise, then Bevel Edges | Full | 2 / 3 / 2 | 2 / 3 / 2 | 17.6 | 18.1 | 14.5 | 14.5 |

The three rounds for Radio Waves, first / again, in ms:

| Shot | before 1 | before 2 | before 3 | after 1 | after 2 | after 3 |
|---|---|---|---|---|---|---|
| Noise, then Radio Waves | 75.7 / 80.5 | 76.2 / 79.1 | 80.9 / 82.6 | 24.1 / 20.6 | 24.0 / 20.5 | 23.9 / 20.4 |

**What it shows.** Radio Waves plays about 4 times as fast played again, from 80.5 to 20.5 ms a
frame, and about 3 times as fast at first sight, from 76.2 to 24.0 ms. Its waves are still worked
out by the CPU every frame and handed to the card (P0-23), as Advanced Lightning's segments are; on
this shot that costs little, about 5 ms over Beam. The five generators already on the card are
unchanged, within 1 ms.
