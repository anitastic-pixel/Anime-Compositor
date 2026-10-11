# B-410: CC Cylinder

Built on 2026-10-11 as D-530, under your /loop request: CC Cylinder, under **Camera & Lens**,
our version of the CycoreFX effect of that name. It wraps the layer round a cylinder, as a label
round a tin, and lights it. It draws its own 3D look: the composition's camera and lights are
not used.

- **Radius** (50): how wide the cylinder is, as a share of half the layer's width.
- **Position** and **Position Z**: where it stands; a bigger Z puts it farther away.
- **Rotation X, Y and Z**: turn it; Y spins it round its own axis, X tips it towards you,
  Z lays it on its side.
- **Render**: Full, or only the Outside, or only the Inside (the far wall, seen mirrored).
- **Light Intensity, Light Color, Light Height, Light Direction**: the light; a negative
  height puts it behind.
- **Ambient, Diffuse, Specular, Roughness, Metal**: how it is shaded. Metal 100 makes the
  highlight the layer's own colour, 0 the light's.

CycoreFX does not say exactly how it draws, so the lens, the light model and the settings
when added are ours. The ends of the cylinder are open, and its outline is not smoothed.

The check, `verification/D-530_cc_cylinder_table.md` (165 of 165), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the card to the
processor within 1 level. The pictures are in `verification/D-530 pictures/`, drawn on the
street picture.

## What to check

1. **As added.** `1_as_added.png`: the street wrapped round an upright cylinder in the middle,
   the middle facing you, lit from the upper left; its sides darken as they curve away, and the
   corners beside it are empty.
2. **Inside.** `2_inside.png`: only the far wall, the street seen from behind and so mirrored.
3. **Turned.** `3_turned_a_quarter.png`: spun a quarter turn, another part of the street now facing you.
4. **Tipped.** `4_tipped_towards_you.png`: the top tipped towards you; the inside shows through
   the open top.
5. **On its side, orange light.** `5_on_its_side_orange.png`: laid on its side, lit orange, the
   highlight orange (Metal 0).
6. **In the app.** Add CC Cylinder (Camera & Lens) to a layer. Key Rotation Y from 0 to 360 over
   a few seconds: the picture turns round like a spinning tin.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the lighting as added and how deep the lens makes it look.

## Speed

PROVISIONAL, measured while other builds ran: on the card the reference shot took about 96 ms a frame with CC Cylinder as added against about 87 ms without it, so the cylinder costs about 8 ms a frame or less; a quiet re-measure would settle it (`verification/B-410_cc_cylinder_timing_table.md`)
