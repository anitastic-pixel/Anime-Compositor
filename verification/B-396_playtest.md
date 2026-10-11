# B-396: Stretch Wipe

Built on 2026-10-11 as D-516, under your /loop request: Stretch Wipe, in **Transition**, our own
take on CycoreFX's CC Scale Wipe (a name of our own). Everything past a line through the Center
is stretched along the Direction, so the line's own colours draw out in streaks. There is no
Completion: you make the wipe by keying the Center across the layer (stretching it off the
screen, or onto it), or by keying the Scale.

- **Scale** (100): how many times the far side is stretched, in per cent; 100 is no stretch.
- **Center** (the middle): where the line is.
- **Direction** (0): the way the stretch goes; 0 is to the right, 90 down.

The side behind the line is left as it is. The layer does not grow.

CycoreFX publishes no formula and no settings when added, so the rule and those settings are
ours, written down in document 21.

The check, `verification/D-516_stretch_wipe_table.md` (118 of 118), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-516 pictures/`.

## What to check

1. **The street.** `0_the_street.png`: the shot without the effect.
2. **Stretched.** `1_stretched.png`: scale 300: the left half as it was, the right half drawn
   out three times wider.
3. **Down.** `2_down.png`: scale 200, direction 90: the top half as it was, the bottom half
   drawn out downward.
4. **Turned.** `3_turned.png`: scale 175, direction -135, Center at 60, 40: stretched up and to
   the left, the bottom right as it was.
5. **Off the left.** `4_off_the_left.png`: scale 150, Center past the left edge: the whole
   street stretched, as the wipe looks when the Center moves off.
6. **Streaks.** `5_streaks.png`: scale 100000: the right half one streak of colour per row.
7. **In the app.** Add Stretch Wipe (Transition) to a layer over another, set Scale to 1000 and
   keyframe the Center from the right edge to past the left, and play. Try a Direction.
8. **Out of range.** Type 50 in Scale: it is refused with a sentence saying the range.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; a wipe made by keying the Center,
with no Completion, as CycoreFX's own controls have.

## Speed

`verification/B-396_stretch_wipe_timing_table.md`. Three Stretch Wipes on a 1920 by 1080 shot, milliseconds a frame played
again: PROVISIONAL (machine not quiet, other lanes' tests and builds running), card 27.9, 29.6 and 77.5 ms a frame (scale 300; scale 100000, direction 90; scale 175, direction -135, Center [60, 40]) against 20.2 for the Noise alone, processor 183.8, 76.7 and 143.1 against 56.2; a quiet re-measure is owed.
