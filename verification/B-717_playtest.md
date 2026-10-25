# B-717 playtest: Particle Space fades its particles in and out (D-837)

What changed: Particle Space has a new choice, Opacity Map, like CC Particle World's. When it is set
to Fade In and Out, each particle fades in when it is born and fades out before it dies, instead of
popping on and off at full strength. A freshly added Particle Space starts at Fade In and Out,
because that is what CC Particle World does when you add it. Projects saved before this change
open with Opacity Map set to Constant, so they look exactly as they did.

## What to look at

The pictures in `verification/B-717 pictures/` each show three panels side by side:

1. After Effects' Opacity Map graph, as added, from a tutorial (Jake In Motion). The red line marks
   how far through its life the particle is. The number is the graph's height there, which is the
   particle's opacity. The Video Cave tutorial's graph is the same shape, and the number is the
   mean of the two.
2. Our app before the change: one particle, frame by frame.
3. Our app after the change: the same particle.

The particle lives 12 frames. The opacity shown on our two panels is measured from the picture.

| Picture | Life | After Effects | Before | After |
|---|---|---|---|---|
| fade_frame10_life0.06 | just born | 0.51 | 1.00 (differs) | 0.49 (matches) |
| fade_frame11_life0.15 | early | 0.84 | 1.00 (differs) | 0.83 (matches) |
| fade_frame12_life0.23 | nearly full | 0.98 | 1.00 (matches) | 0.96 (matches) |
| fade_frame15_life0.48 | middle | 1.00 | 1.00 (matches) | 1.00 (matches) |
| fade_frame19_life0.81 | late | 0.93 | 1.00 (close) | 0.91 (matches) |
| fade_frame20_life0.90 | fading | 0.70 | 1.00 (differs) | 0.68 (matches) |
| fade_frame21_life0.98 | about to die | 0.22 | 1.00 (differs) | 0.18 (close) |

Before is full strength all its life. After is within 0.03 of After Effects in six cases and 0.04
in the last.

## Try it in the app

1. Make a composition and add Simulation > Particle Space to a solid.
2. Play it. Each particle should fade in at birth and fade away before it vanishes, as in
   After Effects. Before this change, particles popped on and off.
3. In Effect Controls, set Opacity Map to Constant. The particles should now pop on and off at full
   strength, as before.
4. Open a project saved before this change that uses Particle Space. It opens with no warning,
   Opacity Map reads Constant, and it looks the same as before.

## Tests

- `tests/b717_particle_space_fade.rs` checks the fixtures, measures one particle's opacity at 10
  frames against the map, checks saving and opening (old projects open Constant and save back
  unchanged), refuses a misspelt map with a message naming it, and checks that the graphics card
  agrees with the processor. Its results are in `verification/B-717_particle_space_fade_table.md`:
  211 of 211 checks pass.
- `tests/b490_particle_space.rs` and `tests/b491_particle_texture.rs` rewrite the D-610 and D-611
  tables and pictures. The as-added pictures now fade, as intended.
