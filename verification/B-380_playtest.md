# B-380: Color Range

Built on 2026-10-10 as D-500, under your /loop request: Color Range, in **Lines & Mattes**
(beside the other keyers), with After Effects' settings. It keys out every colour inside a box
you set, so it can take out a screen of several shades, or one lit unevenly.

- **Color Space** (Lab): which three parts the colour is split into. Lab: lightness, then a
  (green to red, 128 is grey) and b (blue to yellow, 128 is grey). YUV: brightness, then U
  (more blue) and V (more red), 128 grey. RGB: red, green, blue.
- **Min** and **Max** for each part, 0 to 255: the box. A colour is keyed out when all three
  of its parts lie between their Min and Max.
- **Fuzziness** (0): colours just outside the box fade instead of staying whole. 0 keys only
  what is inside; larger numbers soften the edge further.

When added every end is 0, so nothing is keyed until you set the box. Adobe publishes no
formula and no starting values, so the rule and the 0 to 255 scales are our own, written down
in document 21. After Effects sets the box by clicking colours with eyedroppers in a small
matte picture; those are not here: the six ends are typed or dragged.

The check, `verification/D-500_color_range_table.md` (134 of 134), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-500 pictures/`.

## What to check

1. **The shot.** `1_before.png`: a green screen, an oval face whose edge has picked up green
   from the screen, and a blue shirt.
2. **A hard box.** `2_lab_hard.png`, Lab, L 120 to 210, a 40 to 95, b 150 to 210,
   Fuzziness 0: the screen gone (the page shows
   through), the face, its green edge and the shirt all whole.
3. **A soft box.** `3_lab_soft.png`, the same box with Fuzziness 40: the screen still gone, the
   face and shirt whole, and the face's green edge now partly see-through, fading out toward
   the screen. Colour is never changed, only how solid it is.
4. **Another space.** `4_yuv_shirt.png`, YUV, Y 40 to 110, U 160 to 220, V 90 to 140,
   Fuzziness 20: the blue shirt gone, everything else whole.
5. **As added.** With the starting settings the picture is unchanged.
6. **In the app.** Add Color Range (Lines & Mattes) to green-screen footage. Pick a space,
   then widen each Min and Max pair about the screen's values until the screen goes, and raise
   Fuzziness to soften the edge. Keyframe Fuzziness or an end and play it.
7. **Out of range.** Type 300 in a Max: it is refused with a sentence saying the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the empty box when added, and whether the 0 to 255 scale
for Lab's a and b (128 grey) reads naturally.

## Speed

Measured on a quiet machine
(`verification/B-380_color_range_timing_table.md`). Three Color Ranges on a 1920 by 1080 shot, card / processor,
milliseconds a frame played again: the shot after a moving Noise 12.0 / 42.0 ms a frame, RGB, fuzziness 60 17.6 / 49.2, Lab, fuzziness 30 18.6 / 59.5; about 1.9 to 2.2 ms a 1080p layer on the card, over the row's Target P1 (1 ms, a one-pixel colour change), as the other keyers are; on the processor about 2.4 to 5.8 ms a layer.
