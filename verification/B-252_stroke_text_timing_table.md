# B-252: frame times with Path Stroke along text outlines

Measured on 2026-10-09 on the code the B-252 code commit holds, before it was committed (the
fixture commit dc693ec plus that code). No other cargo or rustc process was running and the
owner's app was not open: `tasklist` checked before every round, between rounds and after the
last, none. The processor's load from background programs was not recorded. The timing test is
`b252_stroke_text_timing` in `tests/b252_stroke_text.rs`, built with
`cargo test --release --test b252_stroke_text` and run three rounds in a row with
`--ignored b252_stroke_text_timing --exact`, each round timing the four shots in the order below,
once with Draw on: GPU and once, with `B252_CPU` set, by the processor.

The row with Noise alone is the comparison. Mask and shape numbers are in
`verification/B-235_stroke_timing_table.md` and `verification/B-236_stroke_shapes_timing_table.md`.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (`verification/B-08a_project.json`, 1920 by 1080, 24 a
second, 240 frames) with a Noise that changes every frame on its first three layers, and on top
a text layer `words`, "Path Stroke" in the bundled M PLUS Rounded 1c, size 200, white, at
(360, 620), with Path Stroke on it, Path From Text Outlines, so the shot has new input every
frame. Every eighth frame is asked for whole, as the viewer asks for it, at Full: with Draw on:
GPU (`preview_frame_srgb8`), or by the processor alone (`preview_frame_cached`). **First** is the
median of the first loop of 30 frames, starting with empty caches; **Again** the median of the
next seven loops, 210 frames. Each figure is the median of the three rounds, in milliseconds a
frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone, the text layer without the effect (before) | 16.1 | 12.6 | 44.8 | 47.6 |
| Noise, then Path Stroke from text outlines, outline 1 alone, Brush Size 2 | 24.5 | 22.3 | 52.4 | 52.8 |
| Noise, then Path Stroke from text outlines, All Masks, Stroke Sequentially, End 50, Brush Size 6 | 26.5 | 23.4 | 52.7 | 52.9 |
| Noise, then Path Stroke from text outlines, All Masks, Brush Size 24, Spacing 0 | 28.1 | 25.2 | 54.1 | 54.2 |

The rounds, first / again:

| Shot | Card round 1 | Card round 2 | Card round 3 | Processor round 1 | Processor round 2 | Processor round 3 |
|---|---|---|---|---|---|---|
| Noise alone | 16.1 / 12.6 | 17.0 / 12.6 | 15.8 / 12.4 | 45.3 / 48.6 | 44.6 / 47.5 | 44.8 / 47.6 |
| Outline 1, Brush Size 2 | 24.5 / 22.3 | 24.7 / 22.7 | 24.2 / 22.1 | 52.8 / 53.5 | 52.2 / 52.8 | 52.4 / 52.8 |
| Sequential, End 50, Brush Size 6 | 27.3 / 23.4 | 26.5 / 23.7 | 26.4 / 23.3 | 53.6 / 53.5 | 52.7 / 52.8 | 51.9 / 52.9 |
| All Masks, Brush Size 24, Spacing 0 | 28.1 / 25.2 | 28.0 / 25.4 | 28.5 / 25.1 | 54.6 / 54.9 | 53.5 / 54.2 | 54.1 / 54.2 |

**Where it is drawn.** A text layer has no cel, so its effects are drawn by the processor even
with Draw on: GPU (B-46's rule in `src/compose.rs`, unchanged here): the stroke is worked on the
processor and the rest of the frame on the card. `verification/D-373_stroke_text_table.md`
confirms it: no card run for the stroke, and every viewer picture within 1 level of the export.

**Against the target.** EFFECTS.md gives Stroke Target P2 (4 ms or less added). On one 1080p
text layer it adds about 10 to 13 ms a frame with Draw on: GPU and about 5 to 7 ms by the
processor alone: **over the target**. Most of it is not the stroke itself. A text layer without
effects is drawn once and its picture kept from frame to frame; with an effect on it the layer
is drawn again every frame, the stroke added, and with Draw on: GPU the whole 1920 by 1080
picture is sent to the card again every frame. Keeping a stroked text layer's picture between
frames when nothing it depends on changes, or drawing text layers' effects on the card, is a
separate change, not made here.
