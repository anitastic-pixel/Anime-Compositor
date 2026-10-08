# B-228: frame times with Pass Extract and Depth Key

Measured on 2026-10-08. No other cargo or rustc process was running: checked before and after
every round, none. The owner's app was not open. The timing test is `b228_depth_channels_timing`
in `tests/b228_depth_channels.rs`, built with `cargo test --release --test b228_depth_channels`
and run three rounds in a row, each round timing the four shots in the order below.

The two effects are new, so there is no "before" build: the row without them is the comparison.
Both are drawn by the graphics card in the viewer.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed**, B-227's way:

- **The shots.** One layer, a 1920 by 1080 EXR still written by the test (half colour and alpha,
  a float depth `Z`, half normals `N.x`, `N.y`, `N.z`), in a 1920 by 1080 composition at 24
  frames a second; every eighth frame of 240, at Full, with Draw on: GPU. Each frame is asked for
  whole, in milliseconds, the way the viewer asks for it. On the layer: the effect, then a Noise
  that changes every frame, so no finished frame is kept.
  - Noise alone.
  - Pass Extract, the depth, Black 0, White 50, then Noise.
  - Pass Extract, the normals, Black -1, White 1, then Noise.
  - Depth Key at 15, Feather 5, then Noise.
- **First** is the median of the first loop's 30 frames, started with empty caches (the file and
  its pass are read on the first frame only, so the median does not show that read).
- **Again** is the median of the next seven loops' 210 frames.

Each figure is the median of the three rounds.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 5.5 | 2.1 |
| Pass Extract, depth 0 to 50, then Noise | Full | 5.5 | 2.1 |
| Pass Extract, normals, then Noise | Full | 5.5 | 2.1 |
| Depth Key at 15, feather 5, then Noise | Full | 5.1 | 2.4 |

The rounds, first / again:

| Shot | Round 1 | Round 2 | Round 3 |
|---|---|---|---|
| Noise alone | 5.5 / 2.0 | 5.5 / 2.0 | 5.6 / 2.1 |
| Pass Extract, depth | 5.5 / 2.0 | 5.5 / 2.1 | 5.5 / 2.1 |
| Pass Extract, normals | 5.4 / 2.0 | 5.5 / 2.1 | 5.5 / 2.1 |
| Depth Key | 5.0 / 2.0 | 5.3 / 2.4 | 5.1 / 2.4 |

**Against the target.** EFFECTS.md's Target P1 for one-pixel effects is 1 ms or less added. Pass
Extract adds nothing measurable here (0.0 ms played again) and Depth Key 0.3 ms: both within it.
These are whole-frame times of one layer, not the reference shot's three, so they are not
comparable with B-227's table.
