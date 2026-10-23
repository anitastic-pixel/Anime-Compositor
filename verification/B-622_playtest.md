# B-622 playtest: Fractal Noise shows its Rotation, Sub Settings and Overflow (D-743)

## What changed for you

Fractal Noise's panel has eight more settings, the ones After Effects shows under Transform and
Sub Settings: **Rotation**, **Perspective Offset**, **Sub Influence**, **Sub Scaling**,
**Sub Rotation**, **Sub Offset**, **Center Subscale**, and **Overflow** (Allow HDR Results, Clip,
Soft Clamp, Wrap Back). Each one can be typed, dragged and keyed like any other setting.

A Fractal Noise you add from now on starts at After Effects' Sub Influence 70 and Sub Scaling 56,
so it has more fine detail than before, as After Effects' does. A project you saved before this
change looks exactly as it did: it keeps Sub Influence 50 and Sub Scaling 50.

## What to look at

In `verification/B-622 pictures/`:

- `1_older_project_50_50.png` to `4_rotation_36.png`: the street with Fractal Noise as an older
  project draws it, as a new one draws it, at Cinecom's 77.5 / 59.7, and turned 36 degrees.
- `5_sheet_odisho_def_sub70_56.png`: After Effects' Fractal Noise as it is added (Justin Odisho's
  tutorial), next to ours before and after. After this change ours has fine detail much closer to
  After Effects' (0.262 against After Effects' 0.316; before, 0.137).
- `6_sheet_ci86_rot36_sub77.5_49.png`: Cinecom's tutorial with Sub Influence 77.5 and Sub
  Scaling 49. Before, ours could not take these numbers; now the fine detail moves the same way.
- `7_sheet_ci82_old_file.png`: an older project, drawn by the build before and after: the same
  picture, to the last pixel.

## Try it

1. Add Fractal Noise to a layer. Open its settings: Sub Influence shows 70 and Sub Scaling 56.
2. Type 90 in Sub Influence: the clouds get grainier. Type 30: they get smoother.
3. Turn Rotation: the clouds turn round the layer's middle.
4. Open a project you saved before today that has Fractal Noise: it looks the same as before, and
   its Sub Influence and Sub Scaling show 50 and 50.

## Known differences from After Effects (not fixed here)

- With Scale Width and Height unequal, our Rotation turns the stretched clouds; After Effects'
  keeps them stretched sideways (Cinecom's sheet 1:22). How the noise is turned is our rule.
- Terrain looks quite different from After Effects', and our basic cloud is about 1.5 times too
  big (both found in Wave 1, audit row 16).
- No Uniform Scaling switch: Fractal Noise has Scale Width and Height for that.
