# Audit P2 and P9: Hair, Star Burst and Curl Noise quicker on the graphics card

Built on 2026-10-14 from the approved audit. **What you should see:** nothing different, only
quicker. Every picture is still within 1 level of 255 of the processor's
(`P2_single_precision_table.md`). Exports are drawn by the processor and do not change.

## What to check

Open the reference shot, set the viewer to **Full**, and for each: put it on layer 2 from the
Effects panel with its starting settings, press play, then delete it.

1. **Hair.** Set **Length** high (long fur). The fur should look as before and play more smoothly.
2. **Star Burst.** Stars should look and move as before.
3. **Curl Noise.** Swirling flow lines, as before. Then set **Speed** and **Turbulence Speed** to 0
   and play: the swirl holds still and playing should be quicker than with them moving.
4. **Curl Noise, Source This Layer.** The lines follow the layer's own picture, as before.

For each, nothing should flicker from frame to frame.

## What to report

"works", or the number of the step that did something else, what it did, and the frame number.
