# B-600: Turbulent Noise's six more blending modes

Built on 2026-10-13 as D-730. Turbulent Noise's **Blending Mode** list gains:

- **Saturation** and **Color**: the layer turns grey (the noise is grey), keeping its light and dark.
  The two look the same on Turbulent Noise because the noise has no colour of its own.
- **Luminosity**: the layer keeps its colours, lit and darkened by the noise.
- **None**: the noise alone replaces the layer; Opacity makes it see-through.
- **Stencil Alpha**: the layer is kept where the noise is (everywhere, since the noise is solid);
  Opacity fades the whole layer.
- **Stencil Luma**: the layer shows only where the noise is light, and is clear where it is dark.

Saturation, Color and Luminosity use the public web standard's rules (W3C Compositing Level 1),
our choice, not a copy of After Effects. None and the stencils follow the Timeline's own modes,
as Adobe's help says they should.

The check, `verification/D-730_turbulent_blends_table.md` (309 of 309 checks pass), holds fourteen
small test projects to numbers worked out before the code.

## What to look at

The pictures in `verification/D-730 pictures/` (the noise over a small street):

1. `1_normal.png`: Turbulent Noise in Normal, as before.
2. `2_saturation.png`: the street drained to grey, its light and dark kept.
3. `3_luminosity.png`: the street's colours kept, made lighter and darker by the noise.
4. `4_none.png`: the noise alone, no street. It matches picture 1, because the street is
   solid everywhere and Normal at full opacity covers it too; the difference shows in 5.
5. `5_none_60.png`: the noise alone, see-through.
6. `6_stencil_luma.png`: the street only where the noise is light.
7. `7_stencil_luma_50.png`: the same, fainter.

## What to check in the app

8. **Older projects.** Open a project with Turbulent Noise in it: it looks exactly as before.
9. Add Turbulent Noise to a layer. The Blending Mode list now shows Saturation, Color,
   Luminosity, None, Stencil Alpha and Stencil Luma.
10. Pick each one and compare with the pictures above.
11. Pick None and drag Opacity to 0: the layer goes fully clear (it does not come back).
12. Pick Stencil Luma, key Opacity from 100 to 0 and play: the street fades out.
13. Save, close and open again: the mode is kept.

## Not built (known limits)

- **Silhouette Alpha** and **Silhouette Luma** are not built: the program has no Timeline rule
  for them to follow yet. A file asking for them opens with a warning and draws as if the
  effect were off.
- **Hue** and **Overlay** are still not built. Earlier checks pin them as refused, so adding
  them needs your decision on those checks.
