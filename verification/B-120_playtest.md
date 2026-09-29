# B-120: HSV Key, by hand

Built on 2026-09-28 against D-184, which you accepted as the sixth of the nine ("1, 2,3,5,6,7,8,9",
then "include 4 as well, just 1 thru 9"). It is OpenToonz's HSV Key, ported under its BSD licence
with four changes: it reads a pixel's own colour however see-through the pixel is, its hue window
wraps round the colour circle (a red window at 350 within 40 takes 10 too), saturation and value
are in percent, and it is added as a green-screen key.

The generated halves are `verification/B-120_hsv_key_table.md`, 67 of 67 checks passing, which
renders every FX-HSV case against the numbers written before the code and draws the pictures
below, and `verification/B-12b_state_fields_table.md`, which checks the HSV Key card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window.

## The pictures

`verification/B-120 pictures/`, each three times enlarged; the grey checkerboard is where pixels
were taken out:

- `before.png`: a figure in front of a green screen that darkens to its shadow at the floor. The
  shirt has a grey button, and green light has spilt down its left edge.
- `as_added.png`: HSV Key as it is added. The screen, its shadow and the spill are all gone; the
  head, the ink, the shirt and the button stay whole.
- `narrow.png`: hue 142 within 5, saturation 100 within 10, value 50 within 50. Only the pure
  screen and its shadow go; the paler spill on the shirt stays, green.
- `inverted.png`: the same with Invert on. Now the screen and its shadow are all that stay.

## Before you start

Import `verification/B-120 pictures/plate.png`, make a layer from it and press
**Full resolution**.

## What to check

1. **Adding it.** Pick **HSV Key** in **Add effect…**, under **Lines & Mattes**, next to
   Color Key. The card shows **Hue** 120, **Saturation** 60, **Value** 60, **Hue Range** 40,
   **Saturation Range** 40, **Value Range** 40 and **Invert** Off. The picture looks like
   `as_added.png`.
2. **Narrower.** Set Hue 142, Saturation 100, Value 50, Hue Range 5, Saturation Range 10 and
   Value Range 50. It looks like `narrow.png`: the spill comes back.
3. **Inverted.** Set Invert to On. It looks like `inverted.png`. Set it back to Off.
4. **Round the circle.** Set Hue to 350, Hue Range 40, Saturation 55 within 50 and Value 50
   within 50: the head goes (skin is an orange-red, near 25), and the green screen stays.
5. **Nothing.** Set every range to 0 and Invert Off: the plate is exactly as imported.
6. **Out of range.** Type 400 in Hue: it is refused with a sentence saying hue runs from 0 to
   360, and the card keeps its old number. Type 200 in Hue Range: refused, it runs to 180.
7. **Keyed.** Key Hue Range at 0 at the first frame and at 40 a few frames later, with the
   other settings as added. Scrub between: first nothing goes, then the spill down the shirt
   goes, then the screen and its shadow together, each all at once as the window reaches it.
8. **Draft.** Press **Draft**: the picture is smaller and keyed the same way.
9. **Undo and saved.** Ctrl+Z steps back each change, the Invert choice included. Save, close
   and open again: every setting and key is still there.

## Known limits, on purpose

- A pixel is either taken out whole or kept whole; there is no soft edge. Its outline follows
  the drawing's own pixels.
- It reads the colour as 8 bits, as the Color Key does, so two colours closer than one step
  of 255 are keyed alike.
- It is drawn on the processor, and costs little: on this machine a full 1920 by 1080 frame
  of the reference shot with HSV Key on all four layers took 57 to 61 ms, as added or inverted,
  against 49 to 54 ms with none (frame 100, three runs each, twice). Item 9 of the batch moves
  it to the graphics card.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
