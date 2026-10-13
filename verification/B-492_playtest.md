# B-492: Particle Plane

Built on 2026-10-12 as D-612, the third step of the particle engine (P0-6), under your /loop
request. **Particle Plane** (Generate) is a new effect: a flat particle emitter, after what
CycoreFX's CC Particle Systems II is for. "Particle Plane" is our own name.

It runs on Particle Space's engine, but everything stays in the layer's own flat plane: no
camera, no depth, no floor. Particles are drawn straight onto the layer, over a clear
background, so whatever is under the layer shows through around them.

- **Birth Rate**, **Longevity**: how many are born and how long each lives.
- **Producer Position** and **Radius X/Y**: where they are born, inside an oval.
- **Velocity**, **Inherit Velocity %**, **Gravity** (always pulls down), **Resistance**,
  **Direction** (0 up, 90 right), **Extra** and **Extra Angle**: how they are thrown and fly.
- **Animation**: eleven ways of throwing them: Explosive, Direction, Cone, Viscous, Twirl,
  Twirly, Vortex, Fire, Jet Sideways, Fractal Omni, Fractal Uni.
- **Particle Type**: Line, Faded Sphere, Shaded Sphere, Disc, Bubble, and a new **Star**.
- **Birth/Death Size**, **Size Variation**, **Max Opacity**.
- **Opacity Map**: Constant, Fade In, Fade Out, Fade In and Out, or Custom (five levels from
  birth to death).
- **Color Map** with **Birth/Death Color** and five custom colours, as Particle Space's.
- **Transfer Mode**: Composite, Screen, Add. **Random Seed**.

CycoreFX publishes no formula and no settings when added, so the rules and those settings are
ours, written down in document 21 and in `tools/particle_plane_reference.py`.

The check, `verification/D-612_particle_plane_table.md` (322 of 322), holds every pixel to
numbers worked out by a separate program before the code existed, every particle's position to
within a ten-millionth, and the graphics card's picture to within 1 level of the processor's.
The pictures are in `verification/D-612 pictures/`, drawn over the street picture of D-506.

## What to check

1. **As added.** `1_as_added_frame_24.png`: thin yellow-to-red streaks bursting out of the
   middle and falling a little.
2. **Fountain.** `2_fountain.png`: a column of small blue-white particles shot upward from the street, spreading as it rises.
3. **Star burst.** `3_star_burst.png`: yellow five-pointed stars bursting out from the middle,
   points up.
4. **Vortex.** `4_vortex.png`: soft spheres, pale at birth and turning gold as they age, swept round in a wide swirl over the whole street.
5. **Fire.** `5_fire.png`: a bright yellow flame rising, turning orange and red at the top.
6. **Jet Sideways.** `6_jet_sideways_lines.png`: two thick jets of streaks thrown out to the left and
   right, yellow in the middle turning orange-red towards the ends and drooping.
7. **In the app.** Add Particle Plane to a layer over a picture and play. Try each Animation and
   Particle Type; set Opacity Map to Fade Out and the particles fade as they age.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the name; the settings when the effect is added; that
Star is on Particle Plane only (Particle Space still refuses it, which is yours to change).
Not built: textured particles and Composite With Original on the Plane (it always draws over a
clear layer); CC Particle Systems II's Black Matte and Hold Particle Release (no source says what
they do beyond the name).

## Speed

`verification/B-492_particle_plane_timing_table.md`. PROVISIONAL (another lane's test was running): on the reference shot, Particle Plane as added on each of three layers adds about 8 ms a frame on the graphics card and about 7 on the processor; the busier shots (1000 and 2000 a second) drew faster than the Noise alone, which was not looked into. A quiet re-measure is owed.
