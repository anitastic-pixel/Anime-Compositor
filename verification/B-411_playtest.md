# B-411: CC Environment

Built on 2026-10-11 as D-531, under your /loop request: CC Environment, under **Camera & Lens**,
our version of the CycoreFX effect of that name. It takes a picture of a whole surrounding (a
sky, a room) from another layer and shows it all round you, as a 3D program's sky does; the
layer it is put on only gives the shape it fills. It draws its own 3D look: the composition's
camera is not used, so you turn the view with the effect's own controls.

- **Environment**: the layer holding the surrounding picture (switch that layer off, as with
  any effect that reads another layer).
- **Mapping**: how that picture is laid out. **Spherical**: a 2 by 1 panorama, the usual
  "360" picture. **Probe**: a round "light probe" picture. **Vertical Cross**: six square
  faces of a cube, three across and four down.
- **Horizontal Pan, Tilt, Roll**: look right, look up, and lean the view.
- **Field of View** (40): how wide the view is; small is zoomed in, big is a wide lens.
- **Filter Environment** (on): smooths a Spherical panorama, four looks a pixel.

CycoreFX does not say exactly how it draws, so the projections' orientation and the settings
when added are ours, and Tilt, Roll and Field of View are ours, in place of the camera.

The check, `verification/D-531_cc_environment_table.md` (182 of 182), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the card to the
processor within 1 level. The pictures are in `verification/D-531 pictures/`: the street picture
is used as if it were a panorama, and the tiny coloured test pictures as the cross and probe.

## What to check

1. **All round.** `1_street_all_round.png`: the street's middle ahead, enlarged, the edges of
   the buildings bending a little, as a panorama does.
2. **Behind.** `2_turned_behind.png`: Horizontal Pan 180, the street's left and right edges
   meeting in the middle with no seam.
3. **Up.** `3_looking_up.png`: Tilt 45, almost all sky, one rooftop curving along the bottom.
4. **Wide, down.** `4_wide_looking_down.png`: a wide view looking down and leaning: the road
   bending into a little planet, the buildings fanning out.
5. **Cross.** `5_cross_corner.png`: the cube faces green (ahead) and blue (right) meeting in the
   middle, turned 45 degrees.
6. **Probe.** `6_probe.png`: the tiny probe's colours (blue, purple, brown, olive) blended smoothly across a
   wide view.
7. **In the app.** Import a 360 panorama, put it in a composition and switch it off; add CC
   Environment (Camera & Lens) to a solid above it and choose the panorama as Environment. Key
   Horizontal Pan from 0 to 360: the view turns all the way round.
8. **A missing layer.** Delete the environment layer: the solid shows unchanged and a warning
   says the layer is missing.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: a view of its own in place of the camera, and which way
each mapping faces.

## Speed

PROVISIONAL, measured while other builds ran: on the card the reference shot took about 51 to 57 ms a frame with CC Environment against about 15 ms without it, so the effect costs about 36 to 42 ms a frame at 1920 by 1080; a quiet re-measure would settle it (`verification/B-411_cc_environment_timing_table.md`)
