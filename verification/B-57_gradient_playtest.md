# B-57: Gradient, by hand

Built on 2026-09-26 against D-114, which you accepted the same day as the fourth of the batch of
ten ("proceed with said batch").

The generated halves are `verification/B-57_gradient_table.md`, 87 of 87, which renders every
FX-GRAD case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Gradient card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-57a proposal/` shows what to expect.

## Before you start

Open any project with a drawn layer, for example `C:\Users\Andrew\Downloads\project.json`, select
the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Gradient** in **Add effect…**. It goes to the end of the stack, and
   the card shows **Shape** Linear, **Start Point** 50, 0, **End Point** 50, 100, **Start
   Colour** white, **End Colour** #6450a0, **Start Opacity** 0, **End Opacity** 50 and
   **Blend** Multiply. The drawing darkens towards violet from top to bottom, as in the
   picture's second panel, and the background around it is not touched.
2. **Normal.** Blend Normal, both opacities 100: the drawing is painted over with the gradient,
   white at the top to violet at the bottom. Its outline shape and soft edges stay; nothing
   spills outside it.
3. **Screen and Add.** Blend Screen: everything lightens, nothing darkens. Blend Add: brighter
   still, most at the bottom.
4. **Radial.** Shape Radial, Start Point 50, 50, End Point 100, 50: the colour runs in rings
   out from the middle of the layer, as in the third panel.
5. **Across.** Shape Linear, Start Point 0, 50, End Point 100, 50, Blend Normal, colours
   #ff9060 and #4060ff, both opacities 60: compare with the fourth panel.
6. **Moving.** Move the layer: the gradient moves with it, it is not pinned to the screen.
7. **Out of range.** Type 101 in Start Opacity: it is refused with a sentence saying it runs
   from 0 to 100, and the card keeps its old number. Type 1001, 0 in Start Point: refused,
   -1000 to 1000.
8. **Keyed.** Key Start Point at 50, 0 and at 50, 100 at a later frame. Scrub between: the
   gradient squeezes towards the bottom smoothly, and on the last frame the whole drawing is the
   end colour.
9. **Draft.** Press **Draft**: the picture is smaller, but the gradient sits in the same place.
10. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every
    setting and key is still there.

## Known limits, on purpose

- There are two colours only, a start and an end; no colour stops in between.
- The points are shares of the layer's own width and height, not of the screen, so it follows
  the layer. After Effects' Gradient Ramp puts its points on the layer too.
- Add is not held back: a bright colour added to a bright drawing can go past white, which shows
  as white.
- There is no scatter setting to break up banding.

## What to answer

"works", or which step number did something else and what it did.
