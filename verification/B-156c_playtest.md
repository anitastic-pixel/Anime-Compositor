# B-156c: frame mix and drawing dissolve made quicker

Built on 2026-09-30 under your "sounds great! let's do 1 through 7 to your discretion." This is
the last part of item 5 of the GPU plan that was built, D-227.

Frame mix (a slowed-down layer softly mixing two drawings) and drawing dissolve (a held drawing
fading into the next) are still made by the processor. They are now made about a third quicker:
the processor shares the work among its threads, and no longer copies a drawing it only reads.

**Nothing you should see changes, not even by one level.** Every exported frame of the reference
shot with frame mix and dissolve, all 240, is the same number for number as before the build.
The frame blending and whole-frame tables (`verification/B-150_frame_blending_table.md`,
`verification/B-152_card_whole_frame_table.md`) came out unchanged.

## What to check

1. **Frame mix.** Open the reference shot. Select layer 2, type 150 in **Time stretch** and tick
   **Frame mix**, then turn on **Frame blending** over the timeline. Step through a few frames:
   some show two drawings softly mixed, as before.
2. **Dissolve.** Select layer 4 and set **Drawing dissolve** to 2. Where one of its drawings is
   held for three or five frames, the last frames of the hold fade into the next drawing.
3. **Play.** Press play at **Full**. It should feel smoother than before, with nothing flickering.

## How long a frame takes

Measured on this machine, release build, the median of three runs
(`verification/B-156c_timing_table.md`):

| Shot | Quality | Before ms | After ms |
|---|---|---:|---:|
| the reference shot with frame mix and dissolve | Draft | 46.0 | 31.3 |
| the reference shot with frame mix and dissolve | Full | 61.8 | 44.8 |

## What to report

"works", or the number of the step that did something else, what it did, and the frame number.
