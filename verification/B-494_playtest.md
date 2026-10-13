# B-494: Particle Playground's property mappers

Built on 2026-10-12 as D-614, the second step of Particle Playground (D-613, B-493), under your
/loop request. Particle Playground gains its two **property mappers**, after what After Effects'
Persistent and Ephemeral Property Mappers are for.

Each mapper reads another layer's red, green and blue under every particle and turns each channel
into one of the particle's properties: its red, green or blue, opacity, scale, angle, place
across or down, speed across or down, or force across or down, between a Min and a Max.

- **Persistent Property Mapper**: sets the property and the particle keeps it.
- **Ephemeral Property Mapper**: works the property through an operator (set, add, difference,
  subtract, multiply, min, max) for the moment only; the particle goes back afterwards.
- Each has **Affects**, as the other forces.

A project saved before this step opens as it was and is saved as it was.

Adobe publishes no formula and no settings when added, so the rules and those settings are ours,
written down in document 21 and in `tools/particle_mapper_reference.py`.

The check, `verification/D-614_particle_mapper_table.md` (CHECKS), holds every pixel to numbers
worked out by a separate program before the code existed, and every particle's numbers to within
a ten-millionth. The pictures are in `verification/D-614 pictures/`, a grid of dots over the
street picture of D-506, the street itself the map.

## What to check

1. **Plain grid.** `1_grid_unmapped.png`: white dots in an even grid over the whole street picture, the picture showing between them.
2. **Coloured by the street.** `2_grid_coloured_by_the_street.png`: the same grid, each dot the colour of the street under it, so the dots almost vanish into the picture (look closely at the building edges: they are slightly beaded).
3. **Halftone.** `3_halftone_scale_by_red.png`: dot sizes follow the red under them: small dots over the blue sky and dark road, big dots over the red and yellow buildings and the white road marks.
4. **Fountain painted.** `4_fountain_painted_by_the_street.png`: a fountain of dots rising from the middle, each dot taking the colour of the street where it is, so it shows only as small coloured bumps at the building tops and edges.
5. **Pushed sideways.** `5_ephemeral_push_sideways.png`: the grid moved to the right by the map for the moment: an empty strip at the left edge, dots bunched toward the right.
6. **In the app.** On a Particle Playground with a Grid, set Persistent Map Layer to a picture
   and its red, green and blue to Red, Green and Blue: each dot takes the colour under it.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when the mappers are added (no layer named,
every channel mapped to nothing, Min 0 and Max 1, the operator Set); that a channel is read as the 0-to-1 value you see on screen; that a dot whose scale or
opacity is mapped to 0 or below is not drawn; that a mapped value lying within a ten-thousandth of a level of an 8-bit picture is put back on
that level (the map is held at a precision that moves it slightly).
Not built, and why: Mass, Friction, Angular Velocity, Torque, Gradient Velocity, separate X and Y
Scale and Lifespan (nothing in the playground has mass, friction or spin, or dies, so they would
change nothing yet); Character and Font Size (no text particles); maps read as they move (they are
read as they stand at the frame drawn).

## Speed

PROVISIONAL, measured while other tests were running (`verification/B-494_particle_mapper_timing_table.md`): with both mappers on, frames took about 120 to 135 ms against about 120 to 150 ms for the Playground with no mapper, so the mappers add nothing that stands out from the noise. A quiet re-measure is owed.
