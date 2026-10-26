# B-776: Bilateral Blur and Cartoon quicker to export

Built on 2026-10-26 under your "Do all of your picks at your recommended order." This is pick 10
of wave 11, the written precision rule, D-896.

The processor can do its sums in two precisions. The rule now written down is: the ordinary one
by default, the finer one only where it is shown to be needed, and never the coarse "half" one.
Following it, **Bilateral Blur** (and **Cartoon**, which uses it to smooth) now works its
weights in the ordinary precision. Exports with these effects take about 11 to 17% less time at
large radii. Deband was tried too and left as it was, because 3 or 4 pixels in a whole frame came
out 1 level different (see D-896).

**What you should see:** nothing different, only quicker. The exported pictures are the same
to the last level: on the reference shot not one pixel of the 8-bit frame changed.

## What to check

1. Open the reference shot. Put **Bilateral Blur** on layer 2 from the Effects panel and set
   **Radius** to 40. The layer goes soft inside its colours while its lines stay sharp, as before.
2. Export frames 0 to 30 as PNG. They should look exactly as the same export did before this
   build, and finish a little sooner.
3. Delete it, put **Cartoon** on layer 2 and set **Detail Radius** to 40. Flat shaded colour with
   dark lines, as before. Export frames 0 to 30 again: the same look.

## How long a frame takes

The processor at Full (what an export uses), one 1920 by 1080 frame, ms, the median of five
rounds alternating the old and new build, on a machine busy with other work
(`verification/B-776_precision_table.md`, sitting 1):

| Effect | Before ms | After ms |
|---|---:|---:|
| Bilateral Blur, Radius 40 | 276.4 | 234.7 |
| Bilateral Blur as added | 41.3 | 37.8 |
| Cartoon, Detail Radius 40 | 288.0 | 255.6 |
| Cartoon as added | 65.4 | 60.4 |
| no effect (for comparison) | 16.6 | 16.9 |

## What to report

"works", or the number of the step that did something else, what it did, and the frame number.
