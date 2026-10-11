# B-440: Solid Composite, its fixture folder

Built on 2026-10-11 as D-560, under your /loop request. Solid Composite (Channel) was added in
B-195 for the Advanced Electric tutorial: it lays the layer, at its own opacity, on a solid colour
at another, blended normal, add, screen or multiply. Nothing about the effect changed. What is new
is its check: nineteen cases worked out by a separate program before the test was written, the
way every other effect is held.

The check, `verification/D-560_solid_composite_table.md` (132 of 132), holds every
pixel to those numbers, and holds the graphics card's picture to the processor's (37 of
37 within 1 level). The pictures are in `verification/D-560 pictures/`, made from the town
street.

## What to check

1. **Before.** `1_before.png`: the street.
2. **Half over blue.** `2_half_over_blue.png`: Source Opacity 50 over a #3366cc solid: the street
   half seen through a blue, as if behind tinted glass.
3. **Multiply orange.** `3_multiply_orange.png`: Multiply with #ff8040: the street warmer and
   darker; the white road markings turn orange, nothing gets lighter.
4. **Screen grey.** `4_screen_grey.png`: Screen with #808080: the street paler and washed out;
   nothing darker than mid grey is left.
5. **In the app.** Add Solid Composite to a drawing with see-through parts: they fill with white.
   Change the colour and the fill follows; set Opacity to 0 and the drawing returns exactly.

Not settled: After Effects' own list of Solid Composite's blending modes could not be read, so it
may offer more than these four.

## Speed

`verification/B-440_solid_composite_timing_table.md`: PROVISIONAL (machine not quiet: other lanes' cargo and rustc builds were running, and the card's Noise alone came out at 47.9 ms against 13.5 on B-433's run), the reference shot at Full played again with a moving Noise on three layers: card 49.2 ms a frame with #3366cc at source opacity 50 and 42.8 with multiply #ff8040 at opacity 60, against 47.9 for the Noise alone, so no cost shows above the noise; processor 236.9 and 210.4 against 134.2; a quiet re-measure is owed.
