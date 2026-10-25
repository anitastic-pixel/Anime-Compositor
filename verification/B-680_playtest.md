# B-680 playtest: Caustics with a big Bottom Blur renders about twice as fast, and looks the same

## What changed for you

Caustics with a large Bottom Blur is much faster. At 1920 by 1080, Full, Bottom Blur 40: the final
render went from 467 to 203 ms a frame, and the viewer from 464 to 189 ms (the median of 5 rounds;
the after build was faster in every round). **No pixel changed**: at frame 30, the processor's
picture, the viewer's and 8 bpc's are the same bit for bit as before, as added and at Bottom Blur 40.
A small blur (Caustics as added) is drawn exactly the old way.

## What to look at

- `verification/B-680 pictures/caustics_before_after_difference.png`: before | after | where they
  differ. The third panel is plain dark grey because nothing differs. A changed pixel would be ringed
  in red.
- `verification/B-680 pictures/speed.png`: the times as bars (red before, green after).
- `verification/B-680_caustics_pixel_table.md`: the pixel check. Every "Pixels changed" is 0.
- `verification/B-680_timing_table.md`: the times, the machine and the load.

## Try it

1. Add Caustics to a layer and set Bottom Blur to 40.
2. Play or scrub. It should respond noticeably faster than before and look exactly the same.
3. Set Bottom Blur back to its default. That case is drawn as before.
