# B-684 playtest: the card draws the faint edge of a motion blur in the same colour as the processor

## What changed for you

Pixel Motion Blur and Timewarp leave a few almost invisible pixels at the edge of a moving shape,
covering about one millionth of the pixel. The card used to send such a picture to itself in half
precision, which keeps that covering but loses the pixel's colour. With colour effects after the
blur (Hue/Saturation, Exposure, Levels) the card then wrote those pixels up to 10 levels away from
the processor. They are transparent, so on screen you could not see it, but a saved PNG or an
export with an alpha channel kept the wrong colour.

Now a picture with such a pixel goes to the card in full precision. **No processor pixel changed**,
no fixture changed, and the card's picture is now the processor's to within 1 level everywhere
(`b222_gpu_colour`: 8230 of 8230 checks pass, 3 failed before).

## What to look at

- `verification/B-684_faint_upload_table.md`: the three cases that failed, processor against card,
  before (10, 10 and 6 levels) and now.
- `verification/B-684 pictures/fx_pmb_013_f1_difference.png` and
  `fx_tw_020_f5_difference.png`: black everywhere means the card and the processor agree.
- `verification/B-222_gpu_colour_table.md`: the full check, 8230 of 8230.

## Try it

1. Open `Fixtures/pixel_motion_blur/fx_pmb_013.json`. On its layer put Hue/Saturation before
   Exposure and Levels after it.
2. With the card on, go to frame 1 and export a PNG with alpha; do the same with the card off.
3. The two files should look the same and, in an image viewer that shows transparent colour, have
   the same colour at the blurred edge.

## What else moved

W7's hash check (`perf_audit_unchanged`, all 369 rows of the reference shot, run before and after
this change): 349 rows are the same bit for bit. In 20 rows the card's bytes moved and nothing
else did (the processor's floats and the 8 bpc bytes are the same): Reduce Interlace Flicker, CC
Wide Time, Pixel Motion Blur, Timewarp, Warp Stabilizer, Rolling Shutter Repair, CC Rainfall, Card
Dance, Key Cleaner, Refine Hard Matte and Refine Soft Matte. Each of these processor-only effects
leaves such faint pixels, so its picture now goes to the card in 32-bit floats, and the card's
bytes come closer to the processor's: 32-bit floats hold every value half precision holds and
more. A picture with no such pixel goes up exactly as before; the plain shot's row is unchanged
and no slower (5 alternating rounds, card Full 13.0 ms before, 11.7 ms after).
