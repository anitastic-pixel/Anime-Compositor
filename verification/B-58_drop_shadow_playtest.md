# B-58: Drop Shadow, by hand

Built on 2026-09-26 against D-115, which you accepted the same day as the fifth of the batch of
ten ("proceed with said batch").

The generated halves are `verification/B-58_drop_shadow_table.md`, 92 of 92, which renders every
FX-SHADOW case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Drop Shadow card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-58a proposal/` shows what to expect.

## Before you start

Open any project with a drawn layer, for example `C:\Users\Andrew\Downloads\project.json`, select
the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Drop Shadow** in **Add effect…**. It goes to the end of the stack, and
   the card shows **Shadow Colour** black, **Opacity** 50, **Direction** 135, **Distance** 5 and
   **Softness** 0. A hard grey shadow appears down and to the right of the drawing, behind it,
   as in the picture's second panel. The drawing itself does not change.
2. **Distance and softness.** Distance 14, Softness 9: the shadow moves further out and goes
   soft, as in the third panel.
3. **Colour.** Shadow Colour #2040a0, Opacity 80, Direction 90, Distance 10, Softness 3: a blue
   shadow to the right, as in the fourth panel.
4. **Direction.** Direction 0 puts the shadow straight above, 90 right, 180 below, 270 left.
   -270 is the same as 90.
5. **Opacity 0.** The shadow is gone and the drawing is as it was.
6. **Nothing cut off.** Put the shadow at Distance 40: it still shows in full past the edge of
   the drawing's own box, and is cut only by the edge of the frame.
7. **Out of range.** Type 101 in Opacity: it is refused with a sentence saying it runs from 0 to
   100, and the card keeps its old number. Type -1 in Distance: refused, 0 to 1000.
8. **Keyed.** Key Direction at 0 and at 360 at a later frame. Scrub between: the shadow swings
   right round the drawing smoothly.
9. **Draft.** Press **Draft**: the picture is smaller, but the shadow sits in the same place
   relative to the drawing and is as soft.
10. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- The shadow is always behind the drawing; there is no "shadow only" switch.
- It is the shape of the whole layer, lines included; parts of the drawing cannot be left out.
- Directions over 360 go round again, so 450 is the same as 90.

## What to answer

"works", or which step number did something else and what it did.
