# B-233: frame times with Soft Physical Glow

Measured on 2026-10-08, after the code commit (d298455), on the code that commit holds. No other
cargo or rustc process was running and the owner's app was not open: checked before every round
and after the last, none. The processor's load from background programs was not recorded. The
timing test is `b233_soft_glow_timing` in `tests/b233_soft_glow.rs`, built with
`cargo test --release --test b233_soft_glow` and run three rounds in a row with
`--ignored b233_soft_glow_timing --exact`, each round timing the five shots in the order below
(about 2 minutes a round).

The effect is new, so there is no "before" build: the row with Noise alone is the comparison.
Soft Physical Glow is drawn by the graphics card, except a Threshold above 0 with Threshold
Smooth 0 (a hard step), which is drawn by the processor, the viewer's frame then finished by the
card.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (`verification/B-08a_project.json`, 1920 by 1080, 24 a
second, 240 frames) with a Noise that changes every frame on each of its first three layers and
the effect after it, so the effect has new input every frame and is worked every time. Every
eighth frame is asked for whole, as the viewer asks for it (`preview_frame_srgb8`), at Full with
Draw on: GPU. **First** is the median of the first loop of 30 frames, starting with empty
caches; **Again** the median of the next seven loops, 210 frames. Each figure is the median of
the three rounds, in milliseconds a frame.

| Shot | First | Again |
|---|---:|---:|
| Noise alone (before) | 15.6 | 12.1 |
| Noise, then Soft Physical Glow as added, radius 500 (card) | 61.7 | 59.8 |
| Noise, then Soft Physical Glow, radius 60, threshold 40 smooth 50 (card) | 52.8 | 49.0 |
| Noise, then Soft Physical Glow, radius 2000 (card) | 139.1 | 136.5 |
| Noise, then Soft Physical Glow, radius 60, threshold 40, no smooth (processor) | 368.3 | 357.5 |

The rounds, first / again:

| Shot | Round 1 | Round 2 | Round 3 |
|---|---|---|---|
| Noise alone | 15.5 / 11.8 | 15.8 / 12.6 | 15.6 / 12.1 |
| As added, radius 500 | 61.5 / 59.6 | 62.7 / 60.2 | 61.7 / 59.8 |
| Radius 60, threshold 40 smooth 50 | 52.8 / 49.0 | 52.0 / 48.5 | 54.7 / 51.1 |
| Radius 2000 | 141.2 / 137.4 | 137.9 / 136.5 | 139.1 / 136.5 |
| Radius 60, hard threshold (processor) | 362.6 / 357.5 | 375.6 / 354.0 | 368.3 / 366.5 |

**Against the target.** EFFECTS.md gives Soft Physical Glow Target P3 (8 ms or less added). It
does not meet it.

- **On the card** it adds about 37 to 48 ms a frame on three 1080p layers at radius 60 to 500,
  about 12 to 16 ms a layer, and about 124 ms (about 41 ms a layer) at radius 2,000. Most of it is
  likely the grown layer (not measured apart): so no glow is cut off, the layer grows by the
  glow's reach, about 540 pixels
  on each side at radius 500 and about 2,200 at radius 2,000, and every level is added over that
  whole grown picture. The way to speed it up, not done here, is to add the levels only over the
  part that can be seen in the frame (as B-158 does for the viewer), keeping the grown layer's
  size for what lies beyond.
- **On the processor** (hard threshold only) it takes about 0.35 s a frame on three 1080p
  layers, so the viewer plays at about 3 frames a second; export is unaffected in result. Giving
  the threshold a little Smooth moves it to the card.
