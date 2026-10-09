# B-236: frame times with Path Stroke along shape paths

Measured on 2026-10-08 on the code the D-357 code commit holds, before it was committed (the
fixture commit bb25da1 plus that code). No other cargo or rustc process was running and the
owner's app was not open: checked before every round and after the last, none. The processor's
load from background programs was not recorded. The timing test is `b236_stroke_shapes_timing`
in `tests/b236_stroke_shapes.rs`, built with `cargo test --release --test b236_stroke_shapes`
and run three rounds in a row with `--ignored b236_stroke_shapes_timing --exact`, each round
timing the four shots in the order below (about 19 seconds a round on the card, 47 on the
processor), once with Draw on: GPU and once, with `B236_CPU` set, by the processor.

The row with Noise alone is the comparison. B-235's mask numbers are in
`verification/B-235_stroke_timing_table.md`.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (`verification/B-08a_project.json`, 1920 by 1080, 24 a
second, 240 frames) with a Noise that changes every frame on its first three layers, and on top
a shape layer `paths` holding three circles (radius 400 about (960, 540), 220 about (560, 400),
300 about (1400, 620)) with no fill and no stroke, and Path Stroke on it with Path From Shape
Paths, so the shot has new input every frame. Every eighth frame is asked for whole, as the
viewer asks for it, at Full: with Draw on: GPU (`preview_frame_srgb8`), or by the processor alone
(`preview_frame_cached`). **First** is the median of the first loop of 30 frames, starting with
empty caches; **Again** the median of the next seven loops, 210 frames. Each figure is the median
of the three rounds, in milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone, the shape layer without the effect (before) | 18.8 | 16.4 | 46.3 | 46.9 |
| Noise, then Path Stroke from shapes as added (shape 1, Brush Size 2) | 20.6 | 18.9 | 48.5 | 47.8 |
| Noise, then Path Stroke from shapes, All Masks, Brush Size 24, Spacing 0 | 22.4 | 20.5 | 48.5 | 48.9 |
| Noise, then Path Stroke from shapes, All Masks, Brush Size 60, Hardness 0, Spacing 100 | 23.6 | 20.6 | 47.8 | 48.6 |

The rounds, first / again:

| Shot | Card round 1 | Card round 2 | Card round 3 | Processor round 1 | Processor round 2 | Processor round 3 |
|---|---|---|---|---|---|---|
| Noise alone | 18.8 / 16.1 | 18.6 / 16.4 | 19.6 / 17.4 | 46.2 / 46.6 | 47.4 / 46.9 | 46.3 / 49.2 |
| As added | 20.6 / 18.6 | 20.6 / 18.9 | 22.3 / 20.3 | 48.5 / 47.7 | 47.9 / 47.8 | 50.6 / 50.7 |
| All Masks, size 24, Spacing 0 | 22.4 / 19.7 | 22.2 / 20.5 | 25.1 / 21.9 | 48.0 / 48.1 | 48.5 / 48.9 | 50.9 / 49.1 |
| All Masks, size 60, Spacing 100 | 23.6 / 19.8 | 22.6 / 20.6 | 24.9 / 21.8 | 47.8 / 48.3 | 48.5 / 49.1 | 47.8 / 48.6 |

**Where it is drawn.** An effect on a shape layer is drawn by the processor even with Draw on:
GPU: the card takes a layer's effects only when the layer is a drawing (B-46's rule in
`src/compose.rs`, unchanged here), so with Draw on: GPU the stroke is worked on the processor and
the rest of the frame on the card. `verification/D-357_stroke_shapes_table.md` confirms it: no
card run for a shape layer's effect, and every viewer picture within 1 level of the export.

**Against the target.** EFFECTS.md gives Stroke Target P2 (4 ms or less added). On one 1080p
shape layer with three circles it adds about 2.5 to 4 ms a frame with Draw on: GPU and about 1 to
2 ms by the processor alone, inside the target. The card's own Path Stroke pass (B-235) is not
used for shape layers; putting effects on shape layers on the card is a separate, project-wide
change.
