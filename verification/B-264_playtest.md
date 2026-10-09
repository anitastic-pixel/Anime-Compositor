# B-264: Flow Motion

Built on 2026-10-09 under your effects loop request, decided as D-385. **Flow Motion** (in
**Distort**) is our version of CycoreFX's CC Flo Motion: two points, called knots, that each
pull the picture in towards themselves or blow it out of themselves, like pinching or
stretching a rubber sheet. The formulas are ours; CycoreFX publishes none.

The settings, with the values they start at:

- **Knot 1** (25, 50) and **Knot 2** (75, 50): per cent of the layer's width and height. As it
  starts they sit a quarter and three quarters of the way across, half way down.
- **Amount 1** (10) and **Amount 2** (-10): -1000 to 1000. A positive amount draws the picture
  in towards its knot, a negative one blows it out like a lens. 0 leaves that knot doing
  nothing.
- **Falloff** (5): 0 to 10, how far each knot reaches. Each step up reaches about 1.6 times
  farther.
- **Tile Edges** (On): what shows where the picture is pulled in from past the layer's edges.
  On repeats the picture there, mirrored, so nothing goes see-through; Off leaves it clear.
- **Finer Controls** (Off): On divides the amounts by 20, for small, careful moves.
- **Antialiasing** (Low): Low, Medium or High smooths the stretched parts more, at more cost.

The layer does not grow.

The check, `verification/D-385_flow_motion_table.md` (171 of 171), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-385 pictures/`. They are
saved over white, so parts left see-through show as white.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: round the left knot the street is drawn in, the buildings
   leaning towards a point at the left; round the right knot it swells out, the buildings
   bigger. The edges stay filled.
2. **Amounts 0.** `3_amounts_0.png`: exactly the same as before.
3. **A swell.** `4_swell_middle.png`, Knot 1 in the middle, Amount 1 -40, Amount 2 0, Falloff 3:
   the middle of the street swells out like a magnifying glass, the rest close to untouched.
4. **A pull with the edges off.** `5_pinch_edges_off.png`, Knot 1 in the middle, Amount 1 40,
   Amount 2 0, Falloff 8, Tile Edges Off, Antialiasing High: the whole street drawn in to a
   small picture in the middle, everything round it clear.
5. **In the app.** Add Flow Motion to a layer and drag the knots about in Effect controls, then
   key Amount 1 from 0 to 40 over two seconds: the picture slowly sucks in towards the knot.
   Turn Tile Edges off and on: the clear edges fill with a mirrored copy.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Flow Motion costs about 1.8 ms a 1080p layer at Low and about 19 ms at High
(sixteen points a pixel), and about 16 and 78 ms on the processor
(`verification/B-264_distort_timing_table.md`; provisional, the machine was busy).

## Not built

- The pull, how far Falloff reaches and the scale of the amounts are our own reading of
  CycoreFX's one-sentence descriptions; no tutorial with numbers was matched against them.
- Tile Edges mirrors everything the effect is given, including any room an earlier effect
  added round the layer.
- The layer never grows to hold what is blown past its edges.
