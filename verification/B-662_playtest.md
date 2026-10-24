# B-662 playtest: Pixel Motion Blur and Timewarp render faster, and look the same

## What changed for you

Pixel Motion Blur and Timewarp (drawn on the processor only) are faster everywhere. At 1920 by
1080, Full, a Final Render:

- Pixel Motion Blur: 188 to 167 ms a frame as added, 273 to 259 ms with 32 shutter samples and
  Vector Detail 40.
- Timewarp: 189 to 160 ms as added, 442 to 393 ms with Vector Detail 40, 8 smoothing passes and
  10 shutter samples.

The viewer gains about the same. **No pixel changed**: all 369 rows of the check (every effect, as
added and heavy, on the processor, the card and in 8 bpc) are the same bit for bit as before.

## What to look at

- `verification/B-662_timing_table.md`: before and after times, the machine and the load.
- `verification/B-662_pixel_table.md`: the bit-for-bit check of every effect.
- `verification/B-662 pictures/speed.png`: the times as bars.

## Try it

1. Put Timewarp on a moving clip, Method Pixel Motion, Speed 50, and play.
2. It should play a little more smoothly than before, and look exactly the same.
