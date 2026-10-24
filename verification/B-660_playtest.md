# B-660 playtest: Key Cleaner and Matte Choker render faster, and look the same

## What changed for you

Key Cleaner and Matte Choker are faster wherever the processor draws them: in a Final Render
(export), in 8 bpc (After Effects), and with Draw on: CPU. At 1920 by 1080, Full:

- Key Cleaner: 242 to 233 ms a frame as added, 328 to 256 ms with Additional Edge Radius 40.
- Matte Choker: 93 to 72 ms as added, 303 to 233 ms with heavy softness and 2 iterations.

Refine Hard Matte and Refine Soft Matte use the same code but did not get measurably faster
(their times moved by less than the noise of a busy machine). Matte Choker on the card is
unchanged (about 20 to 57 ms). **No pixel changed**: every effect's frame, on the processor, the
card and in 8 bpc, is the same bit for bit as before (369 of 369 rows).

## What to look at

- `verification/B-660_timing_table.md`: before and after times, the machine and the load.
- `verification/B-660_pixel_table.md`: the bit-for-bit check for these four effects.
- `verification/B-660 pictures/speed.png`: the times as bars.

## Try it

1. Key a green-screen clip with Keylight, add Key Cleaner, set Additional Edge Radius to 40.
2. Render it out (Final Render). It should finish sooner than before, and the file should look
   exactly as one rendered yesterday.
3. Turn on 8 bpc (After Effects) and scrub: it should feel a little lighter, and look the same.
