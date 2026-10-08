# B-224: frame times before and after, Displacement Map and CC Glass on the card

Measured on 2026-10-08. No other cargo or rustc process was running: checked before the runs and
after every run, none. The timing test is `b224_gpu_maps_timing` in `tests/b224_gpu_maps.rs`.
It was built with `cargo test --release --test b224_gpu_maps` on two builds, each in its own
folder. They were taken turn about for three rounds, with the order turned each round:

- **before**: f425a48, the commit before B-224, with B-224's test file and its one-line fix to
  B-223's Selective Color Blur pass (so both builds can make the card's colour runs). A layer
  with Displacement Map or CC Glass is drawn by the CPU from that effect on.
- **after**: B-224 (74f891a). The card draws both.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed**, B-223's way:

- **The shots.** The reference shot, every eighth frame of 240, at Full, with Draw on: GPU. Each
  frame is asked for whole, in milliseconds, the way the viewer asks for it. A Noise that changes
  every frame comes first, then the effect, on the shot's first three layers; the second layer
  has a Drop Shadow before them.
  - Displacement Map, layer 4 as the map, stretched, red across and green down, 12 pixels each.
  - Displacement Map, expanded: luminance across and alpha down, 20 pixels each, Expand Output on.
  - CC Glass with the layer itself as the bump: intensity, softness 10, height 50, displacement
    20, light from -45 degrees, intensity 50.
  - CC Glass on a map: layer 4 as the bump, luminance, softness 25, height 60, displacement 40,
    light from 135 degrees, intensity 70.
- **First** is the median of the first loop's 30 frames, started with empty caches. It shows what
  a frame costs the first time it is seen.
- **Again** is the median of the next seven loops' 210 frames.
- **Left to the card** is how many effects the card draws on each of the first three layers at
  frame 100.

Each figure in the table below is the median of the three rounds.

| Shot | Quality | Left to the card, before | Left to the card, after | First, before | First, after | Again, before | Again, after |
|---|---|---|---|---:|---:|---:|---:|
| Noise, then Displacement Map | Full | 0 / 0 / 0 | 2 / 3 / 2 | 97.0 | 44.2 | 114.8 | 44.9 |
| Noise, then Displacement Map, expanded | Full | 0 / 0 / 0 | 2 / 3 / 2 | 120.5 | 43.4 | 108.6 | 44.1 |
| Noise, then CC Glass | Full | 0 / 0 / 0 | 2 / 3 / 2 | 193.0 | 21.4 | 145.7 | 18.5 |
| Noise, then CC Glass on a map | Full | 0 / 0 / 0 | 2 / 3 / 2 | 200.7 | 47.6 | 166.3 | 48.1 |

The three rounds, first / again, in ms:

| Shot | Quality | before 1 | before 2 | before 3 | after 1 | after 2 | after 3 |
|---|---|---|---|---|---|---|---|
| Noise, then Displacement Map | Full | 100.4 / 114.8 | 86.8 / 118.8 | 97.0 / 108.1 | 44.2 / 44.8 | 46.1 / 48.1 | 43.3 / 44.9 |
| Noise, then Displacement Map, expanded | Full | 125.2 / 118.6 | 120.5 / 108.6 | 111.8 / 108.1 | 43.4 / 44.0 | 46.3 / 44.1 | 43.2 / 44.3 |
| Noise, then CC Glass | Full | 191.1 / 145.7 | 193.0 / 153.8 | 210.4 / 145.3 | 21.4 / 18.4 | 21.1 / 18.5 | 22.2 / 18.7 |
| Noise, then CC Glass on a map | Full | 201.8 / 166.3 | 199.5 / 165.6 | 200.7 / 174.9 | 47.6 / 48.6 | 46.9 / 47.1 | 48.3 / 48.1 |

**What it shows.**

- **Played again**, each about 2.5 to 8 times as fast:
  - Displacement Map: from 114.8 to 44.9 ms a frame; expanded, from 108.6 to 44.1 ms.
  - CC Glass with the layer as its own bump: from 145.7 to 18.5 ms.
  - CC Glass on a map: from 166.3 to 48.1 ms.
  - First sight falls the same way (97.0 to 44.2, 120.5 to 43.4, 193.0 to 21.4, 200.7 to 47.6 ms).
- **A map layer costs about 25 ms a frame.** CC Glass on itself plays at 18.5 ms, on a map at
  48.1; Displacement Map, which always reads a map, at about 44 ms, as Compound Blur did in
  B-223 (49.7 ms). The map is copied to the card as a new picture each time an effect reads it
  (`map_texture`), once per layer, every frame. Keeping a map that has not changed on the card
  is a likely next gain, not measured or taken here.
