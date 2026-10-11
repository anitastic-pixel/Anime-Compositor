# B-337: Advanced Spill Suppressor

Built on 2026-10-10 as D-457, under your /loop request: Advanced Spill Suppressor, in
**Lines & Mattes** (beside the keyers), with After Effects' controls. It takes back out the green
or blue cast a green or blue screen throws onto the subject you keyed out of it: the fringe on
hair and edges, the tint on skin.

- **Method** (Standard): Standard works by itself: wherever green or blue is the strongest of the
  three colours, it is brought down to the next strongest. Ultra works from a **Key Color** you
  choose (pure green when added) and uses the controls below.
- **Suppression** (100): how much of the spill is taken out.
- **Tolerance** (50, Ultra): how far round the colour wheel from the key colour still counts as
  spill.
- **Desaturate** (25, Ultra): how grey a cleaned colour turns.
- **Spill Range** (50, Ultra): how hard the screen colour is cut back.
- **Spill Color Correction** (50, Ultra): how much of what is taken out goes back as the other two
  colours, so a cleaned edge does not go dull.
- **Luma Correction** (50, Ultra): how much of the lost brightness is put back.

Adobe publishes only the control names and what they are for, so the rule, the ranges and the
starting values are our own, written down in document 21. Our pictures will not be exactly Adobe's.

The check, `verification/D-457_advanced_spill_suppressor_table.md` (184 of 184), holds
every pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-457 pictures/`.

## What to check

1. **The shot.** `1_before.png`: a green screen, a face whose edge has picked up the screen's
   green, a blue shirt.
2. **Standard.** `2_standard.png`: the screen turns grey, the green fringe round the face is gone,
   the face itself is unchanged. The blue shirt also loses its blue: Standard cannot tell a blue
   shirt from a blue screen. That is its known limit (Adobe's Standard looks at the whole frame;
   ours looks at each pixel alone).
3. **Ultra.** `3_ultra_green_key.png`: the screen and fringe cleaned the Ultra way (a little of
   the green given back as red and blue), and the blue shirt untouched, because blue is far from
   the green key colour.
4. **Colour given back.** `4_ultra_correction.png` (Spill Color Correction 100, Desaturate 0,
   Luma Correction 0): everything taken from green goes to red and blue, so the cleaned screen turns
   a muted pink.
5. **A shot with no screen.** `5_street_standard.png`: Standard greys the blue sky, so it is for
   keyed shots only. `6_street_ultra.png`: Ultra with the green key leaves the sky and the red and
   blue walls alone and only touches colours near green.
6. **In the app.** Key a green-screen shot (Colour Key or HSV Key), then add Advanced Spill
   Suppressor (Lines & Mattes): the green fringe goes. Switch Method to Ultra and pick the screen's
   colour as Key Color; raise Tolerance to reach more yellow-greens and blue-greens. Set
   Suppression to 0: the picture is back as it was.
7. **Out of range.** Type 150 in Suppression: it is refused with a sentence saying the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: how wide Tolerance 50 reaches (about 90 degrees round the
colour wheel each side), and how much colour and brightness Ultra gives back when added.

## Speed

Measured on a quiet machine
(`verification/B-337_advanced_spill_suppressor_timing_table.md`). Three Advanced Spill Suppressors on a 1920 by 1080 shot, card /
processor, milliseconds a frame played again: the shot after a moving Noise 12.1 / 45.3 ms a frame, Standard 17.5 / 57.7, Ultra 18.1 / 58.8; over Target P1 (1 ms or less for a one-pixel colour change): about 1.8 to 2.0 ms a 1080p layer on the card; on the processor about 4.1 to 4.5 ms a layer.
