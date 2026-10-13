# B-583: Particle Playground's mappers set stretch, spin, lifespan and friction

Built on 2026-10-13 as D-713 (P0-6), under your /loop request. Particle Playground's Persistent
and Ephemeral Property Mappers (B-494) can now send a red, green or blue channel to seven more
properties:

- **X Scale** and **Y Scale**: a particle drawn stretched along its own width or height. A dot
  becomes an oval, turned with the particle; an exploded piece or a Layer Map copy is stretched
  the same way.
- **Angular Velocity** (degrees a second) and **Torque** (degrees a second, each second): the
  particle spins, and torque speeds the spin up or slows it down.
- **Lifespan** (seconds): a particle older than this disappears. A spark from Fireworks counts
  from its own birth.
- **Kinetic Friction**: slows a moving particle down, never making it go backwards.
- **Static Friction**: holds a still particle in place while gravity and the other forces
  pulling on it are this strong or weaker.

Files saved before this change open and save exactly as before.

The check, `verification/D-713_particle_mapper_physics_table.md` (133 of 133), holds every pixel
and every listed particle to numbers worked out by a separate program before the code existed,
and the graphics card's picture to the processor's. The pictures are in
`verification/D-713 pictures/`.

## What to check

1. **Dots stretched and turned.** `1_dots_stretched_and_turned_by_the_street.png`: the grid's
   dots are ovals. Over the dark road they stay small and round; elsewhere they are long ovals,
   turned more where the street is greener (upright in the sky).
2. **Short-lived where dark.** `2_fountain_short_lived_where_dark.png`: a fountain whose dots
   vanish soon after they pass over dark parts of the street, so the spray is thin.
3. **Kinetic friction.** `3_fountain_with_kinetic_friction.png`: the same fountain braked, so it
   stays low and bunched instead of spraying high.
4. **Static friction.** `4_grid_held_except_on_the_dark_road.png`: the grid under strong gravity
   stays put everywhere except over the dark road (and a few dark windows), where the dots fall
   as white streaks.
5. **In the app.** Add Particle Playground to a layer, set Persistent Map Layer to a picture and
   Persistent Map Red To X Scale with Max Red 3: the dots stretch where the picture is red. Try
   Lifespan (Max 1), Angular Velocity on an exploded layer, and Kinetic Friction (Max 100).
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge (Adobe publishes no formula): friction is in pixels a
second, each second; a dot narrower than 0.7 pixels is drawn 0.7 wide and fainter; a particle
dies at the first step at or past its lifespan; an Ephemeral lifespan only hides a particle
while it is drawn.

Still not built: Mass, Gradient Velocity, Character and Font Size, and Repel (your choice of
how is pending).

## Speed

`verification/B-583_particle_mapper_physics_timing_table.md`.
