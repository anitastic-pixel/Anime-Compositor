# B-71: Fractal Noise, by hand

Built on 2026-09-26 against D-128, which you accepted the same day as the sixth of the second
batch of ten ("go ahead with the ten").

The generated halves are `verification/B-71_fractal_noise_table.md`, 107 of 107, which renders
every FX-FRACTAL case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Fractal Noise card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-71a proposal/` shows what to expect.

## Before you start

Open a project with a white solid that covers the frame, and a drawn layer. Select the solid and
press **Full resolution**.

## What to check

1. **Adding it.** Pick **Fractal Noise** in **Add effect…** on the white solid. It goes to the
   end of the stack, and the card shows **Size** 100, **Complexity** 4, **Contrast** 100,
   **Brightness** 0, **Evolution** 0, **Speed** 0, **Seed** 0, **Opacity** 100, **Blend**
   Normal, **Dark Colour** #000000 and **Light Colour** #ffffff. The solid becomes soft grey
   clouds.
2. **The starting contrast.** Try Contrast 170. **Your call:** the agent suggests 170 as the
   starting value, because at 100 the clouds never reach black or white and look flat. Say
   whether 100 or 170 should be where a new Fractal Noise starts. Changing it changes only
   the starting number, not the rule or the checked numbers.
3. **Size and complexity.** Size 30: smaller clouds. Complexity 1: soft blobs with no detail.
   Complexity 8: fine, crisp detail on top.
4. **Brightness.** Brightness 30: lighter overall. Brightness -100: all dark.
5. **Moving.** Speed 20, then play: the clouds drift and change. Speed 0 again: still. Change
   Evolution by 360: they move on by one cloud.
6. **Seed.** Seed 7: different clouds. Back to 0: the first clouds again, exactly.
7. **Colours.** Dark #1e1a24, light #fff0b0: warm clouds on near-black.
8. **Over a drawing.** Add it to the drawn layer with Blend Multiply, Opacity 50: the drawing is
   shaded by cloud shadows. Screen lightens it with mist instead. Empty parts stay empty; soft
   line edges stay soft.
9. **Moves with the layer.** Move the drawn layer: the clouds go with it.
10. **Out of range.** Type 1001 in Contrast: it is refused with a sentence saying it runs from
    0 to 1000, and the card keeps its old number.
11. **Keyed.** Key Brightness at -100 and at 0 at a later frame. Play: the clouds fade up from
    dark.
12. **Draft.** Press **Draft**: the picture is smaller, but the clouds are the same size
    relative to the layer.
13. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there, and the clouds are the same as before.

## Known limits, on purpose

- One kind of noise, a little blockier in its fine detail than After Effects' many types; no
  turbulent or twisting look.
- Blend Add can brighten past white.
- In a draft, a size that would fall under one pixel is held at one.
- The name is After Effects' own and is flagged for the pre-launch naming review. It is not
  claimed to match After Effects.

## What to answer

"works", or which step number did something else and what it did, and 100 or 170 for step 2.
