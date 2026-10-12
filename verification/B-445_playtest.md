# B-445: Energy Stroke's extras

Built on 2026-10-11 as D-565, under your /loop request. Energy Stroke (the glowing beam, pick #11)
gains four settings. **Start Roundness** and **End Roundness** shape the white core's two ends:
100 is round, as before; 0 is cut flat across; in between is a shorter, squashed round end.
**Noise Along Path** chooses where the wobble comes from: On (as before) moves the wobble with the
beam; Off fixes the wobble on the layer, so a beam drawing on passes through wobbles that stay put.
**Alpha Mode** lets the layer's masks cut the beam: Masks cuts the core and the glow, Core cuts the
core only (the glow spills past the masks), Glow cuts the glow only. **Invert Masks** keeps what is
outside the masks instead. Projects saved before today look exactly as they did.

The check, `verification/D-565_energy_stroke_extras_table.md` (191 of 191), holds every pixel to
numbers a separate program worked out before the code was written, and holds the graphics card's
picture to the processor's (57 of 57 within 1 level). The pictures are in
`verification/D-565 pictures/`, an orange beam across the town street.

## What to check

1. **Round ends.** `1_round_ends_as_added.png`: the beam as before, both ends round.
2. **Flat ends.** `2_flat_ends.png`: both ends of the white core cut square; the orange glow
   still rounds off past them.
3. **Flat start, pointed end.** `3_flat_start_pointed_end.png`: flat on the left; the beam
   narrows to the right and ends in a short squashed point.
4. **Wobble along the path.** `4_noise_along_path.png`: a wobbling beam, as before.
5. **Wobble on the layer.** `5_noise_on_the_layer.png`: the same wobble settings with Noise
   Along Path off: a different wobble. In the app, key End Offset from 0 to 100 and play: with Off
   the bends stay where they are as the beam grows; with On they slide along with it.
6. **Masks cut the core.** `6_masks_cut_the_core.png`: a rectangle mask in the middle of the
   picture: the white core shows only inside it, the orange glow runs the whole length.
7. **Masks cut both, inverted.** `7_masks_cut_both_inverted.png`: the same mask with Masks and
   Invert Masks: the beam shows only outside the rectangle, with a gap in the middle.
8. **In the app.** Add Energy Stroke, set Start Roundness to 0: the left end of the core goes
   flat. Draw a mask on the layer and set Alpha Mode to Masks: the beam is cut to the mask. With
   Alpha Mode on and no mask, the beam stays whole and the warnings say there is no mask to cut by.
   The choice says it runs on the processor, so playback may slow a little.

Not settled: Saber publishes no formula, so how these look is this program's own. Open masks (a
mask that is a line, not a closed shape) are still not built.

## Speed

`verification/B-445_energy_stroke_extras_timing_table.md`: quiet (no other cargo, rustc or test process seen): on the card the reference shot with a moving Noise took 12.3 ms a frame, 14.7 with Energy Stroke as added, 15.7 with core 30 and both ends flat, 55.1 with distortion 20 and the noise on the layer, 71.1 with Alpha Mode masks and one mask (drawn on the processor); on the processor 41.8, 44.0, 45.0, 125.6 and 64.5.
