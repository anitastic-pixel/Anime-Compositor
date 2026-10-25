# B-681 playtest: Scatterize plays about twice as fast in the viewer

## What changed for you

In the viewer at Full, at 1920 by 1080, Scatterize went from 251 to 110 ms a frame (the median of
5 rounds; the after build was faster in every round). Before, the card was slower than the
processor; now it is about as fast.

**The export is unchanged, bit for bit.** In the viewer, 2 of the frame's 2,073,600 pixels differ
by 1 level out of 255. You cannot see that; the picture below rings the two pixels in red so you
can find them.

## What to look at

- `verification/B-681 pictures/scatterize_card_before_after_difference.png`: before | after |
  where they differ. The left and right pictures should look the same. The two red rings in the
  third panel mark the only 2 pixels that changed, each by 1 level.
- `verification/B-681 pictures/speed.png`: the times as bars (red before, green after).
- `verification/B-681_scatterize_pixel_table.md`: the pixel check against the bar (at most 1 level
  on 99.9 per cent of pixels, at most 2 anywhere). The card row reads 2 pixels changed, largest 1.
  The processor and 8 bpc rows read 0.
- `verification/B-681_timing_table.md`: the times, the machine and the load.

## Try it

1. Add Scatterize to a drawing layer, set the viewer to Full, and play.
2. It should play noticeably more smoothly than before and look the same.
3. Export a frame. It is exactly the old export.
