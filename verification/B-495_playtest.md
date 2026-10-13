# B-495: Particle Emitter

Built on 2026-10-12 as D-615, the fourth step of the particle engine (P0-6), under your /loop
request. **Particle Emitter** (Simulation) is a new effect, pick #20 in `docs/effects/PLUGINS.md`:
a 3D particle emitter, after what Red Giant's Trapcode Particular is for. "Particle Emitter" is
our own name.

It runs on Particle Space's engine. Particles are drawn over a clear background, through the
composition camera when there is one, so whatever is under the layer shows through around them.

- **Emitter Type**: a point, a box, a ball or a grid of points, sized and turned in 3D.
- **Particles per Second**, **Direction** (every way, one way, two ways, round a disc, straight
  out or in) with **Direction Spread**, **Velocity** and its randomness.
- **Life** and **Life Random**: each particle lives its own time.
- **Size** and **Opacity**, each with its randomness and a five-step curve over the particle's life.
- **Set Color**: one colour, along a five-colour gradient over the life, or picked from the
  gradient. **Color Random**.
- **Gravity**, **Air Resistance**, **Wind X/Y/Z** and **Turbulence** (swirling air).
- **Floor**: where it is, and whether particles **bounce**, **slide** or **stick** on it.
- **Particle Type**: Sphere (with Sphere Feather), Glow Sphere, Star, Cloudlet, Streak.
- **Transfer Mode**: Normal, Add, Screen. **Random Seed**.

Red Giant publishes no formula and no settings when added, so the rules and those settings are
ours, written down in document 21 and in `tools/particle_emitter_reference.py`.

The check, `verification/D-615_particle_emitter_table.md` (270 of 270), holds every pixel to numbers
worked out by a separate program before the code existed, and every particle's place and speed to
within a ten-millionth. The pictures are in `verification/D-615 pictures/`, drawn over the street
picture of D-506.

## What to check

1. **As added.** `1_as_added_frame_24.png`: a small burst of white dots thrown every way from the middle of the picture, thicker at the centre.
2. **Fountain.** `2_fountain.png`: a fountain rising from the road, white near its foot and turning blue as the drops spread and fall.
3. **Sparks on a floor.** `3_sparks_on_a_floor.png`: red, orange and yellow sparks bursting from high in the middle and piling up along the road, where the floor is.
4. **Snow.** `4_snow.png`: soft white flakes of different sizes drifting down across the whole sky and buildings.
5. **Streaks outwards.** `5_streaks_outwards.png`: light blue streaks flying straight out from the middle, longer toward the edges.
6. **Grid of cloudlets.** `6_grid_of_cloudlets.png`: faded orange cloud puffs in a grid across the picture, gently blurred.
7. **Stars turned.** `7_stars_turned.png`: pale yellow stars of a few sizes in an arc over the street, each turned a little differently.
8. **In the app.** Add Particle Emitter to a layer over a picture and play. Try each Emitter
   Type and Direction; raise Gravity and set a Floor with Bounce and the particles bounce on it;
   set Set Color to Over Life and they change colour as they age.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when the effect is added (100 white soft
spheres a second, 4 pixels across, thrown every way from the middle at 100 pixels a second, living
3 seconds, no gravity, no floor); the gradient when added (white, yellow, orange, red, dark red,
like a spark cooling); and that PLUGINS.md's "only if the owner asks by name" was taken as covered
by your effects-loop request.
Not built: particles thrown by particles (trails, sparks off sparks); emitting from a layer,
light, text, mask or 3D model; textured particles (a picture as each particle); flocking and
fluid swirls; bouncing off layers (only a flat floor); spin; lights, shadows, motion blur and
depth of field; and a graphics-card path (it draws on the processor only).

## Speed

PROVISIONAL, measured while another test was running (`verification/B-495_particle_emitter_timing_table.md`): the busiest shot, 2000 sparks a second with turbulence and a floor, rises to about 220 to 255 ms a frame; lighter shots stay at about 120 to 145 ms. A quiet re-measure is owed.
