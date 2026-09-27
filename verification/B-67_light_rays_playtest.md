# B-67: Light Rays, by hand

Built on 2026-09-26 against D-124, which you accepted the same day as the second of the second
batch of ten ("go ahead with the ten").

The generated halves are `verification/B-67_light_rays_table.md`, 92 of 92, which renders every
FX-RAYS case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Light Rays card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-67a proposal/` shows what to expect.

## Before you start

Open a project with a drawn layer that has some bright parts, a sun, a lamp or a white
highlight, select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Light Rays** in **Add effect…**. It goes to the end of the stack, and
   the card shows **Centre** 50, 50, **Length** 50, **Threshold** 70, **Intensity** 1 and
   **Colour** #ffffff. The bright parts stream outward away from the middle, fading, as in the
   picture's second panel. Dark parts make no rays.
2. **Longer and warmer.** Length 100, Intensity 2, colour #ffc070: long orange streaks that
   reach the layer's edges, as in the third panel.
3. **Off centre.** Centre 20, 15: the rays run away from the top left, as in the fourth panel.
4. **Threshold.** Threshold 100: only pure white makes rays. Threshold 40: much more of the
   drawing shines.
5. **Intensity 0.** The drawing is as it was.
6. **Edges.** The rays stop at the layer's own box; they do not spill past it.
7. **Out of range.** Type 11 in Intensity: it is refused with a sentence saying it runs from 0
   to 10, and the card keeps its old number.
8. **Keyed.** Key Length at 0 and at 100 at a later frame. Scrub between: the rays grow
   smoothly.
9. **Draft.** Press **Draft**: the picture is smaller, but the rays reach as far relative to the
   drawing.
10. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- A black colour still adds covering, so it lays a dark smear rather than doing nothing.
- The rays stop at the layer's box. Put Light Rays on a layer that covers the frame, or after an
  effect that grows the layer, to let them travel further.
- It is modelled on After Effects' CC Light Rays in spirit and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
