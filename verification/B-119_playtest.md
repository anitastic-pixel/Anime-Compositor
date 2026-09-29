# B-119: Line Blur, by hand

Built on 2026-09-28 against D-183, which you accepted as the fifth of the nine ("1, 2,3,5,6,7,8,9",
then "include 4 as well, just 1 thru 9"). It is this program's own rule, inspired by OpenToonz's
Line Blur and not a port.

The generated halves are `verification/B-119_line_blur_table.md`, 69 of 69 checks passing, which renders every
FX-LBLUR case against the numbers written before the code and draws the pictures below, and
`verification/B-12b_state_fields_table.md`, which checks the Line Blur card sends every setting the
command reads. This sheet covers what the tables cannot: how it looks and feels in the window.

## The pictures

`verification/B-119 pictures/`, each three times enlarged so the pixels show:

- `before.png`: a jagged ink drawing, black strokes on white paper with no smoothing at all, the
  steps a hard pen or a scan leaves: a ring, a shallow line, a steep line and a wave.
- `length_4.png`: Line Blur as it is added, Length 4. The steps along the ring and the lines turn
  into smooth slopes. The lines keep their thickness, their ends stay where they were, and the
  white paper away from the lines stays white.
- `length_12.png`: Length 12, a longer pass. The straight lines are smoother still. The ring and
  the wave bend too much inside 12 pixels, so their ink spreads off the curve: they turn grey
  and the wave leaves a faint grey shadow on the paper beside it. That is the first known limit
  below, shown on purpose.
- `lines_only.png`: Length 4 with Lines Only on. Only pixels with ink move, so the result is a
  touch crisper than `length_4.png`, and the paper cannot change at all.

## Before you start

Import `verification/B-119 pictures/drawing.png`, make a layer from it and press
**Full resolution**.

## What to check

1. **Adding it.** Pick **Line Blur** in **Add effect…**, under **Lines & Mattes**, next to
   Line Width. The card shows **Length** 4, **Strength** 100 and **Lines Only** Off. The picture
   looks like `length_4.png`: the steps soften into slopes, the lines keep their width and the
   paper stays white.
2. **Longer.** Set Length to 12: the straight lines are smoother still, and the ring and the wave
   go grey. It looks like `length_12.png`.
3. **Strength.** Set Strength to 50: the smoothing is half as strong, between the drawing and
   step 2. Set it back to 100.
4. **Lines only.** Set **Lines Only** to On: it looks like `lines_only.png`. Set it back to Off.
5. **Nothing.** Set Length to 0: the drawing is exactly as imported. Set it back to 4.
6. **Out of range.** Type 60 in Length: it is refused with a sentence saying the length runs
   from 0 to 50, and the card keeps its old number.
7. **Keyed.** Key Length at 0 at the first frame and at 8 a few frames later. Scrub between: the
   smoothing grows steadily, with no jumps.
8. **Draft.** Press **Draft**: the picture is smaller, and the smoothing looks about the same
   compared to the drawing as at full resolution.
9. **A drawing on nothing.** Import the face drawing, `verification/B-31c_selective_blur_drawing.png`,
   make a layer and add Line Blur. The face's outline softens where it steps, its straight sides
   and flat colours stay as they were, and no halo appears around it.
10. **Undo and saved.** Ctrl+Z steps back each change, the Lines Only choice included. Save,
    close and open again: every setting and key is still there.

## Known limits, on purpose

- It follows each line in a straight direction for its whole length, so on a very tight curve
  (a ring only a few pixels across) a long Length takes a little of the ink off the curve. Keep
  the length near the line's own smallest bend.
- The edge of a layer counts as a line too. An opaque picture's own corners, where it meets
  nothing, round off by a pixel or two.
- It is drawn on the processor. On this machine a full 1920 by 1080 frame of the reference shot
  with Line Blur on all four layers took about 113 ms at Length 4, 135 ms at Length 12 and
  250 ms at Length 50, against 52 ms with no Line Blur (frame 100, three runs each, the best
  and worst within 5 ms). Item 9 of the batch moves it to the graphics card.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
