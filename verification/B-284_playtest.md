# B-284: Magnify

Built on 2026-10-09, decided as D-405 under your /loop request. **Magnify** (in **Distort**) is
our version of After Effects' Magnify: a round or square part of the layer enlarged, like a
magnifying glass held over it, and laid back over the layer.

The settings, with the values they start at:

- **Shape** (Circle): Circle or Square.
- **Center** (50, 50): per cent of the layer's width and height, where the lens sits.
- **Magnification** (200): per cent, how much bigger the picture is inside the lens. 100 to 1000.
- **Link** (None): None, Size to Magnification (the radius grows with the magnification) or
  Size & Feather to Magnification (the feather grows too).
- **Size** (100): the lens's radius in pixels (half the side for a square).
- **Feather** (0): pixels over which the lens's edge fades, inside the edge.
- **Opacity** (100): per cent, how strongly the lens shows.
- **Scaling** (Standard): Standard keeps the enlarged pixels as sharp blocks; Soft smooths them;
  Scatter breaks up the blocks' edges with noise.
- **Blending Mode** (Normal): how the lens is laid over the layer: None (the lens alone, the rest
  clear), Normal, Add, Multiply, Screen, Overlay or Soft Light.
- **Resize Layer** (Off): when on, the layer grows so a lens hanging off its edge is not cut off
  (not when Size is linked, as in After Effects).

The check, `verification/D-405_magnify_table.md` (238 of 238), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's (34 pictures identical, 42 one level apart). The pictures are
in `verification/D-405 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: a circle of radius 100 in the middle, the street inside it
   twice as big, in sharp 2 by 2 blocks. The circle reaches below the picture's bottom edge.
2. **Square.** `3_square_300.png`, size 60, magnification 300: a square 120 across, three times
   as big.
3. **Soft with a feather.** `4_soft_feather.png`, size 80, feather 30, Soft: smooth inside, the
   edge fading into the street with no hard rim.
4. **Scatter.** `5_scatter.png`, size 80, magnification 400, Scatter: the big blocks' edges
   broken up into grain.
5. **None.** `6_none.png`, size 80: only the lens, the rest clear (shown white here).
6. **Screen.** `7_screen.png`, size 80: the enlarged street lightening the street under it, so
   both show, paler.
7. **In the app.** Add Magnify to a text or picture layer and key Center across over two seconds:
   the lens slides over it. Set Center to 95, 50 and Resize Layer on: the lens is no longer cut
   off at the layer's edge.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Magnify as added costs about 1 ms a 1080p layer on the graphics card, and about
2.6 ms with a radius of 500, Scatter and Overlay; on the processor about 4 to 9 ms
(`verification/B-284_magnify_timing_table.md`, measured on a quiet machine).

## Not built

- Blending modes past the seven above (After Effects lists all its layer modes, Difference and
  the rest); a project asking for one is refused with a message naming the seven.
- Soft is our smooth sample (bilinear); After Effects' is a spline we have no formula for.
  Scatter uses Noise's own grain, not After Effects' (unknown).
- Ranges: Magnification 100 to 1000, Size and Feather 0 to 1000. After Effects' may go further.
- No handle on the viewer for Center or the radius yet; set them in Effect Controls.
- Motion Array's tutorial shapes the lens with a mask on the effect; effect masks are not part of
  this unit, so that step is a gap. TipTut's video tutorial was not watched.
- A lens far off the layer with Resize Layer on can grow the layer to many times its size, as
  Motion Tile can; there is no size guard on the graphics card yet.
