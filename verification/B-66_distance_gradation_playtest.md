# B-66: Distance Gradation, by hand

Built on 2026-09-26 against D-123, which you accepted the same day as the first of the second
batch of ten ("go ahead with the ten").

The generated halves are `verification/B-66_distance_gradation_table.md`, 88 of 88, which
renders every FX-DISTGRAD case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Distance Gradation card sends
every setting the command reads. This sheet covers what the tables cannot: how it looks and
feels in the window. The picture in `verification/B-66a proposal/` shows what to expect.

## Before you start

Open any project with a drawn layer, for example `C:\Users\Andrew\Downloads\project.json`, select
the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Distance Gradation** in **Add effect…**. It goes to the end of the
   stack, and the card shows **Colour** #6450a0, **Width** 10, **Opacity** 50, **Invert** Off
   and **Blend** Multiply. A violet shade darkens the drawing just inside its outer edge, fading
   out ten pixels in, as in the picture's second panel.
2. **Wider and plainer.** Width 30, Opacity 80, Blend Normal, colour #3050c0: a broad blue rim
   fading to the drawing in the middle, as in the third panel.
3. **Invert.** Invert On, Blend Screen, colour #fff0b0, Opacity 90, Width 40: the middle glows
   pale yellow and the edge keeps its own colour, as in the fourth panel.
4. **Width 0 or Opacity 0.** The drawing is as it was.
5. **Holes.** A hole in the drawing, or an empty gap between two parts, shades its own edge as
   the outside edge does.
6. **Out of range.** Type 1001 in Width: it is refused with a sentence saying it runs from 0 to
   1000, and the card keeps its old number.
7. **Keyed.** Key Width at 0 and at 20 at a later frame. Scrub between: the shade creeps in
   smoothly.
8. **Draft.** Press **Draft**: the picture is smaller, but the shade reaches about as far in
   relative to the drawing.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- A drawing that fills its layer's whole box is shaded along the box's border too, since the
  layer's edge counts as the drawing's edge.
- A pixel less than half covered counts as outside, so a very soft edge is shaded fully.
- The shade follows straight-line distance, so it rounds off sharp inside corners; it does not
  follow the line art's direction.

## What to answer

"works", or which step number did something else and what it did.
