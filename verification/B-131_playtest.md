# B-131: Posterize Time, by hand

Built on 2026-09-29 against D-196, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Posterize Time does, by a rule of our own: it
makes a layer change only a few times a second, as if drawn at a lower frame rate. At 12 a
second in a composition of 24 each picture is held for two frames, **on twos**; at 8, **on
threes**. What is held is everything the layer draws, its drawing, masks and effects; where the
layer is, its position, scale, rotation and opacity, keeps moving every frame. It sits in the
**Time** group after Echo.

The generated halves are `verification/B-131_posterize_time_table.md`, 116 of 116 checks
passing, which renders every FX-PTIME case against the numbers written before the code and draws
the pictures below, and `verification/B-12b_state_fields_table.md`, which checks the Posterize
Time card sends the setting the command reads. This sheet covers what the tables cannot: how it
looks and feels in the window.

## The pictures

`verification/B-131 pictures/`, Echo's ball bouncing left to right on ones, 24 drawings at a
quarter of 1920 by 1080:

- `ball_01.png` to `ball_24.png`: the drawings.
- `strip.png`: frames 12 to 19 left to right, in three rows. Top, no effect: the ball moves every
  frame. Middle, Posterize Time 12: each ball shown twice. Bottom, Posterize Time 8: each ball
  shown three times, then the next.

## Before you start

Make the composition 480 by 270, 24 frames a second, 24 frames long. Import all 24
`ball_##.png` files in `verification/B-131 pictures/` together, so they come in as one layer, a
drawing a frame. Press **Full resolution**.

## What to check

1. **Adding it.** On the ball layer, pick **Posterize Time** in **Add effect…**, under **Time**,
   after Echo; typing "posterize time", "twos", "threes" or "choppy" in the search finds it too.
   The card shows one row, **Frame Rate** 12.
2. **On twos.** Step through frames 12 to 19 with the arrow keys: the ball moves on every second
   frame, as in the middle row of `strip.png`. Play: the ball moves in steps.
3. **On threes.** Set Frame Rate 8: the ball moves on every third frame, as in the bottom row.
4. **Nothing held.** Set 24, or anything above it: the ball moves every frame again.
5. **The start.** Set 12, trim the layer so it starts at frame 3 and go to frame 3: the layer's
   ball of frame 3 shows, never an empty frame from before the layer began.
6. **Movement is not held.** Undo the trim. Key the layer's position from left to right across
   the 24 frames: the ball's drawing still steps on twos, but the layer glides every frame.
7. **Effects are held.** Add a Noise after the Posterize Time with Animate on: the grain changes
   only when the drawing does. Drag the Noise above the Posterize Time: the same, since every
   effect on the layer is held, wherever it sits.
8. **Adjustment layer.** Put a Posterize Time on an adjustment layer above the ball: nothing
   changes.
9. **Out of range.** Type 0.05 in Frame Rate: it is refused with a sentence saying it runs from
   0.1 to 99, and the card keeps its old number. The same for 100.
10. **Keyed.** Key Frame Rate from 24 at frame 0 to 8 at frame 23 and play: the ball starts
    smooth and ends on threes.
11. **Draft.** Press **Draft**: the picture is smaller and still steps on twos.
12. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the setting
    and its keys are still there.

## Known limits, on purpose

- A layer moved by its position, scale or rotation keeps moving smoothly, as in After Effects.
  To put a movement on twos too, put the layer in a composition and posterize that.
- On an adjustment layer it changes nothing, so it cannot put everything beneath it on twos;
  posterize a composition of the shot instead.
- Rates that do not divide the composition's evenly, such as 10 in 24, hold for uneven counts
  of frames, three then two.
- The frame rate is the composition's, however an export is timed.
- A missing drawing is reported against the frame you are looking at, even when that frame
  shows a held drawing from an earlier frame.
- It costs nothing of its own: a held frame is drawn once, as any frame is.
  The reference shot's frames 100 and 101 at full size take 50 to 54 ms without it, and 47 to
  52 ms with Posterize Time 12 on one of its four layers or on all four.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
