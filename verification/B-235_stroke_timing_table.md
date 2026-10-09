# B-235: frame times with Path Stroke

Measured on 2026-10-08, after the code commit (4254823), on the code that commit holds. No other
cargo or rustc process was running and the owner's app was not open: checked before every round
and after the last, none. The processor's load from background programs was not recorded. The
timing test is `b235_stroke_timing` in `tests/b235_stroke.rs`, built with
`cargo test --release --test b235_stroke` and run three rounds in a row with
`--ignored b235_stroke_timing --exact`, each round timing the four shots in the order below
(about 23 seconds a round on the card, 42 on the processor), once drawn by the card and once,
with `B235_CPU` set, by the processor.

The effect is new, so there is no "before" build: the row with Noise alone is the comparison.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (`verification/B-08a_project.json`, 1920 by 1080, 24 a
second, 240 frames) with three circle masks of mode None (radius 400, 220 and 300) on each of its
first three layers, a Noise that changes every frame on each, and Path Stroke after it, so the
effect has new input every frame and is worked every time. Every eighth frame is asked for whole,
as the viewer asks for it, at Full: with Draw on: GPU (`preview_frame_srgb8`), or by the
processor alone (`preview_frame_cached`). **First** is the median of the first loop of 30 frames,
starting with empty caches; **Again** the median of the next seven loops, 210 frames. Each figure
is the median of the three rounds, in milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone (before) | 15.4 | 11.7 | 42.7 | 43.3 |
| Noise, then Path Stroke as added (mask 1, Brush Size 2, Spacing 15) | 23.7 | 20.1 | 43.9 | 43.5 |
| Noise, then Path Stroke, All Masks, Brush Size 24, Spacing 0 | 47.9 | 45.2 | 45.3 | 45.4 |
| Noise, then Path Stroke, All Masks, Brush Size 60, Hardness 0, Spacing 100 | 18.8 | 16.3 | 42.5 | 43.0 |

The rounds, first / again:

| Shot | Card round 1 | Card round 2 | Card round 3 | Processor round 1 | Processor round 2 | Processor round 3 |
|---|---|---|---|---|---|---|
| Noise alone | 15.4 / 11.7 | 15.7 / 11.7 | 15.2 / 12.0 | 42.1 / 43.3 | 42.7 / 43.4 | 43.5 / 43.1 |
| As added | 23.7 / 20.1 | 22.9 / 20.0 | 24.0 / 20.2 | 43.4 / 43.5 | 44.1 / 44.1 | 43.9 / 42.6 |
| All Masks, size 24, Spacing 0 | 47.9 / 45.0 | 48.1 / 45.2 | 47.7 / 45.6 | 44.9 / 45.4 | 45.4 / 45.6 | 45.3 / 45.4 |
| All Masks, size 60, Spacing 100 | 18.8 / 15.6 | 18.6 / 17.2 | 19.6 / 16.3 | 42.4 / 43.0 | 42.5 / 43.1 | 42.5 / 42.8 |

**A first card pass, before this table.** The card pass as first written looked at every piece
of every path for every pixel: one round on the same machine, same build settings, gave 246.6 /
244.0 as added and 547.9 / 547.6 at size 24 with Spacing 0. The code commit holds the faster pass,
which hands the card, for each band of 16 rows, the list of pieces whose box reaches it; the
pixels are the same (`verification/D-356_stroke_table.md`, 247 of 247).

**Against the target.** EFFECTS.md gives Stroke Target P2 (4 ms or less added).

- **On the card** it adds about 8 ms a frame on three 1080p layers as added (about 3 ms a
  layer), about 5 ms with a wide spaced brush, and about 34 ms (about 11 ms a layer) with a 24-pixel
  continuous brush on all three masks. That last is above the target: each pixel still looks at
  every piece of its band, and a band crossing the top or bottom of a circle holds many. Binning
  by 16 by 16 tiles instead of rows would cut it; not done here.
- **On the processor** it adds about 1 to 2 ms: each row skips every piece whose box misses it.
  The processor's frame is slower overall (Noise alone takes 43 ms there), so the card stays the
  faster way to play.
