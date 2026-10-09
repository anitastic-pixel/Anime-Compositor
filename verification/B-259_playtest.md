# B-259: Channel Blur's Units (Blurriness or Sigma)

Built on 2026-10-09 (D-380). You chose option (c) ("c").

Channel Blur now has a **Units** choice in Effect controls, the same one Gaussian Blur got in
D-321:

- **Blurriness (After Effects)**: each of Red, Green, Blue and Alpha Blurriness is read the way
  After Effects reads its Blurriness, about a third as strong as before. Channel Blur and
  Gaussian Blur now give the same softness at the same number.
- **Sigma (older projects)**: the old rule, exactly as before.

A Channel Blur you add from now on starts in Blurriness. A project saved before this opens in
Sigma and looks exactly as it did. A Units word the program does not know (from a hand-edited
file) is kept, a warning is shown, and the effect is left out, as with any other bad setting.

**The pictures:**

- `verification/D-380 pictures/channel_blur_scale.png`, from the proposal: Blurriness 10, 30 and
  red only 20, today (Sigma) against Blurriness, beside Gaussian Blur at the same number.
- `verification/D-380 pictures/built_switch.png`, drawn by the real build, the street four ways:
  - **top left**, Red 20 in Sigma: the red is spread so wide that it mostly leaves the white
    road marks and windows, so they turn cyan, and the road goes a dull red;
  - **top right**, Red 20 in Blurriness: a red glow stays close round the road marks and windows;
  - **bottom left**, all four at 10 in Sigma: the houses are a blur and the windows are gone;
  - **bottom right**, all four at 10 in Blurriness: softened, but every window is still there.
    This is the same picture as Gaussian Blur at Blurriness 10 (`built_5_gaussian_blurriness_10.png`),
    0 levels apart.

## Checks

- `verification/D-380_channel_blur_units_table.md`: **100 of 100**. All of FX-CHBLUR-001 to 014
  (the old file, untouched) and FX-CHBLUR-015 to 024 (Blurriness, written before the code) match
  within 0.0000002. Saving, loading, keys, undo, tiles, and the viewer on the card matches the
  export (0 levels apart) at Full and Draft.
- `verification/B-223_gpu_blurs_table.md`: Channel Blur **264 of 264** within 1 level (the old 164
  plus the 100 new frames).
- Older checks still pass: app (94), b07_effects, b155, b195, b205, b223, b242.
- Timing: `verification/B-259_channel_blur_units_timing_table.md`, **provisional** because another
  lane's build was running the whole time. An old project's Channel Blur (Sigma) is as fast as
  before: 22.5 ms a frame before and 21.8 after, played again. No change.

## What to check

1. Add a Channel Blur to a layer. Units should say **Blurriness (After Effects)**.
2. Set Red Blurriness to 20: a red fringe close to the edges, as in the top right picture.
3. Switch Units to **Sigma (older projects)**: the red spreads much wider, as in the top left.
4. Open a project saved before today that has a Channel Blur: Units says Sigma and the picture
   is unchanged.
5. Set all four Blurriness numbers to 10 in Blurriness, then add a Gaussian Blur at 10 on a copy
   of the layer: the two should look the same.
6. Save, close and reopen: Units is kept.

## Not like After Effects

After Effects' Channel Blur has no Units choice; Sigma exists here only so old projects keep
their look. That After Effects' Channel Blur uses the same 0.3 scale as its Gaussian Blur is
likely, not proven: nothing published states it (see the proposal sheet).
