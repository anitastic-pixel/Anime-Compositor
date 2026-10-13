# B-498: Particle Grid on the graphics card

Built on 2026-10-13 as D-618, under your /loop request. **Particle Grid** (B-496) used to be
drawn only on the processor. Now the graphics card draws it in the viewer and preview too. Where
each point is and how it looks is still worked out on the processor, the same way as before.
Only the drawing moved to the card, through the same card step that draws Particle Space,
Particle Plane and Particle Emitter. Exports and the fixtures are still drawn on the processor,
as for every effect.

The check is `verification/D-616_particle_grid_table.md`, section "On the card (D-618)". It
draws every Particle Grid fixture file both ways at Full and Draft, and four setups on the
reference shot at frames 0, 100 and 239. Every comparison is within 1 level of 255. The table
passes 332 of 332 checks.

## What to check

1. **Same picture on the card.** Open a project with Particle Grid on a layer, with the graphics
   card on in the viewer. Play it, then switch the card off and back on at the same frame. The
   points should look the same both ways.
2. **Glowing points, added.** Set Particle Type to Glow Sphere and Transfer Mode to Add, and turn
   up Displace and Flow. The points should ripple and drift the same on the card as without it.
3. **A layer as a map.** Choose another layer as Color Map and as Displace Map. The points should
   take that layer's colours and be pushed by it, the same both ways.
4. **No warning.** The viewer should not say the frame was drawn on the processor instead of the
   card.

## Speed

PROVISIONAL: other lanes' tests ran through the whole measurement
(`verification/B-498_particle_grid_card_timing_table.md`). The card and the processor took
about the same time per frame in every shot, within the swings the other tests caused. With
100,000 points the card was a little faster at three of the four frames. A quiet re-measure is
owed.

Not done: working the points out on the card. That is still on the processor (see D-618).
