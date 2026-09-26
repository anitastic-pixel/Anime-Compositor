# B-60: Rim Light, by hand

Built on 2026-09-26 against D-117, which you accepted the same day as the seventh of the batch
of ten ("proceed with said batch").

The generated halves are `verification/B-60_rim_light_table.md`, 101 of 101, which renders every
FX-RIM case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Rim Light card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-60a proposal/` shows what to expect.

## Before you start

Open any project with a drawn layer, for example `C:\Users\Andrew\Downloads\project.json`, select
the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Rim Light** in **Add effect…**. It goes to the end of the stack, and
   the card shows **Light Colour** white, **Light Direction** 45, **Width** 3, **Softness** 1,
   **Intensity** 100 and **Blend** Normal. A thin white rim appears along the drawing's upper
   right edge, as in the picture's second panel. Nothing is drawn outside the drawing.
2. **From the left, added.** Light Colour #ffb040, Light Direction 270, Width 6, Blend Add:
   a warm glowing rim on the left side, as in the third panel.
3. **From below, screened.** Light Colour #80c0ff, Light Direction 180, Width 4, Blend Screen,
   Intensity 80: a pale blue rim along the bottom, as in the fourth panel.
4. **Width.** Width 0: only the soft, half-covered edge pixels are lit. Larger widths reach
   further in.
5. **Intensity 0.** The drawing is as it was.
6. **Moving.** Move the layer: the rim stays on the same side of the drawing.
7. **Out of range.** Type 101 in Width: it is refused with a sentence saying it runs from 0 to
   100, and the card keeps its old number.
8. **Keyed.** Key Light Direction at 0 and at 360 at a later frame. Scrub between: the rim
   travels right round the drawing smoothly.
9. **Draft.** Press **Draft**: the picture is smaller, but the rim is on the same side and about
   as deep relative to the drawing.
10. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- The rim follows the outline of the whole layer. A separate part of the drawing, such as a dot
  beside the figure, gets its own rim too, and a gap between two parts counts as an edge.
- Add is not held back: a bright light added to a bright drawing can go past white, which shows
  as white.
- The light has one direction; there is no point light or falloff.

## What to answer

"works", or which step number did something else and what it did.
