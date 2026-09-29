# B-131a: Posterize Time, a layer held for several frames at a time, on twos or threes (D-196)

Written on 2026-09-29, before any code. The sixth of the After Effects picks, A6, accepted with the rest by your "take everything". It does what After Effects' Posterize Time does, by a rule of our own.

## What you will see

A new effect, **Posterize Time**, in the **Time** folder after Echo. It makes a layer change only a few times a second, as if it had been drawn at a lower frame rate: at 12 a second in a 24 composition each picture is held for two frames, **on twos**; at 8, **on threes**. Its card has one row, in After Effects' words:

- **Frame Rate**, frames a second, 0.1 to 99, 12 when added. At or above the composition's own rate nothing is held. It can be keyed, so a layer can go from ones to threes mid-shot.

What is held is everything the layer draws: its drawing, solid, shapes or composition, its masks, and every effect on it, before and after Posterize Time, with every setting it has at the held frame, so a keyed glow or animated grain steps with the drawing. What is not held is where the layer is: its position, scale, rotation, opacity and motion blur keep moving every frame, as in After Effects. The holds are counted from the composition's first frame, so every layer at 12 a second changes on the same frames, and a layer that starts on an odd frame shows its own first drawing there, never a frame before it existed. On an adjustment layer it changes nothing.

`posterize_time.png` in this folder is worked by the rule itself: the ball from Echo's pictures bouncing on ones, frames 12 to 19 in a row; below it the same frames at 12 a second, each drawing shown twice; and at 8, each shown three times. Each held frame is labelled with the frame whose drawing it shows.

## How it differs from what we already have

**Exposure spans** (document 20) already put a drawn layer on twos or threes, drawing by drawing, and stay the way to time a drawn layer. Posterize Time holds anything, a composition, shapes or an effect's animation, with one number, and can be keyed. **Echo** (D-195) shows other frames as copies; Posterize Time shows one.

## Known limits

- A layer moved by its position, scale or rotation keeps moving smoothly. To put a movement on twos too, put the layer in a composition and posterize that, as in After Effects.
- On an adjustment layer it changes nothing, so it cannot put everything beneath it on twos; posterize a composition of the shot instead.
- Rates that do not divide the composition's evenly, such as 10 in 24, hold for uneven counts of frames, three then two.
- The frame rate is the composition's, however an export is timed.

## How you will check it

The build's test draws the seventeen fixture cases and the three wrong settings and compares every pixel with the numbers `tools/posterize_time_reference.py` worked out, and writes `verification/B-131_posterize_time_table.md`, with pictures. A playtest sheet walks you through putting a drawing on twos and threes and keying the rate.
