# B-33c: Glow, by hand

Built on 2026-09-25 against D-89, which you accepted the same day ("proceed").

The generated halves are `verification/B-33b_glow_table.md`, 126 of 126, which renders every
FX-GLOW case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Glow card sends every setting the
command reads. This sheet covers what the tables cannot: how it looks and feels in the window.
The pictures in `verification/B-33a proposal/` show what to expect.

## Before you start

The test drawing is `verification/B-33c_glow_drawing.png`: the flame from the proposal pictures,
on nothing. It has exactly five colours, so the thresholds below are exact:

| Part | Colour | Brightest channel | Glows at threshold |
| --- | --- | --- | --- |
| Core, light pink | `#ffe6fa` | 255 | 0 to 100 |
| Middle, light purple | `#aa5ae6` | 230 | 0 to 90 |
| Outer, dark purple | `#50238c` | 140 | 0 to 54 |
| Rock | `#1e2846` | 70 | 0 to 27 |
| Outline | `#0f0c14` | 20 | 0 to 7 |

Import it, make a layer from it and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Glow** in **Add effect…**. It goes to the bottom of the stack, and the
   card shows **Glow Threshold** 60, **Glow Radius** 10, **Glow Intensity** 1, **Tolerance** 0,
   **Glow Based On** Bright parts, **Glow Operation** Add and **Glow Colours** Its own colours:
   After Effects' own defaults.
2. **What glows.** The core and the middle glow and brighten; the core goes to white. The dark
   outer purple and the rock do not glow themselves, but the light from the middle falls on them.
   The light spreads past the flame's edge into the empty space round it.
3. **Threshold 54 and 55.** At 55 the outer purple does not glow; at 54 it does, and the whole
   flame gets a wider purple light.
4. **Threshold 90 and 91.** At 90 the middle glows; at 91 only the core does.
5. **Radius.** 0: nothing spreads, the glowing parts just get brighter. 50: a wide soft light.
   Drag it: the light grows and shrinks as you drag.
6. **Not cut off.** The flame's tip is 30 pixels below the top of its drawing. With radius 50,
   the light above the tip reaches past the drawing's top edge and fades out smoothly, not cut
   off in a straight line. If the drawing's top is at the frame's top, move the layer down first.
7. **Intensity.** 0: the drawing, unchanged. 2: twice as strong, more of it burns to white.
8. **Screen.** Set **Glow Operation** to Screen: the light is softer and nothing passes white.
   Set it back to Add.
9. **One colour.** Set **Glow Colours** to One colour: a white swatch and its hex appear, and
   the light is white. Type `40c0ff` in the box and press Enter: the light is blue, whatever
   part it comes from. Set it back to Its own colours.
10. **Chosen colours.** Set **Glow Based On** to Chosen colours: the colour boxes appear, and
    with none chosen nothing glows. Type `50238c` in the Add box: only the outer purple glows.
    Threshold does nothing now; Tolerance works as in Selective Colour Blur.
11. **Out of range.** Type 600 in Glow Radius: it is refused with a sentence saying the radius
    runs from 0 to 500, and the card keeps its old number.
12. **Keyed.** Click the key diamond beside Glow Radius at frame 0 with it at 0, go to a later
    frame and set it to 30. Scrub between: the light grows.
13. **Undo.** Press **Ctrl+Z**: the undo list says the Glow settings changed, and each step goes
    back one change.
14. **Saved.** Save, close and open again: every setting and the keys are still there.
15. **Your own drawings.** Try it on your purple flame. Start from the defaults and lower the
    threshold until the parts you want are lit, then set the radius and intensity.

## Known limits, on purpose

- It works in linear light, as every effect here does, so the same numbers give a softer,
  brighter-edged light than a default After Effects project.
- Not in D-89: Composite Original Behind and None, Glow Dimensions (horizontal or vertical
  only), A and B colours, and a soft edge to the threshold.

## What to answer

"works", or which step number did something else and what it did.
