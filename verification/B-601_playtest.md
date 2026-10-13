# B-601: Turbulent Noise's Overlay and Hue

Built on 2026-10-13 as D-731, after your decision that day to let the two checks that held
Overlay and Hue as refused (FX-TURBNOISE-032 and 060) be rewritten so both could be built.
Turbulent Noise's **Blending Mode** list gains:

- **Overlay**: the layer's light parts get lighter and its dark parts darker where the noise is
  light or dark; its colours stay. It is the Timeline's own Overlay rule, worked out on the
  effect's linear colours like its other modes, so it can look a little different in the
  midtones from Overlay chosen in the Timeline.
- **Hue**: the layer turns towards grey, keeping its light and dark. The noise has no colour,
  so Hue looks the same as Saturation and Color. It uses the public web standard's rule
  (W3C Compositing Level 1), our choice, not a copy of After Effects.

The check, `verification/D-731_turbulent_overlay_hue_table.md` (48 of 48 checks pass), holds the
two rewritten test projects to numbers worked out before the code.

## What to look at

The pictures in `verification/D-731 pictures/` (the noise over a small street):

1. `1_normal.png`: Turbulent Noise in Normal, as before.
2. `2_overlay.png`: the street with its colours kept, lit and shaded by the clouds.
3. `3_overlay_contrast.png`: the same, stronger.
4. `4_hue_60.png`: the street partly drained of colour.
5. `5_hue.png`: the street in its own greys. It matches Saturation's picture from D-730
   (`verification/D-730 pictures/2_saturation.png`), because the noise is grey.

## What to check in the app

6. **Older projects.** Open a project with Turbulent Noise in it: it looks exactly as before.
7. Add Turbulent Noise to a layer. The Blending Mode list now ends with Overlay and Hue.
8. Pick each and compare with the pictures above.
9. Key Opacity from 100 to 0 in Overlay and play: the effect fades away smoothly.
10. Save, close and open again: the mode is kept.

## Not built (known limits)

- **Silhouette Alpha** and **Silhouette Luma** are still not built (D-730): the program has no
  Timeline rule for them to follow yet.
