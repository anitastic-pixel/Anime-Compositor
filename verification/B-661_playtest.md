# B-661 playtest: Caustics renders faster, and looks the same

## What changed for you

Caustics (drawn on the processor only) is faster everywhere: at 1920 by 1080, Full, 109 to 97 ms
a frame as added and 372 to 323 ms with Bottom Blur 40 (the viewer: 395 to 332 ms). **No pixel
changed**: both Caustics rows of the check are the same bit for bit as before, and so is every
other effect's.

## What to look at

- `verification/B-661_timing_table.md`: before and after times, the machine and the load.
- `verification/B-661_pixel_table.md`: the bit-for-bit check.
- `verification/B-661 pictures/speed.png`: the times as bars.

## Try it

1. Add Caustics to a layer, set Bottom Blur to 40, and play.
2. It should play a little more smoothly and look exactly as before. A big blur is still slow;
   making it much faster would move the last bits of the picture, so that is a proposal
   (D-783), not built.
