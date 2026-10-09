# B-241: Fast Zoom Blur

Built on 2026-10-08 under your effects loop request, decided as D-362. **Fast Zoom Blur** (in
**Blur & Sharpen**) is our version of CycoreFX's CC Radial Fast Blur: quick streaks out from a
centre that fade as they run. Brightest keeps only light streaks, the look of light passing over
a title; Darkest keeps only dark ones.

The settings, with the values they start at:

- **Amount** (50): how far back toward the centre each pixel reads, 0 to 100 per cent.
- **Centre** (50, 50): per cent of the layer's width and height. Key it to sweep the light.
- **Zoom** (Standard): Standard streaks everything; Brightest only lightens; Darkest only darkens.

The check, `verification/D-362_fast_zoom_blur_table.md` (109 of 109), holds every pixel to
numbers worked out by a separate program before the code existed, and the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-362 pictures/`.

## What to check

**`title.png`** is a title on a dark blue card: five white blocks standing for letters, each with
a dark slit. **`before.png`** is the title with no effect.

1. **As added.** `standard_50.png` is Fast Zoom Blur as it starts (Amount 50, Standard, Centre in
   the middle). The letters should streak outward from the middle, fading.
2. **Light rays.** `brightest_40.png` is Amount 40, Brightest, centre up and left of the title.
   White rays should be thrown down and right from every letter over the blue; nothing anywhere
   gets darker.
3. **Dark rays.** `darkest_40.png` is Amount 40, Darkest, the same centre. The blue should eat
   into the letters' edges and slits; nothing anywhere gets lighter.
4. **In the app.** Put Fast Zoom Blur on a text layer, choose Brightest, Amount about 80, and key
   the Centre from the left of the text to the right: a light passing along it.
5. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU Fast Zoom Blur costs about 5.5 ms a 1080p layer, whatever the zoom
(`verification/B-241_fast_zoom_blur_timing_table.md`).

## Not built

- The passing-light tutorial also adds an Overlay copy of the layer and a stroke-only copy of the
  text; both are layer work, not part of this effect.
