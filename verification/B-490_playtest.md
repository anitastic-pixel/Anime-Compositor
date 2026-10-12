# B-490: Particle Space

Built on 2026-10-12 as D-610, the first effect on the particle engine (P0-6), under your /loop
request: **Particle Space**, in **Generate** (there is no Simulation group), after what CycoreFX's
CC Particle World is for. Particles are born from a producer, thrown out, pulled by gravity, wind
and swirls, stopped or bounced by a floor, and drawn in real 3D: nearer ones larger and in front.
When the composition has a camera, they are drawn through it, so moving or dollying the camera
moves near particles more than far ones. With no composition camera, the effect's own camera is
used, and that one can also turn (the composition camera cannot turn, D-58).

Nothing is remembered from frame to frame: every frame works each particle out from its birth.
So any frame, drawn alone, scrubbed to or rendered out of order, is the same picture.

Distances are in the layer's widths, from its middle (Y down, Z away); sizes are hundredths of
its width; Birth Rate is hundreds of particles a second. The main settings:

- **Birth Rate** (2) and **Longevity** (1 second): how many are born and how long each lives.
- **Producer** position and radius: where they are born.
- **Animation** (Explosive): Explosive, Direction Axis, Cone Axis, Viscous, Twirl, Twirly, Vortex,
  Fire, Jet Sideways, Fractal Omni, Fractal Uni.
- **Velocity**, **Inherit Velocity**, **Gravity**, **Resistance**, **Direction Axis**, **Extra**,
  **Extra Angle**, **Gravity Vector**.
- **Floor** (None, Ice, Glue, Bounce) with its position, bounciness and spread.
- **Wind** and **Turbulence** (ours: CC Particle World has none, Trapcode Particular does).
- **Particle Type** (Line): Line, Faded Sphere, Shaded Sphere, Disc, Bubble; **Birth Size**,
  **Death Size**, **Size Variation**, **Max Opacity**, **Volume Shade**.
- **Color Map** (Birth to Death), the birth, death and five custom colours.
- **Transfer Mode** (Composite, Screen, Add) and **Composite on Original** (off: the particles
  alone; on: over the layer).
- The effect's own **Camera** (distance, three turns, FOV), **Depth Cue** (None, Fade, Fog) and
  the **Light** for the shaded kinds.

CycoreFX publishes no formula and no settings when added, so the rules and those settings are
ours, written down in document 21 and in `tools/particle_space_reference.py`.

The check, `verification/D-610_particle_space_table.md` (383 of 383), holds every pixel to
numbers worked out by a separate program before the code existed, holds every particle's place
and speed to that program's within 1e-7, and holds the graphics card's picture to within 1 level
of the processor's. The pictures are in `verification/D-610 pictures/`, drawn over the street
picture of D-506.

## What to check

1. **As added.** `1_as_added_frame_24.png`: yellow-to-red streaks flying out from the middle on a
   clear layer (the street hidden, because Composite on Original starts off).
2. **Fountain.** `2_fountain.png`: Direction Axis upward, gravity, blue faded spheres added
   together, over the street.
3. **Fire.** `3_fire.png`: rising, narrowing and flickering, white-yellow to red.
4. **Vortex.** `4_vortex.png`: shaded spheres swirling about the upright axis, the far ones faded
   by Depth Cue Fade, seen from a little above.
5. **Glue floor.** `5_glue_floor.png`: discs thrown up, falling and sticking where they land.
6. **Composition camera.** `6_composition_camera_dollied_in.png`: the same kind of cloud through a
   composition camera dollied close: near spheres very large, far ones small, those behind the
   camera gone.
7. **In the app.** Add Particle Space (Generate) to a solid and play. Switch Animation and
   Particle Type. Turn Composite on Original on.
8. **Depth through the comp camera.** Add a camera (Layer > camera, or the camera controls) and
   move it left and right, then dolly it in: near particles move more than far ones.
9. **Scrub.** Jump to frame 40, then play from 0 to 40: the same picture at frame 40.
10. **Out of range.** Type 101 in Birth Rate: it is refused with a sentence saying the range.
11. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; Birth Rate's unit; the light and
how the shaded kinds look; that the comp camera's own view is used whenever there is one; that
Wind and Turbulence are added. Left out, because no source says what they do: Black Matte, Hold
Particle Release and Grid & Guides. Textured particles come next (B-491).

## Speed

`verification/B-490_particle_space_timing_table.md`. PROVISIONAL (the machine was busy with other lanes' builds): on the reference shot, three layers each add about 13 to 15 ms a frame on the graphics card (62.6 ms against 22.6 for the noise alone, as added), and on the processor 79.1 against 57.7. A quiet re-measure is owed.
