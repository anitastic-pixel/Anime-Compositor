# B-626 playtest: Glow's further After Effects controls (D-747)

Decision D-747 (owner, 2026-10-23, from https://claude.ai/artifact/A63KdY44N4HfUHzUnLQhTp: "Proceed ... if it is to be a 1 to 1 match, then go for it").

## What changed

Glow (`core.glow`) gains After Effects' remaining panel controls:

- **Composite Original**: Behind (the default, as before), On Top, None (the glow light alone).
- **Glow Dimensions**: Horizontal and Vertical (default), Horizontal, Vertical.
- **Glow Colors**: Original Colors (default, as before) or A & B Colors, with Color Looping, Color Loops, Color Phase, A & B Midpoint, Color A and Color B.
- **Glow Operation**: Normal, Multiply, Overlay, Soft Light, Hard Light, Darken, Lighten, Difference, Hue, Saturation, Color, Luminosity, joining Add and Screen.

The defaults are After Effects'. An old project with none of these settings draws exactly as before.

## What to look at

Open `verification/B-626 pictures/6_app_before_b625_b626.png`. It has one row per case and three columns: the app before Wave 5, after B-625, and after B-626. Each case is an orange disc on grey, with Glow in After Effects units, at 8 bpc, rendered in the real app (CPU, Full).

| Row | Before / B-625 | B-626 (expected) |
| --- | --- | --- |
| old file, no new settings | glow | the same glow (largest difference from B-625: 0) |
| Composite None | ignored: same as the old file | the disc and grey gone; only the yellow light on checkerboard |
| Dimensions Horizontal | ignored | glow spreads left and right only |
| Dimensions Vertical | ignored | glow spreads up and down only |
| A&B red to blue, None | ignored | red at the core, blue toward the edge |
| A&B loops 3, phase 90, mid 25 | ignored | rings of yellow and cyan |
| Normal On Top | refused (warning, no glow) | the light painted over the disc |
| Normal Behind | refused | the disc unchanged (the picture covers the light) |
| Lighten On Top | refused | the disc lightens toward yellow |
| Overlay Behind | refused | yellow core |
| Difference On Top | refused | the disc turns green-dark |

Pixel values from B-626 (middle, 48 px right, 48 px up):

| Case | Middle | 48 px right | 48 px up |
| --- | --- | --- | --- |
| Composite None | 255,255,0 alpha 255 | 255,255,0 alpha 32 | 255,255,0 alpha 39 |
| Horizontal | n/a | 101,101,64 | 64,64,64 (no glow) |
| Vertical | n/a | 64,64,64 (no glow) | 108,108,64 |
| A&B red to blue, None | 255,0,0 | 32,0,223 alpha 32 | 39,0,216 alpha 39 |
| A&B loops 3 | 255,255,220 | n/a | n/a |
| Normal On Top | 255,255,0 | 88,88,56 | n/a |
| Lighten On Top | 255,255,40 | 88,88,64 | n/a |
| Overlay Behind | 255,255,0 | n/a | n/a |
| Difference On Top | 0,95,40 | n/a | n/a |

Note: the older apps quietly ignore Composite, Dimensions and the A & B settings. This is why a project that uses them must be opened in B-626 or later.

## Checks

- `tests/b626_glow_controls.rs`: 61 of 61. The results table is `verification/B-626_glow_controls_table.md`; the pictures are 1 to 5 in `verification/B-626 pictures/`.
- App test `glow_controls_typed_in_the_panel_reach_the_glow` checks four things:
  - every control typed in the panel reaches the Glow;
  - a bad choice is refused;
  - a setting survives save and reopen;
  - a send without the new keys gives the defaults.
- Glow's earlier tables pass unchanged: B-33b 127/127, B-625 44/44, D-337 40/40.

## Verdict

| Item | Verdict |
| --- | --- |
| Old projects draw as before | Pass. The B-625 and B-626 app frames are identical; saving writes no new keys. |
| Each control works when typed in the panel | Pass (app test, real-app sheet) |
| Defaults are After Effects' | Pass (Behind, Horizontal and Vertical, Original Colors, Triangle A>B>A, 1 loop, 0 degrees, 50%, white and black) |
| Matches After Effects' pixels | **Not measured.** Wave 1's frames from After Effects cover only the defaults. The new controls follow the published formulas (W3C blend modes; AE's documented A & B gradient), which are our basis and are fixed in FX-GLDISP-009..016. Confirming them 1 to 1 needs new After Effects frames. |

## Not done / known gaps

- After Effects frames for the new controls (see above).
- After Effects' remaining Glow Operations without a published formula (Color Dodge, Color Burn, Linear Dodge/Burn, Vivid/Linear/Pin Light, Hard Mix, Exclusion, Subtract, Divide, Classic modes, Stencil/Silhouette and so on) are still refused with a warning, never silently replaced.
- The card (GPU) handles only Add and Screen, Original Colors, both dimensions, and only in a linear-depth project. Every other case runs on the processor, so it is correct but slower.
- B-625's leftovers remain: the 8 bpc "val" sample reads 1.6× After Effects, and the somsqr sample shows a dark edge.
