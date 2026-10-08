# B-229: frame times with ID Key and Pass Extract of the object ids

Measured on 2026-10-08, after the code commit 70f9255. No other cargo or rustc process was
running: checked before and after every round, none. The owner's app was not open. The timing
test is `b229_id_key_timing` in `tests/b229_id_key.rs`, built with
`cargo test --release --test b229_id_key` and run three rounds in a row, each round timing the
four shots in the order below.

ID Key is new, and Pass Extract's object id pass is new, so there is no "before" build: the row
without them is the comparison. Both are drawn by the graphics card in the viewer.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed**, B-228's way:

- **The shots.** One layer, a 1920 by 1080 EXR still written by the test the way Blender writes
  one (half `ViewLayer.Combined.R`, `.G`, `.B`, `.A`, and a float object id
  `ViewLayer.IndexOB.X` holding ids 0 to 5 in blocks), in a 1920 by 1080 composition at 24 frames
  a second; every eighth frame of 240, at Full, with Draw on: GPU. Each frame is asked for whole,
  in milliseconds, the way the viewer asks for it. On the layer: the effect, then a Noise that
  changes every frame, so no finished frame is kept.
  - Noise alone.
  - ID Key, object id 3, no feather, then Noise.
  - ID Key, object id 3, Feather 3, then Noise.
  - Pass Extract, object ids, Black 0, White 5, then Noise.
- **First** is the median of the first loop's 30 frames, started with empty caches (the file and
  its ids are read on the first frame only, so the median does not show that read).
- **Again** is the median of the next seven loops' 210 frames.

Each figure is the median of the three rounds.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 5.7 | 2.1 |
| ID Key, object 3, no feather, then Noise | Full | 3.9 | 1.9 |
| ID Key, object 3, feather 3, then Noise | Full | 3.8 | 1.9 |
| Pass Extract, object ids 0 to 5, then Noise | Full | 5.6 | 2.1 |

The rounds, first / again:

| Shot | Round 1 | Round 2 | Round 3 |
|---|---|---|---|
| Noise alone | 6.2 / 2.3 | 5.7 / 2.1 | 5.6 / 2.0 |
| ID Key, no feather | 4.3 / 1.9 | 3.9 / 1.9 | 3.7 / 1.9 |
| ID Key, feather 3 | 3.9 / 1.9 | 3.8 / 1.9 | 3.7 / 1.9 |
| Pass Extract, object ids | 6.1 / 2.5 | 5.6 / 2.1 | 5.6 / 2.1 |

**Against the target.** EFFECTS.md's Target P1 for one-pixel effects is 1 ms or less added.
Neither effect adds anything measurable here, played again: Pass Extract 0.0 ms, and ID Key comes
out 0.2 ms below Noise alone. That lower figure is not a speed-up claimed for ID Key: after it,
about five sixths of the frame is clear, which is likely cheaper to finish, and the cause was not
measured further. Both are within the target. These are whole-frame times of one layer, the same
kind as B-228's table but on a different file, so the two tables are not compared row by row.
