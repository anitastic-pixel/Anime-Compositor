# B-685 playtest: the motion finder works in single precision

## What changed for you

Pixel Motion Blur, Timewarp, Warp Stabilizer and Rolling Shutter Repair all find how each pixel
moves between frames. That finder now does its arithmetic in 32-bit numbers instead of 64-bit.
The finder alone is 1.16 to 1.24 times faster and a whole frame about 1.1 times faster. The found
motion moves by less than a ten-thousandth of a pixel, and a frame is at most 1 level away from
before (`verification/B-685_flow_single_table.md`, 11 of 11 pass). No fixture changed and no
saved project changes.

## What to look at

- `verification/B-685_flow_single_table.md`: the cost on every fixture and the reference shot,
  and the timings.
- `verification/B-685 pictures/Pixel_Motion_Blur_as_added_f120_before_centre.jpg` and
  `..._after_centre.jpg`: the same blur before and after; they should look identical.
- The four `*_f120_difference.png`: black everywhere, 0 pixels differ at frame 120.

## Try it

1. Open a project with Pixel Motion Blur or Timewarp on a moving shot and play it.
2. It should look exactly as before and play a little faster.

## What else moved

The D-650, D-652, D-653 and D-654 verification tables and three D-653 pictures and one D-654
picture were rewritten by their tests; every check in them still passes.
