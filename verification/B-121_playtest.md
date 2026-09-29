# B-121: Paraffin, by hand

Built on 2026-09-28 against D-185, which you accepted as the seventh of the nine ("1, 2,3,5,6,7,8,9",
then "include 4 as well, just 1 thru 9"). It is anime's airbrushed "para" shade, written from
scratch: a colour washed over the figure from one side, strongest at the edge it comes from and
fading away on an airbrush curve. The wash is fitted to the figure's own drawn pixels, not to the
frame, so it rises from the feet whatever the size of the layer.

The generated halves are `verification/B-121_paraffin_table.md`, 89 of 89 checks passing, which
renders every FX-PARA case against the numbers written before the code and draws the pictures
below, and `verification/B-12b_state_fields_table.md`, which checks the Paraffin card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window.

## The pictures

`verification/B-121 pictures/`, each three times enlarged over a pale blue background:

- `before.png`: a figure on a clear cel, with hair, face, shirt, skirt and legs, each inked.
- `as_added.png`: Paraffin as it is added. The legs and skirt are darkened toward violet, the
  shirt less, and the face and hair not at all.
- `warm_from_above.png`: a warm orange screened down from above (direction 0, spread 50,
  opacity 70). The hair and face glow; the legs are untouched.
- `blue_from_right.png`: a blue shade multiplied in from the right (direction 90, spread 60,
  opacity 60). The figure's right side darkens; its left side is untouched.
- `soft_light_from_below.png`: violet in Soft Light from below at full spread and opacity. The
  whole figure is tinted, gently at the head; whites stay nearly white.

The background never changes in any of them: the wash stays inside the figure.

## Before you start

Import `verification/B-121 pictures/cel.png`, make a layer from it and press
**Full resolution**.

## What to check

1. **Adding it.** Pick **Paraffin** in **Add effect…**, under **Light & Glow**, after Exposure
   Flicker. The card shows **Direction** 180 on a dial, **Spread** 70, **Opacity** 50,
   **Blend** Multiply and **Colour** #6450a0. The figure looks like `as_added.png`.
2. **From above.** Set Direction 0, Spread 50, Opacity 70, Blend Screen and Colour #ffb070.
   It looks like `warm_from_above.png`.
3. **From the right.** Set Direction 90, Spread 60, Opacity 60, Blend Multiply and Colour
   #4060c0. It looks like `blue_from_right.png`.
4. **The blends.** Put it back to Direction 180 and Colour #6450a0, Spread 100 and Opacity 100,
   and step through the six blends. Normal paints flat violet at the feet; Multiply darkens;
   Screen and Add lighten; Overlay and Soft Light tint more gently, Soft Light the gentlest,
   and look like `soft_light_from_below.png`.
5. **Turning the dial.** Drag the Direction dial slowly round: the wash follows, always starting
   at the figure's edge on the side the dial points to.
6. **Nothing.** Set Spread to 0, or Opacity to 0: the cel is exactly as imported.
7. **Out of range.** Type 400 in Direction: it is refused with a sentence saying direction runs
   from 0 to 360, and the card keeps its old number. Type 150 in Spread: refused, it runs to 100.
8. **Keyed.** Key Direction at 0 at the first frame and at 180 four frames later. Scrub between:
   the wash swings from the head, round the figure's right side, to the feet.
9. **Moving the layer.** Move the layer across the frame: the wash moves with the figure and
   keeps its shape, because it is fitted to the figure, not the frame.
10. **Draft.** Press **Draft**: the picture is smaller and washed the same way.
11. **Undo and saved.** Ctrl+Z steps back each change, the Blend choice and the Colour included.
    Save, close and open again: every setting and key is still there.

## Known limits, on purpose

- The figure is everything drawn on the layer that is at least half opaque. Two characters on
  one layer are washed as one figure; put them on their own layers to wash each alone.
- The wash is a straight band across the figure, not a shape that follows the body's outline;
  mask the layer for a curved edge.
- A colour typed in capitals is kept in small letters when saved, as Distance Gradation does.
- It is drawn on the processor, and costs little: on this machine a full 1920 by 1080 frame
  of the reference shot with Paraffin on all four layers took 73 to 75 ms as added and 68 to
  79 ms in Soft Light, against 51 to 57 ms with none (frame 100, three runs each, twice; one
  first run with none took 68 ms). Item 9 of the batch moves it to the graphics card.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
