# B-286: Detail-preserving Upscale

Built on 2026-10-10, decided as D-407 by your choices of that day: Fit to Comp Width / Height as
"One-time buttons (Recommended)" and Scale up to "1000%, like AE (Recommended)".
**Detail-preserving Upscale** (in **Distort**) is our version of After Effects' effect of that
name: the layer made bigger, up to ten times, about its middle, softened first if you ask and
sharpened afterwards so its edges stay crisp.

The settings, with the values they start at:

- **Scale** (100): per cent, 100 to 1000. 100 leaves the size alone.
- **Reduce Noise** (0): 0 to 100, softens the layer before it is enlarged.
- **Detail** (20): 0 to 100, how hard the edges are sharpened after it is enlarged; high values
  can leave light or dark halos along edges, as After Effects warns.
- **Fit to Comp Width** and **Fit to Comp Height**: buttons. Each works out Scale once so the layer
  is as wide (or as tall) as the composition, on the frame you are on. Afterwards Scale is an
  ordinary number: change it, or press Ctrl+Z to take the fit back. If Scale has keys, the fit
  is a key on that frame.

The check, `verification/D-407_detail_upscale_table.md` (155 of 155), holds every pixel to
numbers worked out by a separate program before the code existed, and the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-407 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: the same size as before, the edges a touch crisper.
2. **Twice the size.** `3_scale_200.png`: the street twice as big about its middle, cut off by
   the frame's edges; the edges still crisp, not blurry.
3. **The most.** `4_scale_1000.png`: ten times the size, the middle of the street filling the
   frame; blocky detail is smoothed rather than shown as squares.
4. **Softened.** `5_soft.png`, scale 150, Reduce Noise 100, Detail 0: enlarged and soft.
5. **Sharpened hard.** `6_sharp.png`, scale 150, Detail 100: enlarged and very crisp, with light
   halos along the edges.
6. **Fit to Comp.** In the app, put a small picture in a bigger composition, add the effect and
   press **Fit to Comp Width**: the picture now spans the composition's width (to within a pixel
   or two). Press Ctrl+Z: it goes back. Press **Fit to Comp Height**: it spans the height.
7. **Too big.** On a picture wider than 3000 pixels set Scale to 1000, which would make it wider
   than the 30000 pixels the program allows: the layer is drawn without the effect and the
   warning list says the layer was too large (EFFECT_LAYER_TOO_LARGE), never a crash.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, the effect as added costs about 1.3 to 1.5 ms a 1080p layer on the graphics
card, and about 3 ms at scale 200; on the processor about 24 and 76 ms
(`verification/B-286_detail_upscale_timing_table.md`, measured on a quiet machine).

## Not built

- Adobe gives no formula, so the enlarging is ours (softening, Lanczos resampling, sharpening);
  it will not match After Effects pixel for pixel.
- Detail's starting value of 20 is ours; Adobe's page gives none.
- After Effects' Alpha choice (a cheaper filter for the transparency, for speed) is left out:
  the transparency is enlarged the same careful way as the colour.
- On a hidden layer the Fit buttons say so in words and do nothing: there is no size to fit.
- No handle on the viewer; set the values in Effect Controls.
- No After Effects tutorial with numbers for this effect was found.
