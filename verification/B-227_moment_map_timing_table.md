# B-227: frame times with Moment Map

Measured on 2026-10-08. No other cargo or rustc process was running: checked before and after
every round, none. The owner's app was not open. The timing test is `b227_moment_map_timing` in
`tests/b227_moment_map.rs`, built with `cargo test --release --test b227_moment_map` and run
three rounds in a row, each round timing the three shots in the order below.

Moment Map is new, so there is no "before" build: the row without it is the comparison. It is
drawn by the processor, as Echo is (several frames of the layer).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed**, B-226's way:

- **The shots.** The reference shot (1920 by 1080, 24 frames a second), every eighth frame of
  240, at Full, with Draw on: GPU. Each frame is asked for whole, in milliseconds, the way the
  viewer asks for it. On each of the shot's first three layers: the Moment Map, then a Noise that
  changes every frame, so no finished frame is kept.
  - Noise alone: no Moment Map.
  - Moment Map with the layer's own brightness as the map, Max Displacement Time 1 second, Time
    Resolution 60: up to 49 different frames of each layer in one frame.
  - Moment Map with layer-4 as the map, stretched, Max 0.5 seconds, Time Resolution 24: up to 25.
- **First** is the median of the first loop's 30 frames, started with empty caches.
- **Again** is the median of the next seven loops' 210 frames.

Each figure is the median of the three rounds.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 15.6 | 12.1 |
| Moment Map, its own brightness, Max 1 s, Resolution 60, then Noise | Full | 202.7 | 235.2 |
| Moment Map, layer-4 as the map, Max 0.5 s, Resolution 24, then Noise | Full | 161.7 | 141.8 |

The rounds, first / again:

| Shot | Round 1 | Round 2 | Round 3 |
|---|---|---|---|
| Noise alone | 15.9 / 12.4 | 15.6 / 12.1 | 15.5 / 12.0 |
| Own brightness, Max 1 s | 211.9 / 235.3 | 200.6 / 235.2 | 202.7 / 234.2 |
| layer-4, Max 0.5 s | 180.0 / 141.8 | 161.7 / 142.8 | 158.7 / 141.3 |

**An earlier version, measured the same way the same day.** The first build copied each frame's
picture over the whole layer once per frame used. Its medians were: Noise alone 16.0 / 12.5, own
brightness 466.4 / 410.0, layer-4 170.8 / 150.4. The build above copies each pixel once, from the
frame it uses, and reads that frame's picture in place.

**Against the target.** EFFECTS.md's Target P4 for CPU-only time effects is 100 ms a frame.
These shots put the effect on three 1920 by 1080 layers at once and miss it: most of the time is
drawing each layer at up to 49 other frames, which is the effect's nature (Echo pays the same per
copy). Again is slower than first with its own brightness; the likely reason, not checked, is
that 49 frames of three layers are more than the viewer's cache holds.
