# B-221: frame times before and after, an effect at Mix 50

Measured on 2026-10-08 with no other cargo or rustc process running (checked before every run:
none). `b221_gpu_mix_timing` in `tests/b221_gpu_mix.rs`, built with
`cargo test --release --test b221_gpu_mix --no-run` on two builds, each in its own folder with its
own build, taken turn about, the order turned each round, three rounds:

- **before**: c79cfed, the commit before B-221, with B-221's test file: a layer with an effect
  mixed below 100 is drawn by the CPU from that effect on;
- **after**: B-221, the card drawing the mixed effect and laying it back by its Mix.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed**, B-151's way. The reference shot with the effect at Mix 50 on its first three
layers (the second after a Drop Shadow), every eighth frame of 240, each asked for as the viewer
asks for a frame, whole, in milliseconds, with Draw on: GPU. **First** is the median of the first
loop's 30 frames, started with empty caches: what a frame costs the first time it is seen.
**Again** is the median of the next seven loops' 210 frames. A Glow holds still, so played again
it is already done either way; a Noise changes every frame, so "again" shows the work each frame
costs. "Left to the card" is how many effects the card draws on the first three layers at frame
100. Each figure is the median of the three rounds.

| Shot | Quality | Left to the card, before | Left to the card, after | First, before | First, after | Again, before | Again, after |
|---|---|---|---|---:|---:|---:|---:|
| Glow at Mix 50 | Draft | 0 / 0 / 0 | 1 / 1 / 1 | 13.3 | 13.6 | 11.1 | 11.4 |
| Glow at Mix 50 | Full | 0 / 0 / 0 | 1 / 1 / 1 | 23.8 | 14.3 | 11.5 | 11.9 |
| Noise at Mix 50 | Draft | 0 / 0 / 0 | 1 / 2 / 1 | 25.4 | 21.0 | 14.2 | 13.6 |
| Noise at Mix 50 | Full | 0 / 0 / 0 | 1 / 2 / 1 | 108.5 | 17.6 | 92.2 | 14.2 |

The three rounds, first / again, in ms:

| Shot | Quality | before 1 | before 2 | before 3 | after 1 | after 2 | after 3 |
|---|---|---|---|---|---|---|---|
| Glow at Mix 50 | Draft | 14.3 / 11.3 | 12.9 / 10.8 | 13.3 / 11.1 | 13.6 / 11.8 | 12.9 / 11.2 | 13.6 / 11.4 |
| Glow at Mix 50 | Full | 25.8 / 11.6 | 23.3 / 11.5 | 23.8 / 11.5 | 14.3 / 12.0 | 14.2 / 11.5 | 14.5 / 11.9 |
| Noise at Mix 50 | Draft | 27.9 / 14.4 | 25.4 / 14.2 | 20.8 / 12.8 | 21.9 / 13.6 | 17.3 / 12.9 | 21.0 / 13.7 |
| Noise at Mix 50 | Full | 108.5 / 92.4 | 107.8 / 91.9 | 109.8 / 92.2 | 17.6 / 14.2 | 17.8 / 14.0 | 17.5 / 14.2 |

**What it shows.** At Full, the first sight of a frame with a Glow at Mix 50 falls from 23.8 to
14.3 ms; a Noise at Mix 50, which changes every frame, from 92.2 to 14.2 ms a frame played again,
about 6.5 times as fast. A held Glow played again costs the same either way (11.5 and 11.9 ms,
within the rounds' spread), as does Draft, where the CPU's drawing was already small.
