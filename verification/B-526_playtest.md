# B-526: Turbulent Noise's Overflow, Transform and Sub Settings

Built on 2026-10-13 as D-647. Turbulent Noise gains these After Effects controls:

- **Overflow**: what happens to greys pushed past white or black by Contrast or Brightness.
  Allow HDR Results (as before), Clip, Soft Clamp (eased, no flat patches) or Wrap Back (folded
  back, so very bright turns dark again, giving banded rings).
- **Rotation** turns the clouds round the layer's middle.
- **Uniform Scaling** on: Scale Width and Scale Height are ignored, so the clouds keep their shape.
- **Perspective Offset** on: when Offset slides the clouds, the fine detail slides less than the
  large shapes, as if further away.
- **Sub Settings**: Sub Influence (how strong the fine detail is), Sub Scaling (how much finer
  each layer of detail is), Sub Rotation and Sub Offset (each finer layer turned or slid further),
  Center Subscale (the fine detail grows from the middle, not the corner).
- **Blending Mode** adds Soft Light, Hard Light, Darken, Lighten and Difference.

The check, `verification/D-647_turbulent_more_table.md` (309 of 309 checks pass), holds
twenty-eight small test projects to numbers worked out before the code.

## What to look at

The pictures in `verification/D-647 pictures/` (the clouds over a small street):

1. `1_as_added.png`: Turbulent Noise as added, as before.
2. `2_rotation_45.png`: the same clouds turned an eighth round the middle.
3. `3_sub_settings.png`: busier, softer fine detail, each layer turned further.
4. `4_center_subscale.png`: the fine detail grows out from the middle.
5. `5_wrap_back.png`: high contrast with Wrap Back: white areas fold back to dark rings.
6. `6_soft_clamp.png`: the same contrast with Soft Clamp: no flat white or black patches.
7. `7_difference.png` and `8_hard_light.png`: the clouds mixed into the street two new ways.

## What to check in the app

9. **Older projects.** Open a project with Turbulent Noise in it: it looks exactly as before.
10. Add Turbulent Noise to a layer. The card shows **Rotation** and **Sub Rotation** (dials),
    **Sub Influence**, **Sub Scaling** and **Sub Offset**, and among the choices **Overflow**,
    **Uniform Scaling**, **Perspective Offset**, **Center Subscale**, and the longer Blending
    Mode list.
11. Turn Rotation: the clouds turn round the layer's middle. Key it and play: they spin.
12. Set Contrast to 300 and try each Overflow: Clip and Allow HDR Results look the same,
    Soft Clamp is gentler, Wrap Back makes rings.
13. Set Scale Width to 200, then Uniform Scaling on: the clouds go back to round.
14. Turn Perspective Offset on and drag Offset: the fine detail moves less than the big shapes.
15. Move Sub Influence, Sub Scaling, Sub Rotation and Sub Offset: the fine detail changes, the
    large shapes stay.
16. Save, close and open again: every setting is kept.

## Not built (known limits)

- **Overlay** blending is not built yet. An earlier check pins Overlay as refused, so adding it
  needs your decision on that check.
- Hue, Saturation, Color, Luminosity, None and the stencil modes are not built.
- After Effects publishes no formula: these rules are ours. After Effects starts Sub Influence
  at 70 and Sub Scaling at 56; ours start at 50 and 50 so older projects look the same.
