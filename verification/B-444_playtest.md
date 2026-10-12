# B-444: Shadow/Highlight's Auto Amounts, Temporal Smoothing and Scene Detect

Built on 2026-10-11 as D-564, under your /loop request. Shadow/Highlight (Color Correction) gains
its last three settings. **Auto Amounts** picks how much to lift the shadows and pull down the
highlights from the picture itself, instead of the two sliders: a dark picture is lifted more, a
bright one has its highlights brought down. **Temporal Smoothing** judges each frame together with
the frames either side, so the amounts do not jump from frame to frame. **Scene Detect** stops that
at a cut, so a new shot is not judged by the old one. Auto Amounts starts off, so projects saved
before today look exactly as they did. With it off, the other two do nothing.

The check, `verification/D-564_shadow_highlight_auto_table.md` (115 of 115), holds every pixel
to numbers a separate program worked out before the code was written, and holds the graphics
card's picture to the processor's (32 of 32 within 1 level). The pictures are in
`verification/D-564 pictures/`, made from the town street.

## What to check

1. **Before.** `1_before.png`: the street.
2. **As added.** `2_as_added.png`: Shadow/Highlight as it starts: the dark parts a little
   lighter, from the sliders.
3. **Auto Amounts.** `3_auto.png`: Auto Amounts on, with the shadow slider at 90, which is not
   used. The street is bright, so very little lift: its lightest parts come down a touch and
   the picture looks close to 1.
4. **A dark picture.** `4_dark_before.png`: the street at half its brightness.
   `5_dark_auto.png`: with Auto Amounts on: lifted clearly more than 3 was (its average brightness
   rises by about 8 levels of 255, against about 2 lower on the bright street).
5. **In the app.** Add Shadow/Highlight. Set Auto Amounts to "On": the picture changes by itself,
   and dragging the Shadow or Highlight Amount sliders does nothing. The choice says it runs on the
   processor, so playback may slow. On a moving shot, set Temporal Smoothing to 0.5: the lift
   should settle rather than flicker. Turn on Scene Detect on a shot with a cut: the frames after
   the cut should not borrow the look of the frames before it.

Not settled: Adobe publishes no rule, so how Auto Amounts picks its amounts is this program's own.
After Effects is remembered to start with Auto Amounts on; ours starts off, because on keeps the
effect off the graphics card for every project. Say if you want After Effects' starting setting
anyway.

## Speed

`verification/B-444_shadow_highlight_auto_timing_table.md`: provisional (other lanes' builds and
tests were running throughout): on the card the reference shot with a moving Noise took 12.8 ms a
frame, 24.1 with Shadow/Highlight as added, 198.3 with Auto Amounts on and 473.7 with Auto Amounts
on and temporal smoothing 0.1 (both drawn on the processor); on the processor 55.1, 211.3, 253.3
and 567.2.
