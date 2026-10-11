# B-400: CC Wide Time

Built on 2026-10-11 as D-520, under your /loop request: CC Wide Time, in the **Time** group,
with CycoreFX's settings. It smears a moving layer by averaging it over a few frames before and
after the present one, every frame counted the same: a temporal blur for anything moving over a
still background, or for smoothing jumpy timelapse footage.

- **Forward Steps** (3): how many frames after the present one are averaged in.
- **Backward Steps** (3): how many frames before.
- **Native Motion Blur** (Off): Off draws the compositions inside the layer without their own
  motion blur, so nothing is blurred twice; On draws them as they are.

It reads the layer as drawn, its masks included but its effects not run, as CycoreFX's manual
says (effects meant to be smeared go inside a precomposition). A frame before the layer starts
or after it ends counts as empty, so the smear fades near the ends.

CycoreFX's manual describes the controls but gives no formula and no starting values, so the
rule and those values are ours, written down in document 21. Only whole frames are averaged:
something moving fast shows as separate faint copies rather than one smooth streak.

The check, `verification/D-520_cc_wide_time_table.md` (94 of 94), holds every
pixel to numbers worked out by a separate program before the code existed. The pictures are in
`verification/D-520 pictures/`: an orange ball bouncing on ones over a night-blue solid, frame 16.

## What to check

1. **Before.** `1_before.png`: one ball, sharp.
2. **As added.** `2_as_added.png`: seven faint balls, drawings 14 to 20, along the bounce, the
   present one in the middle no brighter than the others.
3. **Behind.** `3_behind.png`: Forward 0, Backward 6: the faint balls trail behind, from drawing 11, ending at the
   present one.
4. **Ahead.** `4_ahead.png`: Forward 4, Backward 0: the faint balls run ahead to drawing 21, where it is going.
5. **Draft.** `5_as_added_draft.png`: as added at Draft, a quarter the size, the same picture.
6. **In the app.** Add CC Wide Time (Time) to a moving layer and play: the layer smears along
   its path. Set both steps to 0: the layer is exactly as it was.
7. **Out of range.** Type 101 in Forward Steps: it is refused with a sentence saying the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: 3 and 3 as the starting span, and every frame weighted
the same.

## Speed

Measured while another test was running, so PROVISIONAL (`verification/B-400_cc_wide_time_timing_table.md`). Three CC Wide Times
on a 1920 by 1080 shot, Draw on: GPU, milliseconds a frame played again: the shot with a moving
Noise 14.5; with CC Wide Time as added 474.0; with 10 and 10 1194.2. That is about 150 ms a
1080p layer as added, above the 100 ms target: playing it in the viewer will be slow.
