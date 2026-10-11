# B-433: Shift Channels, its fixture folder

Built on 2026-10-11 as D-553, under your /loop request. Shift Channels (Channel) was added in
B-190 for the Colorful Glitch tutorial: it takes each of a picture's alpha, red, green and blue
from another channel or from a measure of its colour (luminance, hue, lightness, saturation), or
sets it full, half or off. Nothing about the effect changed. What is new is its check: twenty
cases worked out by a separate program before the test was written, the way every other effect
is held.

The check, `verification/D-553_shift_channels_table.md` (130 of 130), holds every
pixel to those numbers, and holds the graphics card's picture to the processor's (38 of
38 within 1 level). The pictures are in `verification/D-553 pictures/`, made from the town
street.

## What to check

1. **Before.** `1_before.png`: the street.
2. **Alpha from luminance.** `2_alpha_from_luminance.png`: the Colorful Glitch use. Dark parts
   drop out, bright parts stay; only white stays fully solid.
3. **Red and blue swapped.** `3_red_blue_swapped.png`: the street with its reds and blues traded,
   greens as they were.
4. **Hue as grey.** `4_hue_as_grey.png`: the street in greys; every grey part of the street goes
   black, since a grey has no hue.
5. **Alpha full.** `5_alpha_full.png`: nothing see-through left; solid parts as they were.
6. **In the app.** Add Shift Channels to a drawing and set Take Alpha From to Luminance: the dark
   lines fade out. Set it back to Alpha and the drawing returns exactly.

Words: After Effects' Full On, Half On and Full Off are called Full, Half and Off here.

## Speed

`verification/B-433_shift_channels_timing_table.md`: PROVISIONAL (machine not quiet: other lanes' cargo and rustc builds were running), the reference shot at Full played again with a moving Noise on three layers: card 21.3 ms a frame with alpha from luminance and 30.5 with all four channels shifted, against 13.5 for the Noise alone; processor 74.5 and 62.4 against 46.9; a quiet re-measure is owed.
