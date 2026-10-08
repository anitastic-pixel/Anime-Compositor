# B-223: frame times before and after, five blurs on the card

Measured on 2026-10-08. No other cargo or rustc process was running: checked before the runs and
after every run, none. The timing test is `b223_gpu_blurs_timing` in `tests/b223_gpu_blurs.rs`.
It was built with `cargo test --release --test b223_gpu_blurs` on two builds, each in its own
folder. They were taken turn about for three rounds, with the order turned each round:

- **before**: 1d678ee, the commit before B-223, with B-223's test file. A layer with any of the
  five blurs is drawn by the CPU from that blur on.
- **after**: B-223. The card draws the five blurs.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed**, B-222's way:

- **The shots.** The reference shot, every eighth frame of 240, at Full, with Draw on: GPU. Each
  frame is asked for whole, in milliseconds, the way the viewer asks for it. The blur is on the
  shot's first three layers; the second layer has a Drop Shadow before it.
  - Two shots hold still: Fast Box Blur (radius 30, 3 iterations) alone, and Selective Color
    Blur (blur 30, two colours, tolerance 30) alone.
  - Four shots have a Noise that changes every frame before the blur: Fast Box Blur 30 x 3; CC
    Vector Blur (Natural, amount 20, map softness 20, the layer itself as map); Channel Blur
    (red 12, green 4, blue 0, alpha 6); Compound Blur (maximum 30, layer 4 as the map).
- **First** is the median of the first loop's 30 frames, started with empty caches. It shows what
  a frame costs the first time it is seen.
- **Again** is the median of the next seven loops' 210 frames.
- **Left to the card** is how many effects the card draws on each of the first three layers at
  frame 100. Selective Color Blur only begins a run, so the Drop Shadow before it on the second
  layer stays the CPU's.

Each figure in the table below is the median of the three rounds.

| Shot | Quality | Left to the card, before | Left to the card, after | First, before | First, after | Again, before | Again, after |
|---|---|---|---|---:|---:|---:|---:|
| Fast Box Blur 30 x 3 | Full | 0 / 0 / 0 | 1 / 2 / 1 | 12.0 | 11.5 | 9.8 | 9.1 |
| Noise, then Fast Box Blur 30 x 3 | Full | 0 / 0 / 0 | 2 / 3 / 2 | 134.7 | 25.2 | 109.6 | 21.8 |
| Noise, then CC Vector Blur | Full | 0 / 0 / 0 | 2 / 3 / 2 | 241.4 | 46.1 | 195.1 | 42.8 |
| Noise, then Channel Blur | Full | 0 / 0 / 0 | 2 / 3 / 2 | 261.1 | 24.4 | 206.7 | 21.3 |
| Noise, then Compound Blur | Full | 0 / 0 / 0 | 2 / 3 / 2 | 454.1 | 47.5 | 426.9 | 49.7 |
| Selective Color Blur | Full | 0 / 0 / 0 | 1 / 1 / 1 | 14.8 | 11.5 | 10.5 | 9.6 |

The three rounds, first / again, in ms:

| Shot | Quality | before 1 | before 2 | before 3 | after 1 | after 2 | after 3 |
|---|---|---|---|---|---|---|---|
| Fast Box Blur 30 x 3 | Full | 12.0 / 9.8 | 11.6 / 9.6 | 12.0 / 10.1 | 11.1 / 9.1 | 11.7 / 9.1 | 11.5 / 9.5 |
| Noise, then Fast Box Blur 30 x 3 | Full | 121.7 / 107.7 | 134.7 / 112.8 | 135.4 / 109.6 | 25.2 / 21.8 | 24.9 / 21.8 | 25.3 / 21.8 |
| Noise, then CC Vector Blur | Full | 242.5 / 194.1 | 241.4 / 195.1 | 240.5 / 196.6 | 46.1 / 42.8 | 46.1 / 42.7 | 45.7 / 42.8 |
| Noise, then Channel Blur | Full | 261.1 / 209.3 | 258.8 / 206.5 | 262.4 / 206.7 | 24.5 / 22.9 | 24.4 / 21.3 | 24.3 / 21.1 |
| Noise, then Compound Blur | Full | 456.8 / 426.9 | 452.3 / 433.0 | 454.1 / 425.8 | 52.7 / 54.7 | 47.5 / 49.7 | 47.2 / 49.6 |
| Selective Color Blur | Full | 14.1 / 10.0 | 24.9 / 10.5 | 14.8 / 10.9 | 12.6 / 10.7 | 11.5 / 9.6 | 11.2 / 9.4 |

**What it shows.**

- **A blur after a changing Noise.** Before B-223 the CPU drew each of these blurs every frame.
  Played again:
  - Fast Box Blur: from 109.6 to 21.8 ms a frame, about 5 times as fast.
  - CC Vector Blur: from 195.1 to 42.8 ms, about 4.5 times.
  - Channel Blur: from 206.7 to 21.3 ms, about 10 times.
  - Compound Blur: from 426.9 to 49.7 ms, about 8.5 times.
  - First sight falls the same way (134.7 to 25.2, 241.4 to 46.1, 261.1 to 24.4, 454.1 to 47.5 ms).
- **The shots that hold still.** Fast Box Blur and Selective Color Blur alone cost about the same
  either way (again 9.8 and 9.1 ms, 10.5 and 9.6 ms), since a picture that does not change is
  kept after it is first drawn. Selective Color Blur's second round "before" first sight, 24.9 ms,
  is one slow round; the median ignores it.
- **Compound Blur** on the card blurs all five of its blur sizes, where the CPU stops at the
  largest the map asks for, and its big blurs may be worked small and enlarged (D-235). It is the
  slowest of the five on the card, 49.7 ms a frame, still the largest gain.
