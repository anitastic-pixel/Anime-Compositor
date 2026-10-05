# D-320 / B-201: Polar Coordinates, ellipse or circle

From D-308, approved by the owner on 2026-10-04, after P-26. In tutorial 1 (the anime ring), After Effects' Polar Coordinates bends a wide layer into a **round** ring. Ours followed D-201, which bends the layer into the ellipse that touches its sides, so on a 16:9 layer the ring came out squashed, wider than tall.

## What changed

- Polar Coordinates has a **Shape** row in Effect controls:
  - **Ellipse, fills the layer** is D-201's rule, as before.
  - **Circle** is round, centred on the layer's middle, with a radius of half the layer's shorter side, as After Effects draws it. Polar to Rect unrolls the same circle.
- A Polar Coordinates added from now on starts as a **Circle**.
- A file from before has no shape. It is the Ellipse, draws exactly as before, and is saved as it was. A circle is saved as `shape: "circle"`.
- Any other word is kept as written and reported, and the effect is left out of the frame.
- The preview card draws both shapes.

## Checks (cargo test)

All in `verification/D-320_polar_circle_table.md`, **40 of 40 pass**:

| Check | Expected | Got |
|---|---|---|
| FX-POLAR-016 to 021: circle both ways, half way, there and back, moved, and ellipse written out | every pixel as `Fixtures/polar_coordinates/expected_polar_circle.json`, within 2e-5 | pass, largest difference 2.5e-7 |
| FX-POLAR-022: shape "square" | read, kept, left out, `EFFECT_PARAMETER_INVALID` | pass |
| FX-POLAR-001 to 015, from before | unchanged, still pass | pass (B-136 table) |
| Old files saved | no `shape` line | pass |
| Shape set by the command, and undone | FX-POLAR-016's frame, then the old frame byte for byte | pass, byte for byte |
| "square" or "Circle" by the command | refused with a sentence, nothing changes | pass |
| The preview card, circle and ellipse | drawn on the card, within 1 level of 255 | pass, 0 of 255 on all six |
| A 16:9 picture | the circle's ring as far out across as down, within a pixel | pass: circle 32 across, 32 down; ellipse 57 across, 32 down |

## Pictures

In `verification/D-320 pictures/`. Pictures 1 to 3 are drawn by the test (B-201) from a 16:9 plate with dark streaks and a blue band, three times enlarged. Pictures 4 to 6 are the app's test copy, not your app: a new 640 by 360 composition, a white solid with Fractal Noise (Size 40, Scale Width 2000, Contrast 300), then Polar Coordinates added from the command, as tutorial 1 does (`target/p26/d320_steps.js`). Your Unsaved work and window were put back after the run.

| Picture | What it shows | Look for | Pass? |
|---|---|---|---|
| `1_plate.png` | The plate with no effect | Streaks and a blue band across a wide picture | |
| `2_ellipse_old_file.png` | A file from before D-320 (no shape): the ellipse, as before | The blue ring wider than tall, its streaks reaching the frame's sides | |
| `3_circle_new.png` | Shape Circle, as a new Polar Coordinates is | The blue ring round; the frame's far left and right clear | |
| `4_app_new_is_circle.png` | The app: Polar Coordinates just added | The noise rings round, a circle touching the top and bottom, the sides plain blue ground | |
| `5_app_set_to_ellipse.png` | The app: Shape set to Ellipse | The rings stretched into an ellipse filling the frame | |
| `6_app_undo_back_to_circle.png` | The app: Ctrl+Z | Round again, the same as picture 4 | |

The straight seam at the top of pictures 4 to 6 is where the noise's left and right edges meet; it is the same in After Effects unless the noise is made to repeat. The app read back shape "circle" after adding, no shape (the ellipse) after the change, and "circle" after undo.
