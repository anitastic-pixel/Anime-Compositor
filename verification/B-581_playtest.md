# B-581: Particle Emitter's second system (particles thrown by particles)

Built on 2026-10-13 as D-711 (P0-6), under your /loop request. Particle Emitter (B-495) gains a
second set of particles thrown by the first:

- **Aux Emit**: Off (as before), **Continuously** (each particle drops a trail as it goes) or
  **At Bounce** (each particle throws sparks where it bounces on the floor; needs Floor Action
  Bounce).
- **Aux Particles/sec** (for trails) and **Aux Particles/Bounce** (for sparks): how many.
- **Aux Life**: how long each thrown particle lives, in seconds.
- **Aux Velocity**: how fast they fly off, every way; **Aux Inherit Velocity**: how much of the
  thrower's own speed they keep (in percent).
- **Aux Size** and **Aux Opacity**, each going straight to **Aux Size at Death** and **Aux
  Opacity at Death** (percent of the start) over the thrown particle's life.
- **Aux Particle Type**: any of the five kinds (Sphere, Glow Sphere, Star, Cloudlet, Streak).
- **Aux Set Color**: **From Main** (the thrower's colour when it threw) or **Own** (**Aux
  Color**).

Files saved before this change open with the second system Off and save exactly as before.

The check, `verification/D-711_particle_emitter_aux_table.md` (173 of 173), holds every pixel and
every particle's place to numbers worked out by a separate program before the code existed, and
the graphics card's picture to the processor's (no difference). The pictures are in
`verification/D-711 pictures/`.

## What to check

1. **Comet trails.** `1_comet_trails.png`: a few glowing dots flying up and to the right, each
   with a fading trail of smaller glowing dots behind it.
2. **Sparks at the floor.** `2_sparks_at_the_floor.png`: pale blue balls falling and bouncing on
   the road, orange streak sparks thrown where they hit.
3. **Fireworks with fading trails.** `3_fireworks_fading_trails.png`: a burst of coloured
   particles, each leaving a trail that grows and fades as it ages.
4. **In the app.** Add Particle Emitter to a layer, set Aux Emit to Continuously and play: each
   particle leaves a trail. Set Floor Action to Bounce, raise Gravity, and set Aux Emit to At
   Bounce: sparks fly where the particles hit the floor. Try Aux Set Color Own with a different
   Aux Color.
5. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge (Red Giant publishes no formula): trails are dropped at an
even rate along each particle's path; sparks fly evenly in every direction from the bounce point;
size and opacity change in a straight line to their at-death levels; thrown particles feel
gravity, wind and the floor as the main ones do, but never throw particles of their own.

## Speed

`verification/B-581_particle_emitter_aux_timing_table.md`. Not usable: another lane's test
shared the graphics card, and the numbers contradict each other (the shot with no effect timed
slower than with effects). A quiet re-measure is owed.
