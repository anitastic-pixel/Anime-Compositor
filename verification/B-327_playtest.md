# B-327: Brush Strokes

Built on 2026-10-10 as D-447, under your /loop request: **Brush Strokes**, in **Stylize**. It
repaints the picture as short dabs of paint all leaning one way, as if painted with a brush.

- **Stroke Angle** (135): which way the strokes run, 0 up, 90 right, 135 down and to the right.
- **Brush Size** (2 pixels, 0.5 to 20): how thick each stroke is.
- **Stroke Length** (8 pixels, 0 to 100): how long; 0 paints dots.
- **Stroke Density** (1, 0.1 to 4): more strokes, overlapping, as it rises; below 1 the picture
  shows between them.
- **Stroke Randomness** (1, 0 to 2): how much each stroke wanders from the settings; 0 is a tidy
  grid of identical strokes.
- **Paint Surface**: Paint On Original Image (the picture shows between strokes), Paint On
  Transparent, Paint On White or Paint On Black.
- **Blend With Original** (0 to 100%): mixes the picture back in; 100 is the picture untouched.
- **Random Seed** (ours): a different set of strokes.
- **New Strokes Each Frame** (ours, on): After Effects paints new strokes every frame, so they
  jitter when played. Turn it off for the same strokes every frame.

The controls are After Effects' own; how they draw is our own rule (Adobe publishes none), not
matched to After Effects pixel for pixel.

The check, `verification/D-447_brush_strokes_table.md` (181 of 181), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-447 pictures/`, over the
street (`town.png`; `1_before.png` without the effect).

## What to check

1. **As added.** `2_as_added.png`: the street repainted in short strokes leaning down and to the
   right; the buildings' edges ragged like paint. `2b_as_added_frame_1.png`, the next frame: new
   strokes, so it looks a little different.
2. **Held.** `3_held.png`, New Strokes Each Frame off: the same strokes on every frame.
3. **Level strokes.** `4_level_strokes.png` (angle 90, randomness 0, length 30): long level strokes
   running right.
4. **On black.** `5_sparse_on_black.png` (density 0.3): scattered strokes with black between them.
5. **Big brush.** `6_brush_8.png` (brush 8, length 24): big blocky dabs.
6. **Blend.** `7_blend_50.png`: halfway between the street and `3_held.png`.
7. **In the app.** Add Brush Strokes (Stylize) to a picture and play: the strokes jitter each
   frame; turn New Strokes Each Frame off and they hold still. Try the angle dial, Brush Size,
   Stroke Length 0 (dots) and each Paint Surface.
8. **Out of range.** Type 30 in Brush Size: it is refused with a sentence giving the range.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-327_brush_strokes_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a
moving Noise and a Brush Strokes on three layers, played again, 498.1 ms a frame on the card
as added against 250.9 for the Noise alone; on the processor 222.9 against
61.7: a busy machine cannot say whether the card is quicker than the processor for this effect.
