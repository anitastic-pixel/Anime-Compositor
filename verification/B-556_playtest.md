# B-556: Star Burst

Built on 2026-10-13 as D-677 under your /loop request: **Star Burst**, in **Generate** beside
Scatterize, after what CycoreFX's CC Star Burst is for. It breaks the layer into a field of stars,
each the colour of the picture where it was. The field flies through space towards you, and each
star comes round again from far away. It is seen through the composition camera when the project
has one, otherwise through the effect's own eye.

Nothing is remembered from frame to frame. Every frame adds up the speed from the layer's start,
so any frame is the same picture whether it is drawn alone, scrubbed to or rendered out of order.

The settings (the value when added in brackets):

- **Scatter** (100): how far the stars are spread from their grid places, across and in depth. At
  0 they stay where they were put.
- **Speed** (1): how fast the field flies towards you, in layer heights a second. Below 0 it flies
  away.
- **Phase** (0): moves the field along without time passing. With Scatter 0, Phase 0 lines the
  stars up with the picture.
- **Grid Spacing** (4): how close together the stars are. It also sizes them.
- **Size** (100): how big each star is.
- **Blend w. Original** (0): how much of the picture is mixed back. At 100 the picture is alone.

CycoreFX publishes no formula and no values when added, so the rule and those values are ours. They
are written down in document 21 and in `tools/star_burst_reference.py`.

The check is `verification/D-677_star_burst_table.md`, and 153 of 153 checks pass. It
compares every pixel with numbers that a separate program worked out before the code existed. It
also checks that the graphics card's picture stays within 1 level of the processor's (43 of
43 comparisons). The pictures are in `verification/D-677 pictures/` and use the street picture
from D-506.

## What to check

1. **As added.** `1_as_added_frame_24.png` (frame 24): a field of round dots in the street's colours
   spreading out from the middle: sky blue at the top, dark road grey at the bottom, the buildings'
   reds, greens and yellows between. Small dots crowd the middle; a few bigger ones lie further out.
2. **Close look.** `2_close_look_grid_20_frame_24.png`: far fewer, bigger dots, in the same colour
   order. The ones further from the middle are larger, and one large dark dot near the top centre
   is the nearest.
3. **Picture as dots.** `3_picture_as_dots.png`: no scatter and no speed. The street drawn as a neat
   grid of coloured dots: blue sky, coloured buildings, dark road.
4. **Dots half a height nearer.** `4_dots_half_a_height_nearer.png`: the same grid moved towards
   you by Phase. The dots are bigger and further apart, so less of the street fits: the outer
   buildings are cut off at the sides.
5. **Big stars.** `5_big_stars_size_400_frame_48.png` (frame 48): large round dots overlapping and
   filling the whole picture, small ones in the middle, very large ones at the edges.
6. **Blend with original.** `6_blend_with_original_50_frame_24.png`: the street clearly visible,
   faded, with the dots field laid over it at half strength.
7. **Composition camera.** `7_composition_camera_from_the_left_frame_24.png`: the field of picture
   1 seen from another place: the same kind of cloud, but its dots sit in different places and the
   big dots near the edges are different ones.
8. **In the app.** Put Star Burst (Generate) on the street picture and play. Stars should rush
   towards you and keep coming.
9. **Scrub.** Jump to frame 40, then play from 0 to 40. Frame 40 should be the same picture both
   times.
10. **Keyed speed.** Key Speed from 0 to 5. The field should start still and speed up smoothly.
11. **Out of range.** Type 200 in Speed. It should be refused with a sentence that gives the range.
12. **Saved and opened again.** Save, close and open the project. The settings and the picture
    should be the same.

Choices of ours that you may want to judge:
- the values when added and the units;
- how deep the field is;
- the effect's own eye when there is no camera;
- that there is no seed (CC Star Burst has none).

## Speed

`verification/B-556_star_burst_timing_table.md`. Three layers after a moving Noise, card / processor, ms a frame (PROVISIONAL, the machine was busy): Noise alone 14.1 / 61.5, as added 25.8 / 97.0; Star Burst, Grid Spacing 0.5 (about half a million stars) 320.3 / 516.0; Star Burst, Speed keyed 0 to 5, Blend w. Original 30 91.6 / 102.5; on the card an unknown amount a frame (the machine was busy) as added; a busy machine cannot say whether the card is quicker.
