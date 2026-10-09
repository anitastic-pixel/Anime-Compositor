# B-242: Channel Blur and Unsharp Mask, checked

Done on 2026-10-08 under your effects loop request, decided as D-363. Nothing new to learn in
the app: **Channel Blur** and **Unsharp Mask** (Sharpen, in **Blur & Sharpen**) already had every
setting After Effects has. What was missing was proof. Two separate programs now work out, pixel
by pixel, what each should draw, and the app matched both on the first run, so no code changed.

- **Channel Blur**: Red, Green, Blue and Alpha Blurriness (0 each), Repeat Edge Pixels (off) and
  Blur Dimensions (Horizontal and Vertical).
- **Unsharp Mask**: Amount (100), Radius (1) and Threshold (0). Threshold leaves alone any colour
  closer to its blurred copy than that many levels, so fine grain and flat skin are not crisped.

The check, `verification/D-363_channel_blur_unsharp_table.md` (109 of 109), holds every pixel to
those numbers, and the graphics card's picture to within 1 level of the processor's. The pictures
are in `verification/D-363 pictures/`.

## What to check

**`town.png`** is a street; **`grain.png`** is the same street with a fine grain laid over it.

1. **One channel.** `1_channel_blur_red_6.png` is red Blurriness 6 only: red should smear past
   the houses' edges and windows, while green and blue stay sharp, giving coloured fringes.
2. **Another.** `2_channel_blur_blue_6.png` is blue 6 only: blue fringes instead.
3. **All four.** `3_channel_blur_all_6.png` is all four at 6: a plain, even blur, no fringes.
4. **No threshold.** `4_unsharp_threshold_0.png` is the grainy street with Amount 300, Radius 2,
   Threshold 0: the grain is crisped into speckle all over the sky.
5. **Threshold 16.** `5_unsharp_threshold_16.png` is the same with Threshold 16: the sky's grain
   should look exactly as in `grain.png`, while the windows and road markings are still crisped.
6. **In the app.** Put Unsharp Mask on footage with grain, Amount about 200, and raise Threshold
   from 0 until the grain stops sharpening but the edges still do.

## Worth knowing

- Channel Blur's Blurriness here is a blur width in pixels. After Effects probably uses its
  Gaussian Blur scale, where the same number blurs about a third as much; this is not confirmed,
  so it is logged rather than changed. If a tutorial's number looks too strong, try a third of it.
- Unsharp Mask with a Threshold is drawn by the processor, not the graphics card (D-317).

Speed is unchanged (no code changed): `verification/B-223_gpu_blurs_timing_table.md` and
`verification/B-107_gpu_fx_timing_table.md`.
