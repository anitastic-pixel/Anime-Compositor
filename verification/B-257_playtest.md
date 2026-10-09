# B-257: Bender

Built on 2026-10-09 under your effects loop request, decided as D-378. **Bender** (in
**Distort**) is our version of CycoreFX's CC Bender: the layer pushed sideways between a Base
that stays put and a Top, so a bottle or a building seems to sway, swell or kink. The formulas
are ours; CycoreFX publishes none.

The settings, with the values they start at:

- **Amount** (20): -1000 to 1000, how far the layer is pushed: pixels, or with Adjust To
  Distance on, per cent of the distance from Base to Top. Negative pushes the other way.
- **Style** (Bend): Bend curves smoothly from the Base and runs on straight past the Top;
  Marilyn swells out in the middle, Base and Top staying put; Sharp makes a kink half way;
  Boxer moves smoothly from the Base to the Top and shifts everything past the Top whole.
- **Adjust To Distance** (Off): On reads Amount as per cent of the Base to Top distance, so the
  push grows and shrinks as you move the points.
- **Top** (50, 0) and **Base** (50, 100): per cent of the layer's width and height. As it starts
  the axis stands up the middle, the Base at the bottom edge.

The push is only across the axis, never along it. The layer does not grow: whatever is pushed
past its edges is cut off.

The check, `verification/D-378_bender_table.md` (160 of 160), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-378 pictures/`. They are
saved over white, so parts left see-through show as white.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: the street is swept to the right more and more as it rises,
   the bottom row still and the top 20 pixels over; a white sliver opens at the top left.
2. **Amount 0.** `3_amount_0.png`: exactly the same as before.
3. **Marilyn.** `4_marilyn_40.png`, Amount 40, Style Marilyn: the middle of the street bulges 40
   pixels to the right, the top and the bottom staying put, the houses wavy.
4. **Boxer across.** `5_boxer_across.png`, Amount -30, Style Boxer, Top 100, 50, Base 0, 50:
   the street steps smoothly up 30 pixels from the left edge to the right.
5. **In the app.** Add Bender to a layer and key Amount from -40 to 40 and back over two
   seconds: the layer sways from side to side like a tree. Try each Style, and turn Adjust To
   Distance on and drag the Top towards the Base: the bend tightens with the distance.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Bender costs about 0.7 to 0.9 ms a 1080p layer, and about 6.6 ms on the processor (`verification/B-256_distort_timing_table.md`; provisional, measured while other tests ran).

## Not built

- The four shapes are our own reading of CycoreFX's one-sentence descriptions; no tutorial
  with numbers was matched against them.
- The layer never grows to hold what is pushed past its edges, as in After Effects.
- CycoreFX's blend with the original is the Mix every effect has.
