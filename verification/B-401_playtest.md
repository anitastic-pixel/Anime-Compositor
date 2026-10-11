# B-401: Time Difference

Built on 2026-10-11 as D-521, under your /loop request: Time Difference, in the **Time** group,
with After Effects' settings. It shows how the layer differs from a target, usually the layer
itself a frame or more earlier: whatever moved lights up and whatever stood still goes black.
Useful for motion mattes, for comparing footage with a clean plate, and for trails.

- **Target** (none): the layer to compare with. None means the layer itself. The target's eye
  switch may be off.
- **Time Offset** (0 seconds): how far from now the target is read. -0.042 is one frame back at
  24 a second.
- **Contrast** (0): strengthens every difference.
- **Absolute Difference** (On): On shows no difference as black; Off shows it as mid grey, lighter
  where the layer got lighter and darker where it got darker.
- **Alpha Channel** (Original): which covering the result keeps; nine choices, as in After
  Effects.

It reads the layer as drawn, its effects not run. A frame before the layer starts or after it
ends counts as empty.

Adobe's help describes the controls but gives no formula and no starting values, so the rule and
those values are ours, written down in document 21.

The check, `verification/D-521_time_difference_table.md` (140 of 140), holds every pixel
to numbers worked out by a separate program before the code existed. The pictures are in
`verification/D-521 pictures/`: an orange ball bouncing on ones over a night-blue solid, frame 16.

## What to check

1. **Before.** `1_before.png`: one ball.
2. **What moved.** `2_moved.png` (one frame back, Alpha Channel Full On): black, with two balls
   lit where the ball was a frame ago and where it is now.
3. **Grey.** `3_grey.png`: the same with Absolute Difference off: mid grey, the ball's new place
   light, its old place dark.
4. **Stronger.** `4_strong.png`: as 2 with Contrast 30, the balls brighter.
5. **Draft.** `5_moved_draft.png`: as 2 at Draft, a quarter the size, the same picture.
6. **In the app.** Add Time Difference (Time) to a moving layer, set Time Offset to -0.042 and
   play: only the moving parts show. Set Time Offset back to 0: everything goes black.
7. **Out of range.** Type 101 in Contrast: it is refused with a sentence saying the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the starting values (nothing shows until Time Offset is
set) and how strongly Contrast acts (Contrast 10 doubles every difference).

## Speed

On the reference shot at Full a frame took about 40 to 70 milliseconds with Time Difference as added on three layers and about 85 to 100 one frame back, against about 14 to 18 without it (PROVISIONAL: the machine was busy when the run ended) (`verification/B-401_time_difference_timing_table.md`)
