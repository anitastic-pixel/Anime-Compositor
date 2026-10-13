# B-554: Polygon Shards

Built on 2026-10-13 as D-675 under your /loop request: **Polygon Shards**, in **Generate** beside
Mercury, after what CycoreFX's CC Pixel Polly is for. It cuts the layer along a grid into triangles
or squares and throws them away from a force centre, in 3D, falling and spinning. It is seen
through the composition camera when the project has one, otherwise through the effect's own eye.

Nothing is remembered from frame to frame. Each shard's flight is worked out from the start time,
so any frame is the same picture whether it is drawn alone, scrubbed to or rendered out of order.

The settings (the value when added in brackets):

- **Force** (300): how hard the shards are thrown. Shards near the force centre fly fastest.
- **Gravity** (1): how fast they fall.
- **Spinning** (60): how fast they turn, in degrees a second.
- **Force Center** (the middle): where the shards are thrown from.
- **Direction Randomness** (25), **Speed Randomness** (25): how much each shard's way and speed
  vary.
- **Grid Spacing** (20): the size of the pieces, in pixels.
- **Object** (Textured Polygon): Polygon and Square pieces show their own average colour;
  Textured Polygon and Textured Square pieces show the picture they were cut from.
- **Enable Depth Sort** (on): nearer pieces drawn over farther ones. Off, they are drawn in grid
  order.
- **Start Time** (0): how many seconds after the layer's start the layer breaks.

CycoreFX publishes no formula and no values when added, so the rule and those values are ours. They
are written down in document 21 and in `tools/polygon_shards_reference.py`.

The check is `verification/D-675_polygon_shards_table.md`, and 208 of 208 checks pass.
It compares every pixel with numbers that a separate program worked out before the code existed.
It also checks that the graphics card's picture stays within 1 level of the processor's (54 of
54 comparisons). The pictures are in `verification/D-675 pictures/` and use the street picture
from D-506.

## What to check

1. **As added.** `1_as_added_frame_24.png` (frame 24): blue triangles from the sky flying out in a
   ring round the middle, which is already empty, the nearer ones larger, tilted at many angles.
   Most of the street's lower part has already left the picture.
2. **Just cracked.** `2_just_cracked_frame_3.png` (frame 3): the street still readable, buildings,
   windows and road in place, cut by thin white gaps along a slanting grid, the gaps widest in the
   middle.
3. **Flat squares.** `3_flat_squares.png`: fewer, larger squares in a ring round an empty middle,
   each one a single flat colour.
4. **From the top left.** `4_from_the_top_left.png`: the triangles gathered in the lower right,
   thrown away from the top left corner, which is empty.
5. **Small tumbling pieces.** `5_small_tumbling_pieces.png`: hundreds of small triangles over the
   whole picture, mostly sky blue, a few with the buildings' colours near the bottom.
6. **Depth sort off.** `6_depth_sort_off.png`: the as-added shot drawn in grid order. It looks
   almost the same as picture 1; where pieces overlap, a far piece can sit over a near one.
7. **In the app.** Put Polygon Shards (Generate) on the street picture and play. The street should
   break apart from frame 0.
8. **Start Time.** Set Start Time to 1 and play. The street should stay whole for a second, then
   break.
9. **Scrub.** Jump to frame 40, then play from 0 to 40. Frame 40 should be the same picture both
   times.
10. **Camera.** Add a camera and move it. The flying pieces should be seen from the camera.
11. **Out of range.** Type 2000 in Force. It should be refused with a sentence that gives the range.
12. **Saved and opened again.** Save, close and open the project. The settings and the picture
    should be the same.

Choices of ours that you may want to judge:
- the values when added and the units;
- how far towards you the pieces are thrown;
- a plain piece's colour (the average of its pixels);
- that every setting is read at the frame drawn, so keying Force changes the whole flight;
- that there is no seed (CC Pixel Polly has none).

## Speed

`verification/B-554_polygon_shards_timing_table.md`. Three layers after a moving Noise, card / processor, ms a frame (PROVISIONAL, the machine was busy): Noise alone 39.4 / 58.3, as added 44.3 / 133.1; Polygon Shards, flat triangles on a grid of 8 pixels 44.7 / 336.5; Polygon Shards, force keyed 0 to 600 50.0 / 111.2; on the card an unknown amount a frame (the machine was busy) as added; a busy machine cannot say whether the card is quicker.
