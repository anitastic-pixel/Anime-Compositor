# B-254: Color Link

Built on 2026-10-09 under your effects loop request, decided as D-375. **Color Link** (in
**Color Correction**) is our version of After Effects' Color Link: one colour is read from a
whole layer's picture, another layer's or the layer's own, and laid over the layer, so it keeps
matching that layer as it changes.

The settings, with the values they start at:

- **Source Layer** (none, meaning the layer itself): the layer whose colour is read, with its
  masks and effects, to its edges.
- **Sample** (Average): Average, Median, Brightest, Darkest, Max RGB (each channel's highest)
  or Min RGB (each channel's lowest).
- **Clip** (5): 0 to 49 per cent; that share of the darkest and brightest is left out before
  the colour is read.
- **Stencil** (Off): On lays the colour only where the layer shows; Off lays it over the whole
  layer, its empty parts too.
- **Opacity** (100): 0 to 100.
- **Blending Mode** (Normal): Normal, Multiply, Screen, Add, Overlay or Soft Light.

The check, `verification/D-375_color_link_table.md` (155 of 155), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-375 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: the whole picture one flat colour, the street's own average.
2. **A wash.** `3_overlay_50.png`, Overlay at 50, Stencil On: the street keeps its light and
   dark under a faint wash of its own average colour.
3. **Multiply.** `4_brightest_multiply_40.png`, Brightest, Multiply at 40: a little darker
   everywhere, tinted by the lit windows.
4. **In the app, another layer.** Put a small coloured solid in the composition, hide it, and
   choose it as Source Layer on a drawing with Stencil On, Overlay, 60: the drawing takes on the
   solid's colour. Change the solid's colour: the drawing follows.
5. **A layer that goes.** Delete the source layer: the drawing goes back to how it was and a
   warning names the missing layer; undo brings both back.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

**Timing.** On the reference shot, three 1080p layers with a moving Noise, played again at Full: 11.5 ms a frame without, 38.0 with Color Link reading another layer, about 8.5 ms a layer, because the source layer's colour is counted by the processor at every frame; 83.4 reading the layer's own picture, drawn by the processor (`verification/B-253_colour_timing_table.md`).

## Not built

- Adobe does not publish its method, so the rule is ours (document 21).
- No alpha samples (After Effects' Average Alpha and the like); six blending modes, not After
  Effects' full list.
- With Source Layer left at none (the layer's own picture), the colour is read and laid on by the
  processor; with a layer chosen, the graphics card lays it on.
- The layer chooser says "none" for the layer itself.

## Waiting for you

- FX-CLINK-019's file says the missing-layer warning comes at each frame only; every other
  effect that reads a layer also warns when the project opens (D-189), and so does this one. The
  fix to the file (one word) is proposed, not made: your call.
