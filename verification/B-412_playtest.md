# B-412: CC Sphere

Built on 2026-10-11 as D-532, under your /loop request: CC Sphere, under **Camera & Lens**,
our version of the CycoreFX effect of that name. It wraps the layer round a ball, as a map
round a globe, and lights it. It draws its own 3D look: the composition's camera and lights are
not used. Its light and shading are the same as CC Cylinder's.

- **Radius** (50): how big the ball is, as a share of half the layer's width.
- **Offset**: where its centre is.
- **Rotation X, Y and Z**: turn it; Y spins it like a globe, X tips its top towards you,
  Z lays it on its side.
- **Render**: Full, or only the Outside, or only the Inside (the far half, seen mirrored).
- **Light Intensity, Light Color, Light Height, Light Direction**: the light; a negative
  height puts it behind.
- **Ambient, Diffuse, Specular, Roughness, Metal**: how it is shaded. Metal 100 makes the
  highlight the layer's own colour, 0 the light's.

CycoreFX does not say exactly how it draws, so the lens, the map round the globe, the light model
and the settings when added are ours. Reflective (with its Reflection Map), Internal Shadows and
Transparency Falloff are not built yet. Like any world map on a globe, the picture is pinched at
the top and bottom poles, and the ball's edge is not smoothed.

The check, `verification/D-532_cc_sphere_table.md` (161 of 161), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the card to the
processor within 1 level. The pictures are in `verification/D-532 pictures/`, drawn on the
street picture.

## What to check

1. **As added.** `1_as_added.png`: the street wrapped round a ball in the middle, the street's
   middle facing you, lit from the upper left; the ball darkens towards its edge, and the
   corners round it are empty.
2. **Inside.** `2_inside.png`: only the far half, the street seen from behind and so mirrored.
3. **Turned.** `3_turned_a_quarter.png`: spun a quarter turn, the street's side edge now facing you.
4. **Tipped.** `4_tipped_towards_you.png`: the top tipped towards you; the street's top edge
   gathers at the pole near the top of the ball.
5. **On its side, orange light.** `5_on_its_side_orange.png`: laid on its side, lit orange, the
   highlight orange (Metal 0).
6. **In the app.** Add CC Sphere (Camera & Lens) to a layer. Key Rotation Y from 0 to 360 over
   a few seconds: the picture turns round like a spinning globe.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the map round the globe, the lighting as added and how
deep the lens makes it look.

## Speed

PROVISIONAL, measured while other builds ran: on the card the reference shot took about 35 to 48 ms a frame with CC Sphere against about 30 ms without it, so the effect costs about 5 to 18 ms a frame at 1920 by 1080, more the more of the frame the sphere covers; a quiet re-measure would settle it (`verification/B-412_cc_sphere_timing_table.md`)
