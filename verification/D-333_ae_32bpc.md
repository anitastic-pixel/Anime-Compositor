# D-333 / B-214: 32 bpc (After Effects) working depth

From P-26. After its step F, tutorial 2 works in After Effects' **32 bpc**. Its Lightning Diff copy keeps the bolt's lower part, blurs it wide with Fast Box Blur, puts it on black and lifts it +20.49 stops with Exposure. In the tutorial that lights a soft warm pool on the ground. Here, in Float, it lit a block about 600 pixels wide. The owner approved building this on 2026-10-06 as the app's best reading of After Effects.

## Where the rule comes from (no After Effects needed)

- After Effects' 32 bpc, unless "Linearize Working Space" is ticked, works in display colours. So a blur averages display values, as in 8 bpc (D-330), just without rounding.
- Exposure is the exception. After Effects' manual says it works "in a linear color space" (CS3 manual, p.402), so it converts to linear light, multiplies, and converts back.
- Which conversion curve it uses is **not written anywhere I could find**. An After Effects engineer says its non-linear blending uses "a 2.2 gamma". With a plain 2.2 curve, faint light is crushed to almost nothing and only a pool lights, as in the tutorial. With the sRGB curve the block stays. So the 2.2 curve was built, and it is recorded as this app's best reading, not as a fact about After Effects.

## What changed

- **Composition Settings, Working depth** has a fourth choice after Float: **32 bpc (After Effects)**.
- In this depth everything works as in Float (light past white is kept, nothing is rounded), with two differences:
  - Gaussian Blur, Fast Box Blur, Directional Blur and Radial Blur average display colours.
  - Exposure brightens through the 2.2 curve. Bright parts get a little brighter than in Float and faint parts much fainter, so a wide faint blur stays dark at its edges.
- A composition inside another follows the outermost one, as with the other depths.
- It is a kind of Float: a file saying 32 bpc (After Effects) without Float on is refused.
- Display stays the default. **No existing picture or project changes.** A project only gets an `ae_32bpc` line when this depth is chosen.

## Checks (cargo test)

Everything is in `verification/D-333_ae_32bpc_table.md`, **40 of 40 pass**. Among them:

| Check | Result |
|---|---|
| FX-AE32-001 to 006: every pixel matches values written by `tools/ae_32bpc_reference.py` before the code existed | pass, tolerance 2e-5 (largest miss 4.5e-6) |
| A file with this depth but no Float, or with `ae_32bpc` set to a word, is refused | pass |
| Set to this depth, saved, opened again and undone | pass |
| A composition inside another follows the outermost one | pass, 4 ways |
| The preview draws the same picture | 0 of 255 apart |
| Cut into tiles, the frame is identical | pass |

The full crate and app suites were run afterwards (see the commit).

## Pictures

In `verification/D-333 pictures/`, the Lightning Diff copy after step F, drawn from this app's own frame 20 of the bolt (`bolt_f20.png`), with the tutorial's settings: Linear Wipe 70%, Fast Box Blur 101.2 across and 121.3 down, Solid Composite on black, Exposure +20.49, Linear Wipe 69% feathered 32.2.

- `built_1_float.png`: Float, as before. A hard-edged block of light (206,573 pixels brighter than a quarter).
- `built_2_ae_32bpc.png`: 32 bpc (After Effects). A soft orange pool where the bolt lands, fading out (23,267 pixels), as in the tutorial.

These match the proposal's pictures (`diff_ours.png` and `diff_ae_gamma22.png`), which were worked separately in Python.

## Tutorial 2 with this depth

Tutorial 2 was replayed from an empty project with this depth at step F, saved again to Downloads and exported again. **Partly right.** The 600-pixel yellow block is gone. Under the strike there is a smaller patch that fades out, but it is red rather than warm, and the ground texture still shows dark holes in it. The tutorial at frame 20 shows almost no light on the ground. The red comes from Exposure through the 2.2 curve, which lifts the orange's red far more than its small green part. Nothing further was changed or guessed.

## Known limits

- An adjustment layer's effects work as in plain Float. The tutorials do not use one.
- The graphics card does none of a layer's effects in this depth, so the viewer may be slower on heavy effects.
- The 2.2 curve is a best reading. If After Effects is ever shown to use another, only this depth changes.

## For the owner to try

1. Open `advanced_electric_tutorial2.json` from Downloads (re-saved in this depth). Look at the ground under the bolt around frame 20: a smaller red patch that fades out, not the big yellow block.
2. Open Composition Settings: Working depth reads **32 bpc (After Effects)**. Switch it to **Float (past white)** and the block comes back. Undo, and the patch returns.
3. In a new composition, set Working depth to 32 bpc (After Effects). Add a small white solid, then Fast Box Blur radius 40, Solid Composite (black) and Exposure +17. The glow should fade softly to its edges. In Float it is a flat white square.

**Awaiting the owner's playtest.**
