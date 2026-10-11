# B-406: Numbers

Built on 2026-10-11 as D-526, under your /loop request: Numbers, in the **Text** group. It draws
a number on the layer: a value you can key so it counts, the layer's timecode, a time of day, a
date or a hexadecimal number, or a random number each frame, in white or any colour, with an
outline if you like.

- **Type** (Number): Number, Number [Leading Zeros] (00042), Timecode [30], [25] or [24], Time,
  Numerical Date, Short Date, Long Date, Hexadecimal.
- **Random Values** (Off): On picks a new number every frame, up to Value (or 30,000).
- **Value/Offset/Random Max** (0): the number; seconds added to the timecode; hours for Time;
  days after 1 January 1995 for the dates. Key it to count.
- **Decimal Places** (0), **Position** (the middle), **Size** (64), **Tracking** (0),
  **Alignment** (Center), **Font** (the font that comes with the program).
- **Display Options** (Fill Only), **Fill Color** (white), **Stroke Color** (orange-red),
  **Stroke Width** (3): fill, outline, or both.
- **Proportional Spacing** (On): Off puts every character in an even cell, so a counter does not
  jiggle as its digits change.
- **Composite On Original** (On): Off shows the words alone on transparent.

The check, `verification/D-526_numbers_table.md` (252 of 252), holds every frame to a
picture of the same words made from ordinary text layers by a separate program before the code
existed, and holds the card to the processor within 1 level. The pictures are in
`verification/D-526 pictures/`.

## What to check

1. **As added.** `as_added.png`: a white 0 in the middle of a dark blue picture.
2. **Places.** `two_places.png`: 1234.57.
3. **Timecode.** `timecode_30.png`: 00:00:02:02 (the layer's time at frame 50, at 30 frames a
   second).
4. **Date.** `long_date.png`: Monday, January 1, 1996, smaller, from the left.
5. **Hexadecimal.** `hexadecimal.png`: 18000.
6. **Random.** `random.png`: 90.47.
7. **Counter.** `counter_frame_4.png`: 40 (keyed from 0 to 100).
8. **Outline only.** `stroke_only.png`: 2026 as an orange ring, the picture showing through the
   letters.
9. **Fill and outline.** `fill_over_stroke.png` and `stroke_over_fill.png`: 2026 in white with a
   thick orange edge; the two look the same, since the outline lies outside the letters.
10. **Even cells.** `monospaced.png` (1.11, the point as wide as a digit) and
    `monospaced_timecode.png` (00:00:01:13 with an orange edge, from the left).
11. **Alone.** `composite_off.png`: a light blue 88 with a dark blue edge on black.
12. **Missing font.** `missing_font.png`: the plain picture, and a warning.
13. **In the app.** Add Numbers (Text) to a layer and key Value from 0 to 100 over two seconds:
    it counts. Turn Proportional Spacing off: the digits stop jiggling.
14. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the font is typed as a file name (no font menu yet), and
there is no Current Time/Date (it would make every render different).

## Speed

On a 1920 by 1080 still at Full, a frame Numbers has not drawn before took about 27 milliseconds with the timecode at size 200 and about 800 with the stroked long date at size 120, against about 6 without it; a frame drawn before took about 3 (PROVISIONAL: the machine was busy). The stroked long date is slow to draw the first time (`verification/B-406_numbers_timing_table.md`)
