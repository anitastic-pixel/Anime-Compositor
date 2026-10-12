# B-443: Shadow/Highlight's Midtone Contrast and Black/White Clip

Built on 2026-10-11 as D-563, under your /loop request. Shadow/Highlight (Color Correction) gains
the three sliders it was missing. Midtone Contrast adds contrast to the middle tones, or takes it
away below 0. Black Clip and White Clip stretch the picture so the darkest and lightest few per
cent reach the picture's own extremes, which gives a punchier picture. All three start at 0, so
projects saved before today look exactly as they did.

The check, `verification/D-563_shadow_highlight_clip_table.md` (139 of 139), holds every pixel
to numbers a separate program worked out before the code was written, and holds the graphics
card's picture to the processor's (42 of 42 within 1 level). The pictures are in
`verification/D-563 pictures/`, made from the town street.

## What to check

1. **Before.** `1_before.png`: the street.
2. **As added.** `2_as_added.png`: Shadow/Highlight as it starts: the dark parts a little
   lighter. The three new sliders at 0 change nothing.
3. **More midtone contrast.** `3_midtone_80.png`: as 2, with the middle tones pushed apart: the
   buildings' colours a touch richer, darks a touch darker, lights a touch lighter.
4. **Less midtone contrast.** `4_midtone_minus_80.png`: the middle tones drawn together: a
   flatter, greyer look than 2.
5. **Clips.** `5_clips_5.png`: both clips 5: the sky's lightest part goes to white and the
   darkest windows and road go darker than in 2.
6. **Without the lift.** `6_no_lift.png`: both amounts 0, midtone contrast 50, both clips 1:
   the new sliders work on their own.
7. **In the app.** Add Shadow/Highlight. Drag Midtone Contrast both ways: the picture gains and
   loses punch. Raise Black Clip or White Clip from 0: the darks or lights stretch out. The
   panel's hint says a clip above 0 is drawn on the processor, so playback may slow.

Not settled: Adobe publishes no rule for these three, so how they work is this program's own,
and the starting values are 0 where After Effects is remembered to start both clips at 0.01 per
cent. Starting at 0.01 would keep the effect off the graphics card for every project; say if you
want After Effects' numbers anyway.

## Speed

`verification/B-443_shadow_highlight_clip_timing_table.md`: provisional (other lanes' builds and
tests were running throughout): on the card the reference shot with a moving Noise took 19.4 ms a
frame, 43.2 with Shadow/Highlight as added, 29.0 with midtone contrast 60 and 281.1 with both
clips 1 (drawn on the processor); on the processor 42.1, 175.5, 179.9 and 256.8.
