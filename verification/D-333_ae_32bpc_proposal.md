# D-333 (proposed): After Effects' default 32 bpc, as a choice

From P-26's tutorial 2 (Advanced Electric), rebuilt from scratch on 2026-10-06. **A proposal: nothing is built.**

## What is wrong

After the tutorial's step F (the project goes to 32 bpc and the Lightning Diff copy is retuned: Fast Box Blur 101.2 across, 121.3 down, Exposure +20.49), the Diff copy lights a glowing block about 600 pixels wide under the strike. The tutorial's own frames after step F show a soft warm pool there instead.

## What was ruled out first

1. **The ground texture stand-in.** The Diff copy is seen only through a luma matte from Ground_Texture, and our stand-in (the plate, a Fractal Noise grunge in Overlay, a Soft Light plate copy at Exposure +3.88) is brighter than the tutorial's dark grunge. Three replacements were rendered at frames 12 and 20 (`tests/zz_scratch_p26t2.rs`, not committed):

   | Ground_Texture | Ground brightness | The block |
   |---|---|---|
   | today's stand-in | 0.10 | a full block |
   | dark Fractal Noise (Brightness -30, Contrast 200), ground only | 0.10 | a full block with holes |
   | darker Fractal Noise (Brightness -60, Contrast 200), ground only | 0.02 | scattered bright specks over the same 600 pixels |
   | the plate alone, ground only | 0.12 | a full, even block |

   A darker texture only punches holes in the block; it never turns it into a pool. The texture is not the cause, so it was left as it is.

2. **Solid Composite rounding.** If After Effects' Solid Composite were 8 or 16 bpc only, it would round the faint edge to nothing before Exposure. Adobe's first-party effect table lists Solid Composite (`ADBE Solid Composite`) as **32 bpc**, so it does not round. Ruled out.

## Why it happens

This program's Float depth works in linear light, the way After Effects does only when "Linearize Working Space" is switched on. The tutorial uses After Effects' default 32 bpc, which has no linear working space: blurs average display values. Exposure is the exception. After Effects' CS3 manual (p.402): "The Exposure effect works by performing calculations in a linear color space, rather than in the image's current color space." So Exposure converts display values to linear light, multiplies by 2 to the power of the stops, and converts back.

Which curve that conversion uses decides everything here:

- With the **sRGB curve**, which is a straight line near black, faint values stay faint, and 2^20.49 (about 1.4 million) lights them all: still a block.
- With a plain **2.2 power curve**, faint values are crushed (0.001 becomes 0.0000003) before the multiply, and only a soft pool lights.

## Pictures

`verification/D-333 pictures/`, drawn by `tools/d333_ae_32bpc_demo.py` from this program's own bolt at frame 20 (`bolt_f20.png`), with only the Diff copy's own steps applied (Linear Wipe 70%, Fast Box Blur 101.2 / 121.3, Exposure +20.49, Linear Wipe 69% feather 32.2), over black.

`sheet.png`, left to right:

| Picture | How it is worked | Pixels brighter than a quarter |
|---|---|---|
| `diff_ours.png` | this program today (Float, linear light) | 210,232 |
| `diff_ae_srgb.png` | blur on display values, Exposure through the sRGB curve | 200,640 |
| `diff_ae_gamma22.png` | blur on display values, Exposure through a 2.2 power curve | 24,929 |

The left picture matches what the app really draws (the same block, the same 600 pixels), so the demonstration reproduces the app. Only the right picture looks like the tutorial's pool.

## What is proposed

A fourth composition depth, **32 bpc (After Effects)**, alongside Display, Float (D-319) and 8 bpc (After Effects) (D-330):

- Float values, nothing rounded.
- Gaussian Blur, Fast Box Blur, Directional Blur and Radial Blur average display values, as D-330 already does in 8 bpc.
- Exposure converts display values to linear light with a 2.2 power curve, multiplies, and converts back.
- Display stays the default. **No existing picture or project changes**; old files keep today's depth.
- The tutorial 2 replay would switch to it at step F instead of Float.

## What stays open

- **The curve itself.** No source found says which curve After Effects' Exposure uses without a working space. The evidence for 2.2 is indirect: an After Effects engineer (Adobe community forum) says that with "Blend colors using 1.0 gamma" off, After Effects blends "using a 2.2 gamma", and the tutorial's step-F frame matches the 2.2 picture and not the sRGB one. If this is approved, it is built as this program's reading, written down as such, like D-328.
- **8 bpc (After Effects).** D-330's Exposure uses the sRGB curve. If 2.2 is right, 8 bpc should use it too, which would change D-330's fixtures; not proposed here.
- **Blend modes.** After Effects' default 32 bpc also blends layers (Add, Screen, ...) on display values. That is a much larger change and is not proposed.

**Awaiting the owner's decision.**
