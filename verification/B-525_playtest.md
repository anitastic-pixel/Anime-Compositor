# B-525: Curl Noise's Rotation, Scale and Perspective Offset

Built on 2026-10-13 as D-646. Curl Noise gains After Effects' three Transform controls:

- **Rotation** turns the swirling noise round the layer's middle.
- **Scale** makes it bigger (over 100) or smaller (under 100) round the same middle.
- **Perspective Offset** on: when the noise drifts (Speed above 0), the fine detail drifts
  slower than the large shapes, as if further away.

The check, `verification/D-646_curl_transform_table.md` (122 of 122 checks pass), holds
seventeen small test projects to numbers worked out before the code.

## What to look at

The pictures in `verification/D-646 pictures/`:

1. `1_noise.png`: the noise itself (View Input Noise), as before.
2. `2_noise_rotation_90.png`: the same noise turned a quarter clockwise round the middle.
   Tip your head to the right and the blobs of picture 1 should line up.
3. `3_noise_scale_300.png`: the same noise three times as big, grown out from the middle.
4. `4_flow_upright.png` and `5_flow_rotation_45.png`: the flow lines, then the same lines turned
   an eighth of a turn.

## What to check in the app

6. **Older projects.** Open a project with Curl Noise in it: it looks exactly as before.
7. Add Curl Noise to a layer. The card shows **Rotation** (a dial) and **Scale** under Offset,
   and **Perspective Offset** among the choices.
8. Turn Rotation: the swirls turn round the layer's middle. Key it from 0 to 360 over a few
   seconds and play: the pattern spins.
9. Set Scale to 300: the swirls grow; 50: they shrink.
10. Set Speed to 40 and Perspective Offset on, play: the fine wisps drift slower than the large
    shapes.
11. Save, close and open again: the three settings are kept.

## Not built (known limits)

- With Source This Layer or Other Layer, Rotation and Scale only change where the streaks start;
  the picture itself is not turned. Perspective Offset does nothing there.
- After Effects publishes no formula: the turn is round the layer's middle and the slower fine
  detail halves per level of detail, both our choice.
