# B-339: Color Difference Key

Built on 2026-10-10 as D-459, under your /loop request: Color Difference Key, in
**Lines & Mattes** (beside the other keyers), with After Effects' controls. It keys out a
screen colour (green or blue) by building two partial mattes, A and B, and joining them into
one matte that sets how solid each pixel is.

- **View** (Final Output): **Final Output** is the keyed layer; **Source** is the layer
  untouched; the other six show partial A, partial B or the joined matte as grey (white is
  solid, black see-through), each before (Uncorrected) or after (Corrected) its levels.
- **Key Color** (blue, #0000ff): the screen's colour.
- **Partial A** and **Partial B**, five each: **In Black**, **In White**, **Gamma**, **Out Black**,
  **Out White**, exactly as in Levels.
- **Matte In Black**, **Matte In White**, **Matte Gamma**: levels on the joined matte. In Black
  is the grey that becomes fully see-through, In White the grey that becomes fully solid.

Adobe publishes no formula, so the rule is our own, after Stefan Ihringer's measured
reconstruction of After Effects' key, written down in document 21. Our mattes will be close to
After Effects' but not proven identical. After Effects' four-up view and its Faster / More
Accurate setting are not here.

The check, `verification/D-459_color_difference_key_table.md` (171 of 171), holds
every pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-459 pictures/`.

## What to check

1. **The shot.** `1_before.png`: a green screen, an oval face whose edge has picked up green
   from the screen, and a blue shirt.
2. **The matte, untuned.** `2_matte_uncorrected.png`, Key Color set to the screen's green
   (#28b43c), view Uncorrected Matte: the screen is the darkest grey, the face lighter, the shirt
   lightest. The screen is not yet black: that is what the matte's levels are for.
3. **The matte, tuned.** `3_matte.png`, Matte In Black 110 and Matte In White 180, view
   Corrected Matte: the screen black, the face and shirt white, the green edge of the face grey.
4. **The key.** `4_final.png`, the same settings, view Final Output: the screen gone (the page
   shows through), the face and shirt exactly as they were, the green edge half see-through.
   The edge keeps its green tint: the key only takes covering, never colour. Advanced Spill
   Suppressor, after this, takes the green out.
5. **As added, on single colours.** With the starting settings (blue key): pure blue is fully
   see-through, the violet (128, 64, 255) half, pure green fully solid.
6. **In the app.** Add Color Difference Key (Lines & Mattes) to green-screen footage. Pick the
   screen's colour in Key Color. Switch View to Corrected Matte, raise Matte In Black until the
   screen is black and lower Matte In White until the subject is white, then switch back to
   Final Output. Try the Partial views and their levels to clean harder edges. View Source shows
   the layer untouched.
7. **Out of range.** Type 300 in Matte In White: it is refused with a sentence saying the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the starting values (Adobe gives none), and how much
tuning a plain green screen needs before it is clean.

## Speed

Measured on a quiet machine
(`verification/B-339_color_difference_key_timing_table.md`). Three Color Difference Keys on a 1920 by 1080 shot, card /
processor, milliseconds a frame played again: the shot after a moving Noise 12.1 / 40.9 ms a frame, as added 18.5 / 52.1, every levels set 18.4 / 54.7; about 2.1 ms a 1080p layer on the card, within the row's Target P2 (4 ms or less) but over Target P1 (1 ms, a one-pixel colour change), as Advanced Spill Suppressor's is; on the processor about 3.7 to 4.6 ms a layer.
