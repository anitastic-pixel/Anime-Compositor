# B-499: Particle Playground on the graphics card

Built on 2026-10-13 as D-619, under your /loop request. **Particle Playground** (B-493, with
its property mappers from B-494) used to be drawn only on the processor. Now the graphics card
draws it in the viewer and preview too. Where each particle is and how it looks is still worked
out on the processor, the same way as before. Only the drawing moved to the card, through the
same card step that draws the other particle effects. The card learned Playground's three looks:
round dots, pieces of the Explode layer, and copies of the Layer Map. Exports and the fixtures
are still drawn on the processor, as for every effect.

The checks are in two tables:

- `verification/D-613_particle_playground_table.md`, section "On the card (D-619)". It draws
  every Particle Playground fixture file both ways at Full and Draft, and five setups on the
  reference shot at frames 0, 100 and 239. The five setups are: as added; a fountain and a grid
  of dots; layer 2 exploded into turning pieces; fireworks; and copies of layer 2 shot from the
  cannon. Every comparison is within 1 level of 255. The table passes 338 of 338 checks.
- `verification/D-614_particle_mapper_table.md`, the same for every property-mapper fixture
  file. All 43 were identical (0 levels apart). The table passes 208 of 208 checks.

## What to check

1. **Same picture on the card.** Open a project with Particle Playground on a layer, with the
   graphics card on in the viewer. Play it, then switch the card off and back on at the same
   frame. The particles should look the same both ways.
2. **Exploded layer.** Set Explode Layer to another layer and turn Auto-Orient on. The layer
   should break into turning square pieces, the same on the card as without it.
3. **Layer Map.** Set Layer Map to another layer. Each particle should show a copy of that
   layer, the same both ways.
4. **Property mappers.** Use a Persistent or Ephemeral map layer on colour or scale. The
   particles should change the same way on the card as without it.
5. **No warning.** The viewer should not say the frame was drawn on the processor instead of the
   card.

## Speed

NOT USABLE YET: two other graphics-card tests ran on the same card through the whole
measurement (`verification/B-499_particle_playground_card_timing_table.md`). Even the shot
with no effect took twice as long on the card as on the processor, so these numbers say nothing
about Particle Playground. A quiet re-measure is owed.

Not done: working the particles out on the card. That is still on the processor (see D-619).
