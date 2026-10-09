# B-271: Smear

Built on 2026-10-09 under your effects loop request, decided as D-392. **Smear** (in
**Distort**) is our version of CycoreFX's CC Smear: it drags a round patch of the picture from
one point toward another, like a finger smudging wet paint. Key it to pull a face, stretch a
punch, or melt something. The formulas are ours; CycoreFX publishes none.

The settings, with the values they start at:

- **From** (40, 50) and **To** (60, 50): per cent of the layer, where the drag starts and ends.
- **Reach** (100): per cent of the way from From to To that the picture moves; 200 drags twice
  as far, minus drags it back the other way.
- **Radius** (70): pixels, how big the dragged patch is. The drag is full along the line between
  the points and eases to nothing at the patch's edge.

Nothing grows, and outside the patch the picture is untouched.

The check, `verification/D-392_smear_table.md` (141 of 141), holds every pixel to numbers worked
out by a separate program before the code existed, and the graphics card's picture to within 1
level of the processor's. The pictures are in `verification/D-392 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: the buildings in the middle dragged to the right in a curling
   streak.
2. **A long drag.** `3_long_drag.png`, from (30, 50) toward (60, 40), Reach 200, Radius 60: the
   buildings pulled out in a long streak up and to the right, like wet paint.
3. **Pushed back.** `4_pushed_back.png`, from (50, 60) toward (50, 30) at Reach -100: the
   buildings are dragged down into the road instead of up.
4. **In the app.** Add Smear, key Reach from 0 at one frame to 300 ten frames later and play: the
   patch stretches out smoothly. Set Radius to 0: the picture goes back to normal.
5. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Smear costs too little to read against the timing noise, and about 15.4 ms a
1080p layer on the processor (`verification/B-270_distort_timing_table.md`; provisional, the card
figures were within their own noise).

## Not built

- How the drag grows along the line from From to To, and how it eases off at the patch's edge,
  are our own reading of CycoreFX's one-sentence descriptions; no tutorial with numbers was
  matched.
- There are no From and To handles on the viewer yet; set them in Effect Controls.
