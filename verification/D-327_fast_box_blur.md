# D-327 / B-208: Fast Box Blur

From P-26. Tutorials 2 and 3 blur with After Effects' **Fast Box Blur**. The replays had been standing Gaussian Blur in for it. Tutorial 2 then raises Exposure 17 to 20 stops, which lit the Gaussian's faint far edge white and made the puddle about half as wide again as After Effects'.

## Where the rule comes from (no After Effects needed)

After Effects' manual lists Fast Box Blur's settings: Blur Radius, Iterations, Blur Dimensions and Repeat Edge Pixels. It is a box blur laid on several times (3 by default), so nothing lies further out than iterations times the radius. A Gaussian has no such end: its tail fades but never stops.

## What changed

- **Fast Box Blur** is a new effect in the Blur group, with After Effects' settings:
  - **Blur Radius**: 0 to 500 pixels.
  - **Iterations**: 1 to 50; 3 to start, as in After Effects.
  - **Blur Dimensions**: Horizontal and Vertical, Horizontal, or Vertical.
  - **Edges**: Transparent, or Repeat Edge Pixels.
- It starts at radius 0, which changes nothing.
- It adds to the app only. **No existing picture or project changes.**

## Checks (cargo test)

Everything is in `verification/D-327_fast_box_blur_table.md`, **56 of 56 pass**. Among them:

| Check | Result |
|---|---|
| FX-FASTBOX-001 to 011: every pixel matches values written by `tools/fast_box_blur_reference.py` before the code existed | pass, tolerance 2e-5 |
| Iterations 0, radius 501 and Blur Dimensions "Both" are refused with a sentence | pass |
| Saved and opened again, nothing changes | pass, 6 files |
| A half-size draft halves the radius | pass |
| The preview draws the same picture | 0 to 1 levels of 255 apart |
| Cut into tiles of 1 or 64 pixels, the frame is identical | pass |

The full crate and app suites were run afterwards (see the commit).

## Pictures

In `verification/D-327 pictures/`, a white square 24 pixels across, over black:

- `square.png`: no blur.
- `1_gaussian_blurriness_20.png`: Gaussian Blur, Blurriness 20.
- `2_fast_box_radius_6.png`: Fast Box Blur, radius 6, iterations 3. About as soft.
- `3_gaussian_then_exposure_17.png`: the Gaussian with Exposure +17 after it. Its faint tail lights up into a big rounded blob.
- `4_fast_box_then_exposure_17.png`: Fast Box Blur with Exposure +17 after it. The light stops 18 pixels out (radius 6 × 3 iterations), so the square only grows a little and keeps its corners.

From the app's test copy (not your app):

- `5_app_fast_box_blur.png`: a white square with Fast Box Blur, radius 20, iterations 3. Effect controls shows Blur Radius, Iterations, Blur Dimensions and Edges.
- `6_app_horizontal.png`: the same with Blur Dimensions set to **Horizontal**. The square spreads sideways only.

## Known limit

Radius 500 with 50 iterations reaches 25,000 pixels. With transparent edges the layer grows that much on every side, which can run out of memory. A cap can be decided later if anyone needs settings that high.

## For the owner to try

1. Add **Fast Box Blur** to a white solid and set Blur Radius 20. It should blur softly, much like Gaussian Blur.
2. Add Exposure +17 after it in a 32 bpc (Float) composition. The glow should stop at a clear edge. Do the same with Gaussian Blur and compare: Gaussian Blur spreads much further.
3. Set Blur Dimensions to Horizontal. The blur should go sideways only.

**Awaiting the owner's playtest.**
