# B-481: Bezier Warp

Built on 2026-10-12 as D-602, the first effect of mesh and brush warping (P0-18), the group you
put first on 2026-10-12. B-480 (D-601) built the part all six of these effects share; this is the
first effect on it. Bezier Warp is in the Distort group. It gives the layer four corners and, on
each edge, two curve handles; move them and the layer's outline bends, the picture bending with
it. It has After Effects' controls (Top Left Vertex, Top Left Tangent, and so on round the edge,
and Quality). Adobe does not publish how it is worked out, so the bending is this program's own.

The check, `verification/D-602_bezier_warp_table.md` (121 of 121), holds every pixel to numbers a
separate program worked out before the code was written, and holds the graphics card's picture to
the processor's (38 of 38 within 1 level). The pictures are in `verification/D-602 pictures/`.

## What to check

1. **`1_before.png`**: the town street, no effect.
2. **`2_sag.png`**: both top handles pulled down: the top edge sags into the picture in a curve,
   clear (see-through) above it, deepest in the middle.
3. **`3_half_size.png`**: all twelve points moved in to a quarter of the way: the street drawn at
   half size in the middle, clear round it.
4. **`4_s_curve.png`**: the top and bottom handles set opposite ways: the street waved into an S.
5. **`5_corner_pulled.png`**: the top left corner pulled in: the top and left edges bend to meet
   it, the corner it left clear. **`5b_corner_pulled_quality_1.png`** is the same at Quality 1:
   the curve is a coarser chain of straight pieces.
6. **In the app.** Add Bezier Warp to a layer: nothing changes until you move a point. Drag the
   numbers of Top Left Tangent and watch the top edge bend; key a vertex and play.

Not settled: there are no handles to drag on the viewer yet; the points are numbers in Effect
Controls.

## Speed

`verification/B-481_bezier_warp_timing_table.md`: PROVISIONAL, the machine busy with other lanes: on the card 27.2 ms a frame with the top edge sagging and 29.7 folded, against 18.3 for the Noise alone, about 3 to 4 ms added a 1080p layer; the processor 206.1 and 189.8 against 199.2, the machine's own swing larger than the effect.
