# D-325 / B-204: A blurred layer keeps its own outline, and Blur is named Gaussian Blur

From the owner's report of 2026-10-04: with Gaussian Blur on a masked white solid, the yellow outline of the selected layer slid up and to the left of the picture, and grew (picture 0, the owner's own screenshot).

## What changed

- The outline round a selected layer, and its handles, are the layer's own shape and mask, **before** its effects. A blur no longer stretches or moves them. (The cause: the outline was worked out from the blurred picture, which is larger than the layer by the blur's reach, and then placed as if it were the layer.)
- The effect is now called **Gaussian Blur** everywhere it was called Blur: Effect controls, the Effects list, Add effect, Undo/Redo lines and messages. Nothing about the picture or the file changes.

## Checks (cargo test)

All in `verification/D-325_blur_outline_table.md`, **7 of 7 pass**:

| Check | Expected | Got |
|---|---|---|
| The masked solid's outline, no effect | the composition's corners, 0,0 to 1920,1080 | pass |
| With Gaussian Blur 20 | the same corners (before the fix: 60 pixels wider on every side) | pass |
| The same in Draft | a quarter of it | pass |
| The mask's corners on screen | 744,369 and 1176,702 | pass |
| The blurred picture's middle and the mask's middle | the same point, 960,535 | pass |

The full crate and app suites were run afterwards (see the commit).

## Pictures

In `verification/D-325 pictures/`:

- `0_before_your_screenshot.png`: your report. The yellow outline sits up and to the left of the blurred rectangle.
- `1_app_masked_no_blur.png`: the app's test copy, not your app. A new 1280 by 720 composition, a white solid cut to a rectangle by a mask (300,150 to 800,500), selected (`target/p26/d325_steps.js`).
- `2_app_masked_blur_20_outline_kept.png`: the same with Gaussian Blur added and set to 20. The mask's yellow outline sits right on the blurred rectangle's middle edge, and Effect controls names it **Gaussian Blur**.

## For the owner to try

1. Make a white solid, draw a rectangle mask on it, add **Gaussian Blur** (it is under that name now) and set it to 20.
2. With the layer selected, the yellow outline should stay where the mask is, centred on the soft edge, not jump up and to the left.
3. Turn the blur up and down: the outline should not move.

**Awaiting the owner's playtest.**
