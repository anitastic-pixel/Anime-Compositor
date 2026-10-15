# B-493: Particle Playground

Built on 2026-10-12 as D-613, the particle engine's third effect (P0-6), under your /loop request.
**Particle Playground** (Simulation) is a new effect, after what After Effects' Particle
Playground is for. Its property mappers are the next step, B-494.

It runs on Particle Space's engine. Particles are drawn over a clear background in the order they
were born, so whatever is under the layer shows through around them.

- **Cannon**: shoots dots from a point, with a barrel radius, a rate, a direction and speed with
  their spreads, a colour and a radius.
- **Grid**: lays dots out in rows and columns, with a place, width, height, colour and radius.
- **Layer Exploder**: cuts a layer into squares that fly apart.
- **Particle Exploder**: splits a particle into children that fly out from it.
- **Layer Map**: each particle shows a piece of another layer instead of a dot.
- **Gravity**: a pull with a force, a spread and a direction.
- **Wall**: one of the layer's masks, which particles bounce off.
- Each force has **Affects** (which particles it takes: from the Cannon, the Grid or the Layer
  Exploder, by a selection map, or by age). **Auto-Orient Rotation**, **Random Seed**.

Adobe publishes no formula and no settings when added, so the rules and those settings are ours,
written down in document 21 and in `tools/particle_playground_reference.py`.

The check, `verification/D-613_particle_playground_table.md` (338 of 338 checks pass), holds every pixel to numbers
worked out by a separate program before the code existed, and every particle's place and speed to
within a ten-millionth. The pictures are in `verification/D-613 pictures/`, drawn over the street
picture of D-506.

## What to check

1. **As added.** `1_as_added_frame_24.png`: a thin trail of small red dots rising from a point above the left buildings and drifting right as it climbs. You should see a cannon's stream, no other change to the street.
2. **Fountain and grid.** `2_fountain_and_grid.png`: three rows of yellow dots evenly spaced across the sky (the Grid), and a pale blue fountain of dots spraying up from the right-hand buildings and falling back.
3. **Layer exploded.** `3_layer_exploded.png`: the street picture cut into small squares that have flown apart and are falling, sky pieces above and building and road pieces below, tumbling at different angles.
4. **Fireworks.** `4_fireworks.png`: small pale gold sparks scattered over the sky, a shell's children spreading out from where it burst; the street below is untouched.
5. **Wall box.** `5_wall_box.png`: green dots bouncing around inside an invisible box in the middle of the picture: none escape past its edges, so the dots outline a rough rectangle. The mask itself is not drawn.
6. **In the app.** Add Particle Playground to a layer over a picture and play. Draw a mask on the
   layer, **switch the mask on**, and set Wall Mask to it: the dots bounce off its edges. (A mask
   that is switched off is no wall.) Set Explode Layer to a layer and Cannon rate to 0: that
   layer flies apart in squares.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when the effect is added; that nothing dies
of age; that a whole Grid is born every frame (After Effects' grid is continuous); that the Layer
Exploder reads its layer once, at the layer's first frame.
Not built, and why: **Repel** (every particle would push every other at every step, so a particle
could no longer be worked out from its own birth alone); text particles (each would need its own
text drawing); a child exploding again; the Layer Map's time offsets; field rendering; masks and
maps read as they move (they are read as they stand at the frame drawn); a graphics-card path (it
draws on the processor only).

## Speed

PROVISIONAL (three other tests were running): about 50 to 70 ms a frame on the processor against about 37 to 47 ms with no effect, roughly 10 to 25 ms for the Playground. Table: `verification/B-493_particle_playground_timing_table.md`.
