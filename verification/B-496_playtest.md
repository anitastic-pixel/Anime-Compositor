# B-496: Particle Grid

Built on 2026-10-12 as D-616, the fifth step of the particle engine (P0-6), under your /loop
request. **Particle Grid** (Simulation) is a new effect, the parked pick in
`docs/effects/PLUGINS.md` section 2.2: particles that stay put, after what Red Giant's Trapcode
Form is for. "Particle Grid" is our own name.

It runs on Particle Space's engine and draws with Particle Emitter's camera and particle shapes.
Particles are drawn over a clear background, through the composition camera when there is one, so
whatever is under the layer shows through around them.

- **Base Form**: a box grid of points, or layered spheres (a ball of rings, shells inside).
- **Form Size X/Y/Z**, **Particles in X/Y/Z**, **Form Center** and **Form Center Z**,
  **Form Rotation X/Y/Z**: how big the form is, how many points, where it sits and how it is turned.
- **Size**, **Size Random**, **Opacity**, **Opacity Random**, **Sphere Feather**, **Color**.
- **Disperse**: each point nudged its own way. **Twist**: the bottom turned against the top.
- **Displace**, **Fractal Scale**, **Flow X/Y/Z**, **Flow Evolution**, **Evolution**: a rippling
  field that pushes the points, drifting and changing over time. **Affect Size** and
  **Affect Opacity**: the field also grows, shrinks and fades them.
- **Color Map**, **Size Map**, **Displace Map** with **Map Displace**: another layer colours,
  sizes or pushes the points towards the camera (bright pushes towards it).
- **Particle Type**: Sphere, Glow Sphere, Star, Cloudlet. **Transfer Mode**: Normal, Add, Screen.
  **Random Seed**.

Red Giant publishes no formula and no settings when added, so the rules and those settings are
ours, written down in document 21 and in `tools/particle_grid_reference.py`.

The check, `verification/D-616_particle_grid_table.md` (244 of 244), holds every pixel to numbers
worked out by a separate program before the code existed, and every point's place, size and
opacity to within a ten-millionth. The pictures are in `verification/D-616 pictures/`, drawn over
the street picture of D-506.

## What to check

1. **As added.** `1_as_added.png`: an even grid of small white dots over the whole street picture.
2. **Rippling field.** `2_rippling_field.png`: fine pale-blue dots rippling in soft waves across the buildings, the picture showing through.
3. **Layered ball.** `3_layered_ball_turning.png`: warm yellow dots on the shells of a ball, rings of them stacked inside each other, turned so the rings tilt toward you.
4. **Coloured by the street.** `4_coloured_by_the_street.png`: a fine grid whose dots each take the street’s colour under them, so it blends in; the building edges and road marks look slightly beaded.
5. **Pushed by the street.** `5_pushed_by_the_street.png`: dots pushed toward or away from you by how bright the street is under them, so the bright buildings and road marks seem to lean out in streaks of dots.
6. **Twisted stars.** `6_twisted_stars.png`: pale yellow stars in a twisting band from top middle down to the road, nearer ones bigger.
7. **Cloudlets.** `7_cloudlets.png`: faded warm cloud puffs in a grid over the street, soft and see-through.
8. **In the app.** Add Particle Grid to a layer over a picture. Raise Displace and Flow X and play:
   the dots ripple and the ripple drifts. Set Color Map to the layer underneath: each dot takes
   the colour under its home spot.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when the effect is added (a still grid 820 by
620 pixels of 41 by 31 white soft dots, 4 pixels across, round the middle, no maps); the map rule
(each map is stretched over the layer and read where the point sits on the form's front, so a
turned or rippled point keeps the colour of its home spot); and that PLUGINS.md's "park" note was
taken as covered by your effects-loop request, which names Particle Grid; and that a map’s opaque colour lying within a ten-thousandth of a level of an
8-bit picture is put back on that level, as for Particle Playground’s mappers.
Not built: points from a layer's shape, text, strings or a 3D model; reacting to sound; the
spherical field and the kaleidoscope; more than one level of ripple; quick maps along an axis;
colour randomness; streaks (the points do not move); lights and shadows; and a graphics-card path
(it draws on the processor only).

## Speed

Not yet known. The run (`verification/B-496_particle_grid_timing_table.md`) was made while other tests were using the machine so heavily that the shot with no effect was sometimes slower than shots with 100,000 points. A quiet re-measure is owed.
