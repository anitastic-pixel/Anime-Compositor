# B-777: more drawings kept in the viewer's memory

Built on 2026-10-26 under your "Do all of your picks at your recommended order." This is pick 6
of wave 11, D-897.

The viewer remembers the drawings it has read so it does not read them from disk again. It
keeps each one as large numbers, four times the size of the picture file's own 8-bit colours.
It still does while they fit. When memory is full, instead of forgetting a drawing it now keeps
just its 8-bit colours, a quarter of the size, so the same memory setting remembers many more
drawings. When such a drawing is needed, its large numbers are rebuilt from the 8-bit colours
with exactly the same sums the program uses after reading the file, so the picture is the same
to the last bit.

**What you should see:** nothing different in the picture. With a middling memory setting,
playing and moving the playhead around a long shot should feel quicker, because fewer drawings
are read from disk again. With the smallest setting and with your machine's Automatic setting
there is no difference in speed.

## What to check

1. Open the reference shot with ten layers. In Settings, under Memory, choose **Custom** and set
   **RAM, in GB** to 4.
2. Play the shot through twice, then click the playhead to scattered frames (for example 7, 200,
   45, 150, 90). Frames should come up quickly once the shot has played through, and the pictures
   look exactly as before.
3. Export frames 0 to 30 as PNG. They should be exactly the same as the same export before this
   build (export does not use this memory at all).
4. Set Memory back to **Automatic**.

## How long a frame takes

The processor at Draft, one ten-layer 1920 by 1080 frame, ms, the median of five rounds
alternating the old and new build, on a machine busy with other work
(`verification/B-777_timing_table.md`):

| Memory setting | Drawings kept, before / after | Playing, before / after | Scattered frames, before / after |
|---|---|---:|---:|
| 1 GB (the smallest) | 18 / 18 | 42.5 / 39.9 | 49.3 / 49.5 |
| 4 GB | 72 / 166 | 41.5 / 20.6 | 50.8 / 29.1 |
| 8 GB | 145 / 166 | 10.9 / 10.8 | 39.1 / 13.2 |
| 16 GB (about your Automatic) | 166 / 166 | 9.0 / 8.9 | 9.0 / 9.2 |

About twice as fast at 4 GB, and three times as fast jumping around at 8 GB. The other rows are
the same within the machine's wobble. A first version of this was slower at 1 GB and when
playing at 8 GB; it was reworked until nothing got slower (D-897 says how). If you would rather
not have it, it can be switched off and the viewer goes back to how it was before, with the
same pictures.

## What to report

"works", or the number of the step that did something else, what it did, and the frame number.
