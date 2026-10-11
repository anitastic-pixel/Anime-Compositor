# B-383: Key Cleaner

Built on 2026-10-10 as D-503, under your /loop request: Key Cleaner, in **Lines & Mattes**
(beside the other keyers), with After Effects' settings but Reduce Chatter. Put it after a key:
near the keyed edge it fits the see-through part to the picture's colours, bringing back hair and
soft detail that the key, or the footage's compression, lost.

- **Additional Edge Radius** (10): how many pixels either side of the keyed edge are worked on.
- **Alpha Contrast** (0): makes the part-see-through edge pixels more solid or more clear; at 100
  every pixel is either kept or gone.
- **Strength** (100): how much of the cleaning is applied; 50 is halfway back to the key alone.

After Effects' Reduce Chatter (a smoothing of the edge over time) is not built yet. Adobe publishes
no formula, so the rule is our own, written down in document 21: it is the same fit as Refine Soft
Matte. The radius of 10 when added is Adobe's; the other settings when added are ours. The
cleaner is worked on the processor for now; the graphics card draws whatever follows it.

The check, `verification/D-503_key_cleaner_table.md` (132 of 132), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-503 pictures/`.

## What to check

1. **The key.** `1_keyed.png`: the street keyed by Extract (its dark parts gone), no cleaner.
2. **Cleaned.** `2_cleaned.png`: Key Cleaner as added. It should look nearly the same as 1 at a
   glance; the soft edges of the key are fitted to the street's colours (about 14,000 pixels
   change slightly).
3. **A tighter radius.** `3_radius_3.png`: radius 3; only a few pixels right at the edges change.
4. **Full contrast.** `4_contrast_100.png`: Alpha Contrast 100: every edge pixel is now either
   fully kept or fully gone, so the edges look harder.
5. **Half strength.** `5_strength_50.png`: halfway between 1 and 2.
6. **In the app.** Key a shot (Extract, Color Key or Color Range), add Key Cleaner after it
   (Lines & Mattes) and look at the edges zoomed in; try the radius, Alpha Contrast and
   Strength. Keyframe the radius and play it.
7. **Out of range.** Type 101 in Strength: it is refused with a sentence saying the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: Alpha Contrast 0 and Strength 100 when added, and how the
edge is fitted, which you can compare with After Effects' own Key Cleaner on the same shot.

## Speed

Measured on a quiet machine
(`verification/B-383_key_cleaner_timing_table.md`). Three Key Cleaners on a 1920 by 1080 shot, card / processor,
milliseconds a frame played again: the shot after a moving Noise 13.0 / 43.9 ms a frame, as added (radius 10) 684.3 / 670.9, radius 4, contrast 40 695.5 / 666.2, radius 30, strength 60 711.7 / 690.2; about 223.8, 227.5 and 232.9 ms a 1080p layer in the card's frame (the cleaner drawn on the processor, the card drawing what follows it), over the row's Target P3 (8 ms); on the processor about 209.0, 207.4 and 215.4 ms a layer.
