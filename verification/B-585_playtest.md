# B-585: Particle Playground's mappers set Mass

Built on 2026-10-13 as D-715 (P0-6), after your decision that day: fixture FX-PMAP-039 now
expects the word "repel" to be refused instead of "mass", and Particle Playground's Persistent and
Ephemeral Property Mappers can send a red, green or blue channel to **Mass**.

- **Mass** (1 to start with): a particle's own Force (X Force and Y Force, from a mapper) is
  divided by its mass, so a heavier particle is pushed less. Gravity pulls every mass alike, as
  real weight does. At a mass of 0 or below the Force does nothing.
- Static Friction holds a heavier particle more easily, since the push on it is smaller.
- Sparks from Fireworks keep their shell's mass.

Files saved before this change open and save exactly as before.

The check, `verification/D-715_particle_mapper_mass_table.md` (79 of 79), holds every pixel and
every listed particle to numbers worked out by a separate program before the code existed, and
the graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-715 pictures/`.

## What to check

1. **Mass 1.** `1_grid_pushed_down_mass_1.png`: a grid of white dots pushed down hard; by this
   frame each column is a long white streak.
2. **Mass 4.** `2_grid_pushed_down_mass_4.png`: the same push on dots four times as heavy: short
   separate dashes, each a quarter as long.
3. **Heavy where bright.** `3_fountain_blown_right_heavy_where_bright.png`: a fountain pushed to
   the right, the dots made heavier where the street is redder; the spray leans right.
4. **In the app.** Add Particle Playground to a layer, set Persistent Map Layer to a picture,
   Persistent Map Green To Y Force with Min and Max Green 100, and play: the dots are pushed down.
   Then set Persistent Map Red To Mass with Min Red 1 and Max Red 5: they are pushed less where
   the picture is red.
5. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge (Adobe publishes no formula): Mass divides only the
particle's own Force, not Gravity; a mass of 0 or below means no push at all.

Still not built: Gradient Velocity, Character and Font Size, and Repel (your gap, 2026-10-13).

## Speed

`verification/B-585_particle_mapper_mass_timing_table.md`, measured on a quiet machine (one run):
Mass costs nothing measurable.
