# B-294: Ellipse

Built on 2026-10-10 as D-415, under your /loop request: After Effects' Ellipse, in **Generate**.
It draws the glowing outline of an ellipse, over the layer or alone.

- **Center** (50, 50 per cent): where the ellipse sits.
- **Width**, **Height** (200 by 200 pixels): its size.
- **Thickness** (8 pixels): how thick the outline is.
- **Softness** (50): how soft its edges are; 0 is a hard edge.
- **Inside Color** (white): the colour along the middle of the outline.
- **Outside Color** (blue): the colour it fades to at its edges.
- **Composite On Original** (On): On draws the outline over the layer; Off draws it alone, the
  rest see-through.

The outline is drawn exactly as Beam draws its line, so the two look alike. Adobe's manual says
only that the effect "draws an ellipse"; how far each pixel lies from the outline is measured our
way (exact for a circle, very close for an ellipse).

The check, `verification/D-415_ellipse_table.md` (209 of 209), holds every pixel to numbers worked
out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-415 pictures/` (`town.png` is the layer). In these
pictures a see-through part shows as white or as your viewer's checker.

## What to check

1. **As added.** `2_as_added.png`: a glowing circle in the middle, white along its middle and blue
   at its edges, over the street.
2. **Neon, alone.** `3_neon_alone.png`: a wide orange oval with a pale middle, the rest
   see-through.
3. **Tall.** `4_tall_violet.png`: a thin tall oval, white fading to violet, over the street.
4. **Halo.** `5_halo.png`: a thick, very soft ring left of the middle.
5. **In the app.** Add Ellipse (Generate) to a layer: a glowing ring in the middle. Drag Center: it
   moves. Set Width 400 and Height 150: a wide oval. Raise Thickness to 40 and Softness to 100: a
   soft glow. Turn Composite On Original off: only the ring shows. Key Width from 50 to 600 and
   play: the ellipse stretches. Switch to Draft: the same ellipse, a little softer.
6. **Out of range.** Type 0 in Width: it is refused with a sentence saying it runs from 1 to 10000.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-294_ellipse_timing_table.md` (quiet): the reference shot with a moving Noise and an Ellipse on three layers, played
again, 19.6 ms a frame on the card as added against 13.8 for the Noise alone, and 19.5
for a wide soft one; on the processor 54.8 and 56.8 against 47.4.
