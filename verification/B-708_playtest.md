# B-708 playtest: the T switch, Preserve Underlying Transparency (D-828)

What changed: every layer that draws has a **T** button in the timeline, in its own column after
Mode. With T on, the layer only shows where something is already drawn beneath it, like a
clipping mask in Photoshop. Where nothing is beneath, the layer is invisible. Null and adjustment
layers have no T button, as in After Effects. Old projects open and look exactly as before.

## What the pictures show

`B-708 pictures/fixtures_T_off_on_matte.png`, the test frames, enlarged:

- left, T off: the four coloured strips cover the whole frame, as before.
- middle, T on: the strips show only over the backdrop. Columns 8 and 9 have nothing beneath,
  so they stay clear (checkerboard). Where the backdrop is half or a quarter see-through, the
  strips are too.
- right, T on with an alpha track matte over columns 0 to 4: the matte cuts first, so from
  column 5 on you see the backdrop unchanged.

Verdict: matches the worked-out reference on all 10 cases (largest difference 6.9e-8, the limit
is 2e-5). See `B-708_preserve_transparency_table.md`.

`B-708 pictures/t1_smoke_T_three_way.png`, Ben Marriott's "2D Smoke & Explosions"
(https://www.youtube.com/watch?v=w8mHmW6GgHM), rebuilt by lane w7e, 8 bpc, frame 7:

| Panel | White share of the frame | White pieces |
| --- | --- | --- |
| After Effects at 9:45 (T on, Multiply noise) | 2.0% | 24 |
| before: the old app, same file (no T, noise everywhere) | 7.7% | 290 |
| after: T on (this build) | 1.7% | 5 |
| w7e's stand-in (an alpha matte instead of T) | 1.8% | 5 |

Verdict: **close**. With T the noise stays inside the clouds, as in After Effects; before, it
covered the whole frame. T and the stand-in agree on all but 0.044% of pixels (the soft cloud
edges). After Effects breaks the strokes into more pieces (24 against 5); that comes from the
rebuild's noise and cloud shapes, not from T, because the stand-in gives the same 5.

## Try it yourself (5 minutes)

1. Make a new composition. Add a solid, make it small with Scale (say 50%) so it does not fill
   the frame.
2. Add a second solid of another colour above it, full size.
3. In the timeline, find the **T** column (after Mode) on the top solid and click its T.
   Expected: the top solid now shows only inside the small solid's square; the rest of the
   frame goes back to empty (checkerboard).
4. Change the top solid's Mode to Multiply. Expected: still only inside the square, now
   multiplied.
5. Press Ctrl+Z. Expected: the mode goes back. Ctrl+Z again: T turns off and the top solid fills
   the frame again.
6. Add an adjustment layer and a null. Expected: neither shows a T button.
7. Turn T on again, save, close and reopen the project. Expected: T is still on.
8. Open any project from before this build. Expected: it looks the same as before, every T off.
