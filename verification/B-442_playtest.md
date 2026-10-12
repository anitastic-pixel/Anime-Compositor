# B-442: CC Vignette

Built on 2026-10-11 as D-562, under your /loop request. CC Vignette (Stylize) darkens a picture
toward its edges the way a real lens does: the wider the lens's angle of view, the darker the
corners. A negative Amount lightens the edges instead, and Pin Highlights keeps the bright parts
of the picture bright. It is a new effect beside the Vignette you already have, which is
unchanged: CC Vignette's controls do not fit Vignette's colour, size, roundness and softness.

The check, `verification/D-562_cc_vignette_table.md` (169 of 169), holds every pixel
to numbers a separate program worked out before the code was written, and holds the graphics
card's picture to the processor's (46 of 46 within 1 level). The pictures are in
`verification/D-562 pictures/`, made from the town street.

## What to check

1. **Before.** `1_before.png`: the street.
2. **As added.** `2_as_added.png`: the corners and edges darker, fading smoothly into a middle
   that looks as before.
3. **Lighter edges.** `3_amount_minus_100.png`: Amount -100: the edges lighter instead, nothing
   darker anywhere.
4. **A wide lens.** `4_angle_of_view_150.png`: Angle of View 150: much darker corners than 2.
5. **Pin Highlights.** `5_pin_highlights_100.png`: as 2, but the bright parts (sky, lights) keep
   most of their brightness while dark parts still darken.
6. **Off centre.** `6_centre_top_left.png`: the centre moved to the top left corner: the top left
   as the street, darkening toward the bottom right.
7. **In the app.** Add CC Vignette (under Camera & Lens, beside Vignette). Drag Angle of View
   from 0 up: the corners darken. Drag Amount below 0: they lighten. Drag Center: the bright
   middle follows.

Not settled: CC Vignette's real starting values and exact falloff could not be read (Adobe's page
refused the request and CycoreFX publishes no formula), so the falloff is a real lens's natural
one and the starting values are this program's choice.

## Speed

`verification/B-442_cc_vignette_timing_table.md`: provisional (other lanes' GPU tests and builds were running throughout): on the card the reference shot with a moving Noise took 22.9 ms a frame, 27.9 with CC Vignette as added and 21.1 lightening; on the processor 62.3, 208.7 and 226.1.
