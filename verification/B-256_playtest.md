# B-256: Bend It

Built on 2026-10-09 under your effects loop request, decided as D-377. **Bend It** (in
**Distort**) is our version of CycoreFX's CC Bend It: the strip of the layer along a bar from
Start to End is curled into an arc, as a bar of rubber bends, turning Bend degrees over its
length. At 360 the bar's two ends meet in a circle. The formulas are ours; CycoreFX publishes
none.

The settings, with the values they start at:

- **Bend** (45): degrees, -360 to 360, how far the bar turns from Start to End. Positive curls
  to the right of the bar as it runs from Start to End, negative to the left.
- **Start** (50, 100) and **End** (50, 0): per cent of the layer's width and height. As it
  starts the bar stands up the middle, from the bottom edge to the top.
- **Render Prestart** (None): what is drawn before the Start: None leaves it out, Static keeps it
  as it is, Bend carries the bend on round the other way, Mirror bends a mirror copy of the bar
  back from the Start.
- **Distort** (Legal): what is drawn past the End: Legal leaves it out, Extended lays it on
  straight in the bar's last direction.

Where the bend lays the layer over itself, the part farther along the bar is on top. The layer
does not grow: whatever the bend carries past its edges is cut off.

The check, `verification/D-377_bend_it_table.md` (161 of 161), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-377 pictures/`. They are
saved over white, so parts left see-through show as white.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: the street leans and curls to the right as it rises, its top
   an eighth of a turn over; the top left, carried past the edge, is cut off.
2. **Bend 0.** `3_bend_0.png`: exactly the same as before.
3. **Half a turn.** `4_curl_180_static.png`, Bend 180, Start 50, 70, End 50, 10, Render Prestart
   Static: the street above the Start rolls over into a half circle; the road below it stays
   where it was.
4. **A banner.** `5_banner_across_mirror.png`, Bend -90, Start 50, 50, End 100, 50, Render
   Prestart Mirror: the street curls up at both sides into a bowl, like a banner hung by its
   middle.
5. **In the app.** Add Bend It to a layer and key Bend from 0 to 360 over two seconds: the
   layer starts straight and rolls up until its two ends meet in a ring. Drag Start and End
   across the picture and the bend follows the bar. Try each Render Prestart and Distort choice.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Bend It costs about 2.7 to 3.1 ms a 1080p layer, within our 4 ms target, and about 10 ms on the processor (`verification/B-256_distort_timing_table.md`; provisional, measured while other tests ran).

## Not built

- The bend is our own reading of CycoreFX's one-sentence description; no tutorial with numbers
  was matched against it.
- The layer never grows to hold a bend carried past its edges, as in After Effects.
- CycoreFX's other controls, if any beyond these four, are not offered; its blend with the
  original is the Mix every effect has.
