# B-517: Color Grade, fourth part

Built on 2026-10-13 under your effects loop request, decided as D-637. **Color Grade** (in
**Color Correction**) gains two more of Lumetri Color's pieces:

- **Sharpen** (-100 to 100): above 0 the edges get harder, below 0 the picture gets softer,
  -100 a gentle blur. It sits after Faded Film and before Vibrance, as in Lumetri.
- **A squarer vignette**: Vignette Roundness now goes below 0. 0 follows the frame's shape, as
  before; below 0 the darkening gets squarer, and at -100 it is a rectangle with soft edges.

Both start where they change nothing, so an older Color Grade looks the same as before. Both
can be keyed.

The check, `verification/D-637_color_grade_4_table.md` (115 of 115 checks pass), holds every
pixel to numbers worked out by a separate program before the code existed, and the graphics
card's picture to within 1 level of the processor's. The pictures are in
`verification/D-637 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **Sharpen 100.** `2_sharpen_100.png`: edges crisper than `1_before.png`, windows and
   outlines more defined; flat walls unchanged.
2. **Sharpen -100.** `3_sharpen_minus_100.png`: slightly softer than `1_before.png`, a light
   blur.
3. **Round vignette.** `4_vignette_round_0.png`: a dark vignette following the frame's oval;
   the corners much darker than the middle of each side.
4. **Squarer.** `5_vignette_round_minus_50.png`: the same, squarer; the sides darken nearly as
   much as the corners.
5. **Rectangle.** `6_vignette_round_minus_100.png`: a dark frame of even width all round, the
   corners no darker than the sides.
6. **In the app.** On a Color Grade, drag Sharpen up and down: harder, then softer. Set
   Vignette Amount to -3 and drag Roundness from 100 down to -100: circle, oval, then
   rectangle.
7. **Saved and opened again.** Save, close and open: the same Sharpen and Roundness.

Our own choices you may want to judge: how strong the sharpen is (it works on the finest
detail, one pixel wide), and how quickly the vignette squares off below 0.

**Timing.** On the reference shot with three noisy layers, the squarer vignette costs about
3 ms a layer on the graphics card. A Color Grade with Sharpen is worked out by the processor,
which is slower (about 270 ms a frame for three layers against 43 without). PROVISIONAL,
measured while other builds were running (`verification/B-517_color_grade_4_timing_table.md`).

## Not built (your answers of 2026-10-13)

- **Dragging** curve points and colour wheels (they are typed instead), and the key's
  **eyedropper**: these stay gaps, so Lumetri stays partial.
- Also recorded: **Vegas** stays masks-only (Image Contours and open masks stay gaps), and
  **Contour Paint**'s Freehand Flow Adjustments stay a gap.
