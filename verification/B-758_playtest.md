# B-758 playtest: the motion finder on the graphics card (D-878)

## What changed for you

Pixel Motion Blur, Timewarp, Warp Stabilizer and Rolling Shutter Repair all find how each pixel
moves between frames. In the viewer, when the frame is drawn on the graphics card, that finding
now happens on the card too, but only where it is clearly faster: at Draft at any Vector Detail,
and at Full from Vector Detail 40. At Full with Vector Detail below 40 (the default 20 included)
nothing changed: the processor finds the motion, and the card does not set aside its memory for
it. Exports, and any frame the processor draws, find it on the processor as before. No setting,
no saved project and no fixture changed.

The motion the card finds is the processor's to the bit on the reference shot, and a frame is at
most 1 level away (`verification/B-758_card_motion_finder_table.md`).

Speed, on this machine with other work running: the finder alone is about 5 times faster at
Vector Detail 100 and about 1.7 to 2 times at 40, at Full; a whole Full frame with Pixel Motion
Blur is about 1.8 to 2 times faster at Vector Detail 100 and 1.2 to 1.3 times at 40. At Full with
Vector Detail 20 the card was not faster, so it is not used there (`verification/B-758_timing_table.md`).

## What to look at

- `verification/B-758_card_motion_finder_table.md`: the found motion (0 px apart) and every
  fixture and the reference shot, all within the bar.
- `verification/B-758 pictures/pmb_f120_processor_finder.png` and `pmb_f120_card_finder.png`:
  Pixel Motion Blur at Vector Detail 100, frame 120, the middle of the picture. They should look identical.
- `verification/B-758 pictures/pmb_f120_difference_x64.png`: the difference, 64 times brighter.
  Black, or a few dark grey specks, is right.

## Try it

1. Open a project with Pixel Motion Blur or Timewarp on a moving shot, with the viewer on the
   graphics card, and play it.
2. It should look exactly as before. At the default Vector Detail 20 at Full it should play as
   before; with Vector Detail raised to 40 or more it should update faster than before.
3. Export a few frames: the files should be the same as before this change.
