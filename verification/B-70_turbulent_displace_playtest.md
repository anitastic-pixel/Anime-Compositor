# B-70: Turbulent Displace, by hand

Built on 2026-09-26 against D-127, which you accepted the same day as the fifth of the second
batch of ten ("go ahead with the ten").

The generated halves are `verification/B-70_turbulent_displace_table.md`, 106 of 106, which
renders every FX-TURB case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Turbulent Displace card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-70a proposal/` shows what to expect.

## Before you start

Open a project with a drawn layer that has clear straight lines, a building, a fence or a
character's outline, at least a second long. Select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Turbulent Displace** in **Add effect…**. It goes to the end of the
   stack, and the card shows **Amount** 10, **Size** 60, **Complexity** 2, **Evolution** 0,
   **Speed** 20, **Seed** 0 and **Edges** Transparent. Play: the drawing wobbles gently, straight
   lines bending into slow waves.
2. **Heat haze.** Amount 4, Size 15, Complexity 4, Speed 60: a fine, quick shimmer, like air over
   a hot road.
3. **Size.** Size 200: broad, lazy swells. Size 5: a fine ripple that roughens the lines.
4. **Still.** Speed 0: the warp holds still. Change Evolution by 360: it moves by one wave.
5. **Seed.** Seed 12: a different wobble. Back to 0: the first one again, exactly.
6. **Edges.** With Transparent, the drawing's edge may be pushed out past its old box or eaten
   in a little. With Repeat Edge Pixels, the edge smears its own colour instead and never goes
   past its box.
7. **Moves with the layer.** Move the layer: the wobble goes with it, it does not stay fixed on
   the screen.
8. **Amount 0.** The drawing is as it was.
9. **Out of range.** Type 1001 in Amount: it is refused with a sentence saying it runs from 0 to
   1000, and the card keeps its old number.
10. **Keyed.** Key Amount at 0 and at 20 at a later frame. Play: the wobble grows from nothing.
11. **Draft.** Press **Draft**: the picture is smaller, but the waves look the same size
    relative to the drawing.
12. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there, and the wobble is the same as before.

## Known limits, on purpose

- One kind of warp, a push in a noise direction: no twist, bulge or up-and-down-only kinds, and
  no pinning of the corners.
- At the start settings a wave is wider than a small drawing, so a small drawing mostly slides
  whole. On a full frame it reads as a wobble.
- With Transparent edges, a line touching the drawing's border can be eaten in.
- In a draft, a size that would fall under one pixel is held at one.
- The name is After Effects' own and is flagged for the pre-launch naming review. It is not
  claimed to match After Effects.

## What to answer

"works", or which step number did something else and what it did.
