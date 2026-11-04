# B-888 playtest: Color Grade's colour wheels you can drag (D-1008)

What it is: in the Lumetri tutorial (4:50) the shadows are pushed towards blue and the highlights
towards orange by dragging the dot in the middle of each colour wheel. Color Grade's wheels were
typed numbers only. Now they are three wheels you drag, each with a lightness slider under it.

## Steps

1. Open a project with a picture, such as a street or a face, and select its layer.
2. Effect > Color Grade. In Effect Controls, find the three wheels (Shadows, Midtones, Highlights)
   just above the Shadow Wheel numbers.
3. Drag the dot in the **Shadows** wheel down and to the right, towards blue.
4. Drag the dot in the **Highlights** wheel left, towards orange and yellow.
5. Start another drag on a wheel and press **Escape** before letting go.
6. Double-click a wheel.
7. Move the slider under **Midtones** to the left, then to the right.
8. Press Ctrl+Z after a drag.

## What you should see

- Each wheel's ring is coloured round its edge, with red up and to the left.
- After step 3 the dark parts of the picture turn blue while you drag. The further from the middle,
  the stronger the colour. The Shadow Wheel Hue and Amount numbers below follow the dot.
- After step 4 the bright parts turn warm.
- After step 5 the dot and the picture go back to where they were before that drag.
- After step 6 the dot goes back to the middle and that wheel's colour goes away.
- After step 7 the middle tones get darker, then lighter.
- After step 8 one undo puts back the whole drag, not one step of it.

## Pictures

`verification/B-888 pictures/wheels_panel.png`: the panel in a private copy of the app, after the
scripted page check below. The Shadows dot is pushed to blue (hue 221, amount 52), the Highlights
dot to yellow-orange, and the Midtones dot sits in the middle.

## Fixtures

- `verification/B-888_color_wheels_page_table.md`: every row "yes". It covers WHEEL-001 to 014
  run through the page's own code, then a drag, Escape, a double-click and the slider in the app.
- The picture is drawn by D-398's numbers, unchanged, so FX-GRADE2- (`verification/D-398_color_grade_2_table.md`)
  still passes.
