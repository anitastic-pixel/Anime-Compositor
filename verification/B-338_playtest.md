# B-338: CC Simple Wire Removal

Built on 2026-10-10 as D-458, under your /loop request: CC Simple Wire Removal, in
**Lines & Mattes** (beside the keyers), with CycoreFX's controls. It paints out a thin wire or
rig running straight between two points.

- **Point A**, **Point B** (25, 50 and 75, 50): the wire's two ends, in per cent across and down.
- **Removal Style** (Displace): **Fade** makes the wire see-through; **Frame Offset** fills it
  from the same layer at another frame (a clean plate, or a frame where the wire has moved);
  **Displace** covers it with the picture just either side of it; **Displace Horizontal** does the
  same but only looks left and right along each row.
- **Thickness** (5): how wide a band is covered, in pixels.
- **Slope** (50): how hard the band's edge is; 100 is a hard edge, 0 a soft edge as wide again.
- **Mirror Blend** (25, Displace only): how much of the picture mirrored across the wire is mixed
  in, so the cover is not a plain smear.
- **Frame Offset** (-1, Frame Offset only): which frame fills the wire, counted from this one.

Cycore publishes only the control names and what they are for, so the rule, the ranges and the
starting values (except Mirror Blend's 25, which Cycore gives) are our own, written down in
document 21. Our pictures will not be exactly Cycore's.

The check, `verification/D-458_simple_wire_removal_table.md` (194 of 194), holds
every pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-458 pictures/`.

## What to check

1. **The shot.** `1_before.png`: a row of town buildings with a dark wire slanting down across
   the sky onto a red roof.
2. **Displace.** `2_displace.png`: the wire is gone from the sky, covered by the sky either side
   of it. Where it crossed a building's edge a short smudge stays, because the cover is made
   from the picture on both sides of the wire, and those differ there. A faint dot may stay at
   each end: the drawn wire's round end sticks out just past the points, and the remover's ends
   are square. Setting the points a little past the wire's ends clears it.
3. **Fade.** `3_fade.png`: the wire's band is made see-through, so the white page behind shows
   as a white streak with soft edges.
4. **Displace Horizontal.** `4_displace_horizontal.png`: on the slanting wire, each row's cover
   is taken from the left and right only, so the smudges where the wire crossed a building lie
   along the rows.
5. **Mirror Blend 100.** `5_mirror_100.png`: the cover is only the picture reflected in the edges
   of the band: a faint line shows in the sky, where its shading is mirrored, and the roof
   corners the wire crossed are mirrored too.
6. **In the app.** Add CC Simple Wire Removal (Lines & Mattes) to footage with a wire. Move Point
   A and Point B onto the wire's ends, raise Thickness until the wire is covered, and try each
   Removal Style. For Frame Offset, set the offset to a frame where the wire is elsewhere or
   absent. Set Thickness to 0: the picture is back as it was.
7. **Adjustment layer.** On an adjustment layer, Frame Offset leaves the picture alone and the
   app warns that an adjustment layer cannot read another frame.
8. **Out of range.** Type 150 in Thickness: it is refused with a sentence saying the range.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: how soft the edge is at Slope 50, and how much the mirror
shows at Mirror Blend 25.

## Speed

Measured on a quiet machine
(`verification/B-338_simple_wire_removal_timing_table.md`). Three CC Simple Wire Removals on a 1920 by 1080 shot, card /
processor, milliseconds a frame played again: the shot after a moving Noise 12.4 / 40.4 ms a frame, Displace, thickness 20 16.4 / 56.1, Frame Offset -1, thickness 20 52.0 / 77.8; Displace about 1.3 ms a 1080p layer on the card (5.2 on the processor), within Target P2 (4 ms or less); Frame Offset about 13.2 on the card (12.5 on the processor), over it, because the layer at the other frame, with the effects before this one, is drawn again by the processor and sent to the card every frame (Color Stabilizer's way).
