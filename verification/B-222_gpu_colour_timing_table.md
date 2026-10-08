# B-222: frame times before and after, colour effects on the card

Measured on 2026-10-08. No other cargo or rustc process was running: checked before the runs and
after every run, none. The timing test is `b222_gpu_colour_timing` in `tests/b222_gpu_colour.rs`.
It was built with `cargo test --release --test b222_gpu_colour --no-run` on two builds, each in
its own folder. They were taken turn about for three rounds, with the order turned each round:

- **before**: 5aa3e91, the commit before B-222, with B-222's test file. A layer with any of the
  ten effects is drawn by the CPU from that effect on.
- **after**: B-222. The card draws the ten effects, and draws a row of colour effects in one
  pass (B-172).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed**, B-221's way:

- **The shots.** The reference shot, every eighth frame of 240, at Full, with Draw on: GPU. Each
  frame is asked for whole, in milliseconds, the way the viewer asks for it. The effects are on
  the shot's first three layers; the second layer has a Drop Shadow before them.
  - The first shot has Color Key, then Exposure, then Tint.
  - The second shot has a Noise that changes every frame, then Exposure, then Tint.
- **First** is the median of the first loop's 30 frames, started with empty caches. It shows what
  a frame costs the first time it is seen.
- **Again** is the median of the next seven loops' 210 frames.
- **Left to the card** is how many effects the card draws on each of the first three layers at
  frame 100.

Each figure in the table below is the median of the three rounds.

| Shot | Quality | Left to the card, before | Left to the card, after | First, before | First, after | Again, before | Again, after |
|---|---|---|---|---:|---:|---:|---:|
| Color Key, Exposure, Tint | Full | 0 / 0 / 0 | 3 / 3 / 3 | 12.4 | 11.8 | 10.1 | 9.8 |
| Noise, then Exposure, Tint | Full | 0 / 0 / 0 | 3 / 4 / 3 | 81.7 | 18.7 | 86.7 | 14.3 |

The three rounds, first / again, in ms:

| Shot | Quality | before 1 | before 2 | before 3 | after 1 | after 2 | after 3 |
|---|---|---|---|---|---|---|---|
| Color Key, Exposure, Tint | Full | 12.4 / 10.3 | 11.9 / 10.1 | 12.7 / 10.1 | 11.7 / 9.7 | 11.8 / 9.9 | 12.0 / 9.8 |
| Noise, then Exposure, Tint | Full | 87.8 / 86.8 | 81.7 / 86.6 | 79.8 / 86.7 | 17.8 / 14.2 | 19.8 / 14.9 | 18.7 / 14.3 |

**What it shows.**

- **The changing Noise.** It is followed by Exposure and Tint, which the CPU drew before B-222.
  - Played again: from 86.7 to 14.3 ms a frame, about 6 times as fast.
  - First sight: from 81.7 to 18.7 ms.
- **The shot that holds still.** With Color Key, Exposure and Tint, a frame costs about the same
  either way. First sight is 12.4 and 11.8 ms; played again is 10.1 and 9.8 ms, within the
  rounds' spread. The picture does not change, so it is already kept after it is first drawn.
