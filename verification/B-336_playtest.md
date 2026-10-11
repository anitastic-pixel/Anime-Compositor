# B-336: Deband

Built on 2026-10-10 as D-456, under your /loop request: our own effect named Deband, in
**Blur & Sharpen** (beside Remove Grain), with the controls of CelFX's Deband. It smooths away
the visible steps (banding) in a soft gradient such as a sky, and adds a fine dither so they do
not come back, while keeping hard edges hard.

- **View** (Normal): Edge Only shows in white what counts as an edge; Selected Edge shows the
  result with the edges in red.
- **Edge Threshold** (4) and **Edge Softness** (2): a step to a neighbour, in levels, above the
  threshold starts to count as an edge, fully so past the threshold plus the softness.
- **Blur Amount** (8): how far the flat areas are smoothed. **Edge Blur Reduction** (100): how
  much less near edges.
- **Diffusion Amount** (1): each pixel borrows from a neighbour up to this many pixels away, to
  break up what is left of the steps. **Edge Diffusion Reduction** (100), **Diffusion Reseed
  Interval** (1: changes every frame; 0: never changes).
- **Noise Amount** (1): a fine dither, in levels. **Edge Noise Reduction** (100), **Noise
  Reseed Interval** (1).

CelFX publishes only the control names, so the rule, the ranges and the starting values are our
own, written down in document 21. Our pictures will not be CelFX's.

The check, `verification/D-456_deband_table.md` (157 of 157), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-456 pictures/`.

## What to check

1. **The banding.** `1_banded.png` is the smooth sky (`0_smooth.png`) cut into bands, with a
   dark hill.
2. **As added.** `2_as_added.png`: the bands mostly gone, the sky smooth again, the hill's edge
   still hard.
3. **Strong.** `3_strong.png` (blur 30, diffusion 4, noise 4): smoother still, with a visible
   fine grain; the hill's edge still hard.
4. **The edges.** `4_edge_only.png`: only the hill's outline in white, the bands not counted as
   edges. `5_selected_edge.png`: the same outline in red over the result.
5. **In the app.** Put Deband (Blur & Sharpen) on a picture with a soft gradient: the steps
   soften. Raise Blur Amount: smoother. Set Blur, Diffusion and Noise to 0: the picture is back as
   it was. Try View Edge Only and raise Edge Threshold: fewer edges.
6. **Out of range.** Type 300 in Edge Threshold: it is refused with a sentence saying the range.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: how strong the starting values are (we think as added
should hide gentle banding without softening detail), and the noise being one grey dither, not
coloured.

## Speed

Measured on a quiet machine
(`verification/B-336_deband_timing_table.md`). Three Deband on a 1920 by 1080 shot, card / processor,
milliseconds a frame played again: the shot alone 11.4 / 20.3 ms a frame, as added 52.4 / 234.7, blur 30, diffusion 4, noise 4 75.1 / 306.3; over Target P2 (4 ms or less added on the card): about 13.7 to 21.2 ms a layer on the card; on the processor about 71 to 95 ms a layer.
