# B-497: Particle Emitter on the graphics card

Built on 2026-10-13 as D-617, under your /loop request. **Particle Emitter** (B-495) used to be
drawn only on the processor. Now the graphics card draws it in the viewer and preview too. Where
each particle is and how it looks is still worked out on the processor, the same way as before.
Only the drawing moved to the card, through the same card step that already draws Particle
Space and Particle Plane. Exports and the fixtures are still drawn on the processor, as for
every effect.

The check is `verification/D-615_particle_emitter_table.md`, section "On the card (D-617)". It
draws every Particle Emitter fixture file both ways at Full and Draft, and five setups on the
reference shot at frames 0, 100 and 239. Every comparison is within 1 level of 255. The table
passes 368 of 368 checks.

## What to check

1. **Same picture on the card.** Open a project with Particle Emitter on a layer, with the
   graphics card on in the viewer. Play it, then switch the card off and back on at the same
   frame. The particles should look the same both ways: soft spheres, glowing spheres, stars,
   cloudlets and streaks.
2. **Glowing spheres.** Set Particle Type to Glow Sphere and Transfer Mode to Add. Each particle
   should have a bright core with a soft glow around it, the same on the card as without it.
3. **No warning.** The viewer should not say the frame was drawn on the processor instead of the
   card.

## Speed

PROVISIONAL: two other tests ran through the whole measurement
(`verification/B-497_particle_emitter_card_timing_table.md`). With 1000 glowing spheres a
second, the card was about 10 ms a frame faster than the processor. With 2000 sparks, which
spend most of their time working the particles out on the processor, the card was not faster.
The shot with no effect was the slowest of all on the card, so these numbers are not to be
trusted until a quiet re-measure.

Not done: working the particles out on the card. That is still on the processor (see D-617).
