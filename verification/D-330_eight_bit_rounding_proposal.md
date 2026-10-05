# D-330 (proposed): After Effects' 8 bpc rounding, as a choice

From P-26's tutorial 2 replay, after D-327 (Fast Box Blur). **A proposal: nothing is built.**

## What is wrong

Tutorial 2 makes the ground reflections of its bolt from copies of the bolt. Each copy keeps only the bolt's tips (a Linear Wipe at 70%), blurs them very wide with Fast Box Blur, then brightens them by 9 to 17 stops with Exposure. In After Effects that gives a few soft streaks and a small glow where the bolt meets the ground. Here it gives a glowing block about 600 pixels wide.

Fast Box Blur is now exact (D-327), so the blur is not the cause.

## Why

The tutorial is drawn in an **8 bpc** project until its step F. In 8 bpc, After Effects keeps each layer's pixels as whole numbers 0 to 255 between effects. The blur's faint outer edge is under half a step, so it becomes 0. Exposure then has nothing out there to brighten.

This program keeps every value exactly between effects, however faint. Times 2 to the 17th power (about 160,000), that faint edge turns white all the way out to the blur's reach.

After Effects' CS3 manual (p.402) also says Exposure "works by performing calculations in a linear color space, rather than in the image's current color space". The pictures below take that into account.

## Pictures

`verification/D-330 pictures/` holds three of the bolt's tips, 9 by 6 pixels, in the tutorial's blue, drawn by `tools/d330_eight_bit_demo.py`. Only the blur and Exposure are applied.

`sheet.png`: top row is the Lightning Diff copy (Fast Box Blur 75.2 across, 87.3 down, Exposure +17.33); bottom row is the Lightning Spec 2 copy (6.3, 34.3, +9.33). Left to right:

| Picture | How it is worked | Pixels lit (Diff) |
|---|---|---|
| `diff_ours.png` | this program today | 321,402 |
| `diff_ae_8bpc.png` | rounded to 8 bits after each effect, as After Effects' 8 bpc | 16,572 |
| `diff_ae_32bpc.png` | the same without the rounding | 309,452 |

Rounding to 8 bits alone shrinks the glow to about a twentieth. That matches the tutorial's small glow. The Spec 2 copy gives the three soft vertical streaks seen in the tutorial's frame either way.

## What is proposed

A new composition depth, **8 bpc (After Effects)**, alongside today's Display and Float (D-319):

- In 8 bpc, each layer's pixels are rounded to 8 bits after every effect. The proposal is to round display (sRGB) values, as After Effects stores them.
- Display stays the default. **No existing picture or project changes**: old files keep today's depth.
- The tutorial 2 replay would use 8 bpc until step F, as the tutorial does.

## What stays open

- **Display values or linear?** After Effects without a linear working space blurs and blends display values. This program works in linear light everywhere. The rounding is what matters here. Blurring display values as well would be a much larger change, and is not proposed.
- **The step-F frame.** After step F the tutorial is in 32 bpc, where nothing is rounded. Here the Diff glow (Exposure +20.49) still lights a large block, and the "ae_32bpc" picture says After Effects' arithmetic would too. Yet the tutorial's last frames show a small glow. The video does not show which setting explains that, so it is left open rather than guessed.

**Awaiting the owner's decision.**
