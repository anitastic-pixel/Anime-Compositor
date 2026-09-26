# B-54: Curves, by hand

Built on 2026-09-26 against D-111, which you accepted the same day as the first of the batch of
ten ("proceed with said batch").

The generated halves are `verification/B-54_curves_table.md`, 75 of 75, which renders every
FX-CURVES case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, 95 of 95, which checks the Curves card sends every
curve the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-54a proposal/` shows what to expect.

## Before you start

Open any project with a drawn layer, for example `C:\Users\Andrew\Downloads\project.json`, select
the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Curves** in **Add effect…**. It goes to the end of the stack, and the
   card shows four boxes, **Master**, **Red**, **Green** and **Blue**, each `0 0, 255 255`, the
   straight line. Nothing changes yet.
2. **An S curve.** Type `0 0, 64 40, 192 215, 255 255` in Master and press Enter. The darks get
   darker and the lights lighter, as in the picture's second panel.
3. **Warmer.** Put Master back to `0 0, 255 255`. Type `0 0, 128 170, 255 255` in Red and
   `0 0, 128 96, 255 255` in Blue: the drawing turns warmer, as in the third panel.
4. **Inverted.** Put Red and Blue back, and type `0 255, 255 0` in Master: every colour turns
   to its opposite, as in the fourth panel. Transparent parts stay transparent.
5. **Escape.** Type something in a box and press Escape: the box goes back to what it was and
   nothing changes.
6. **Refused.** Type `128 128` alone in Master: it is refused with a sentence saying a curve
   takes 2 to 16 points, and the box keeps its old points. Type `0 0, 255 300`: refused, 0 to
   255. Type `0 0, 200 50, 100 100, 255 255`: refused, the in values must go up.
7. **Draft.** Press **Draft**: the picture is smaller, but the colours are the same.
8. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every curve
   is still there, points and all.

## Known limits, on purpose

- The points are typed as numbers, in then out; there is no graph to drag. They are the numbers
  After Effects' own Curves graph shows when you hover a point.
- The curves are not keyable: a curve is the same on every frame.
- Between points the curve is a smooth spline, so a point pulled hard can overshoot a little;
  the result is always held inside 0 to 255.

## What to answer

"works", or which step number did something else and what it did.
