# B-151: ten of the fourth batch on the card, by hand

Built on 2026-09-29 under your "sounds great! let's do 1 through 7 to your discretion." (D-217).
Median, Smart Blur, Roughen Edges, Radial Shadow, Bevel Alpha, Snowfall, Cell Pattern, Polar
Coordinates, Optics Compensation and Corner Pin are now drawn by the graphics card in the viewer
when one is a drawn layer's last effect. Exports and renders to file are untouched: the processor
still draws those, byte for byte as before.

`verification/B-151_gpu_fx_table.md` is the comparison: every fixture frame of the ten, and the
reference shot with each on three layers (one after a Drop Shadow), at Full and Draft, drawn by
the processor and by the card. **2100 of 2100 checks pass**, none more than 1 level of 255 apart,
with the same warnings on both. The three pictures of the worst frame, the reference shot with
Median, are in `verification/B-151 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`,
black where the two agree and white round every pixel 1 level apart.

**Kaleidoscope stays on the processor for now (D-240, a proposal for you).** On the card it
matched everywhere you can see, but at a few exact mirror lines the card's sine and cosine land a
hair from the processor's, and a pixel that should be fully see-through comes out a millionth
visible. Its hidden colour is then anything up to 246 levels off, one to four pixels a frame on
four fixtures. You cannot see it, but the check counts it, and the rule is never to loosen the
check. D-240 in document 14 gives the choices.

## Before you start

- Use the release build. It opens on the reference shot.
- Leave **Draw on** at **Auto**.

## What to check

For each, select a layer, add the effect with the settings given, and switch **Draw on** between
**CPU** and **GPU** a few times. The picture should not change in any way you can see.

1. **Median.** Radius 4, Operate on Alpha On. The drawing goes soft and blotchy the same on both.
2. **Smart Blur.** Radius 4, Threshold 40. Flat colours smooth out and lines stay sharp, the same
   on both.
3. **Roughen Edges.** Edge Type Roughen Color, Border 6, Size 8. The torn brown edge is the same
   on both. Play it on GPU: the edge crawls as on CPU.
4. **Radial Shadow.** Light Source 30, 10, Distance 15, Softness 8, Render Glass Edge. The shadow
   falls away from the light the same on both.
5. **Bevel Alpha.** Edge Thickness 4, Light Angle -45. The raised edge is lit from the same side
   on both.
6. **Snowfall.** Density 60, Spacing 40, Size 5. The same flakes in the same places on both. Play
   it on GPU: they fall as on CPU.
7. **Cell Pattern.** Crystals, Size 30, Blend Screen. The same cells on both. Try other patterns.
8. **Polar Coordinates.** Rect to Polar, Interpolation 70. The drawing wraps into the same ring
   on both.
9. **Optics Compensation.** Field of View 60. The same bulge on both. Turn Reverse Lens Distortion on: the same
   pinch on both.
10. **Corner Pin.** Drag the four corners anywhere. The drawing bends to the same shape on both.
11. **Draft.** Switch to **Draft** and repeat steps 1, 6 and 10. CPU and GPU still match.
12. **Not the last effect.** Put a Gaussian Blur after any of the ten. The picture is the same on
    both; the ten then stay on the processor, as any effect before another does.
13. **Export is untouched.** An export is the same file whatever **Draw on** says.

## How long a frame takes

Measured on this machine with other builds running, so these are first numbers, to be measured again when the machine is quiet. Milliseconds for one whole frame of the reference shot at Full with the effect on three layers, drawn by the graphics card:

| Effect | Before | After |
|---|---:|---:|
| Roughen Edges, every frame | 102.6 | 22.7 |
| Snowfall, every frame | 67.0 | 22.9 |
| Median, first time a frame is seen | 29.5 | 15.8 |
| Smart Blur, first time a frame is seen | 28.9 | 15.5 |
| Radial Shadow, first time a frame is seen | 28.4 | 17.4 |

The other five already cost little on these drawings. At Draft the card is no faster, and Snowfall and Roughen Edges are a little slower there (21.2 and 17.4 ms against 12.5 and 13.1); that is looked at later in the plan. The full table is `verification/B-151_gpu_fx_timing_table.md`.

## What to report

"works", or which step number did something else and what it did, with the settings and the frame
number.
