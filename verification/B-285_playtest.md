# B-285: Spherize

Built on 2026-10-10, decided as D-406 under your /loop request. **Spherize** (in **Distort**) is
our version of After Effects' Spherize: a round part of the layer wrapped onto a ball, so its
middle swells toward you and the picture crowds toward the ball's rim.

The settings, with the values they start at:

- **Radius** (100): the ball's radius in pixels, 0 to 2500 (After Effects' limit). 0 turns it off.
- **Center of Sphere** (50, 50): per cent of the layer's width and height, where the ball sits.

The check, `verification/D-406_spherize_table.md` (126 of 126), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's (15 comparisons identical, 24 one level apart). The pictures
are in `verification/D-406 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: a circle of radius 100 in the middle swells toward you; the
   houses in the middle look bigger, those near the circle's edge squeezed and bent round it.
   Outside the circle the street is untouched, and there is no step at the circle's edge.
2. **Bigger.** `3_radius_130.png`: the same, the ball nearly the street's height; the road's
   dashes bend down round the bottom of the ball.
3. **Off centre.** `4_off_centre.png`, radius 80 round 25, 40: the ball over the left houses.
4. **The whole picture.** `5_whole.png`, radius 2500: the ball far bigger than the street, so the
   whole street sits near its top and is simply enlarged about one and a half times (pi / 2) from
   the middle, smoothly, with no bending you can see.
5. **On the edge.** `6_edge.png`, radius 120 round 0, 50: half a ball on the left edge.
6. **In the app.** Add Spherize to a text or picture layer, key Radius from 0 to 200 over two
   seconds: the ball grows out of nothing. Key Center of Sphere across: the ball slides over it.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Spherize as added costs about 0.7 to 0.8 ms a 1080p layer on the graphics
card, and about 2 to 3 ms with a radius of 2500; on the processor about 5 to 9 ms
(`verification/B-285_spherize_timing_table.md`, measured on a quiet machine).

## Not built

- Adobe gives no formula, so how the picture is wrapped is ours (a half ball seen from in front,
  the same curve as our Fisheye at full strength); it may not match After Effects pixel for pixel.
- The starting radius of 100 is ours; Adobe's was not found.
- Only the outward swell; After Effects' Spherize has no pinch either.
- At Draft the radius shrinks with the picture, and the smooth sample is kept, where After
  Effects samples whole pixels in Draft.
- No handle on the viewer for the centre or the radius yet; set them in Effect Controls.
- Adobe's "curve a flat design" tutorial could not be read (the page has gone and the archived
  copy needs scripts), and no After Effects tutorial with numbers for Spherize was found.
