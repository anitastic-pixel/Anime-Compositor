# B-154: RAM preview, by hand

Built on 2026-09-30 under your "sounds great! let's do 1 through 7 to your discretion." This is
step G3 of the GPU plan. It is decision D-223.

Until now the viewer made every frame again each time it was shown, even on the tenth time
round a loop. Now it keeps each finished frame in memory, and a frame it has made once is sent
straight from memory the next time. While the shot plays, the viewer also makes the frames
ahead of the one on screen in the background, so after one pass the whole loop plays from memory.

**Nothing you see changes**, only how fast it arrives. A frame from memory is the same bytes as
the frame made fresh, with the same messages. The viewer checks before sending one: any edit,
undo, solo, a change of Draft or Full, of **Draw on**, or a drawing changed on disk makes it
draw the frame again. Exports and renders to file never use these frames.

The memory comes out of the **RAM** setting in Preferences, which is now shared. Finished frames
may take up to half of it; the unpacked drawings have whatever the finished frames are not using,
and never less than 1 GB. A full-size frame is about 8 MB, so 240 frames at Full need about 2 GB
and at Draft about 0.12 GB. On this computer Automatic gives 16 GB, so up to 8 GB of finished
frames: about 1000 frames at Full. When that is full, frames from earlier edits go first; a
loop longer than that keeps its first part.

## The checks

- `verification/B-154_ram_preview_table.md`: **37 of 37 pass.** Before this build 19 of 36
  passed (commit d7cb950); the 37th, added with the build, checks that the unpacked drawings
  give way to the finished frames. They check that a frame asked for twice is sent from memory the
  second time and is byte for byte the frame made the first time, with the same messages; that
  a different frame, Draft against Full, a solo, an edit, and drawings replaced on disk each
  make the frame again, and that it then matches a viewer that had remembered nothing; that
  undo brings back the remembered frame from before the edit; that after Play the other 239
  frames are made ahead, on the processor and on the card; and that the export code never
  names the remembered frames.
- Every other check in the program passes, with its table unchanged.

## Before you start

- Use the release build. It opens on the reference shot.
- Leave the viewer's **Draw on: Auto**, and Preferences' **Memory: Automatic**.

## What to check

1. **Two passes.** Set the viewer to **Full**. Press play and let the shot go round twice.
   When you stop, read the sentence under the viewer. After the second time round it should say
   "No frames were dropped" or very few, even if the first time round dropped some.
2. **Memory in use.** Open Preferences. Under Memory, "RAM: holding ..." now counts the
   finished frames too: after step 1 it should be about 2 GB more than before you played.
3. **An edit shows at once.** Stop on frame 100. Hide layer 1. The frame changes. Undo. It
   comes back, instantly.
4. **Solo.** Solo a layer on frame 100, then unsolo it. Each time the viewer shows the right
   picture.
5. **A drawing changed on disk.** With the shot open, replace one of layer 1's drawings in its
   folder with another picture of the same size, then step onto a frame that shows it. The new
   drawing shows, without reopening the project.
6. **Draft and Full.** Switch between Draft and Full while stopped. Each shows its own picture.
7. **Export.** Export a few frames as before. They are exactly what they were: exports never use
   the remembered frames.

## How long a frame takes

Measured on this computer (Ryzen 9 9900X, RTX 4070 Ti SUPER), release build, the build before
this one and this one three times each, turn about, nothing else building. Milliseconds for the
viewer's own work on one frame of a 240-frame loop at Full, the middle of the three runs. A frame
is due every 41.7 ms.

| Shot | Drawn on | Before, first pass | Before, second pass | Now, first pass | Now, second pass |
|---|---|---:|---:|---:|---:|
| Reference shot | processor | 15.2 | 15.0 | 16.1 | 0.9 |
| Document 08's ten layers | processor | 27.7 | 27.6 | 27.8 | 1.1 |
| Reference shot | graphics card | 2.4 | 2.3 | 3.4 | 0.9 |
| Document 08's ten layers | graphics card | 4.6 | 3.6 | 5.9 | 1.0 |

The ten-layer shot at Full on the processor drops 24 or 25 of its 240 frames the first time
round, before and now. The second time round it dropped none before either, but only just: now
each frame takes about a millisecond. The first pass costs up to about a millisecond more a frame
at Full, because the frame sent to the window is a copy of the one kept. Frames that come late on
the first pass are the same number as before.

While a shot plays, the frames ahead are made in the background: the whole 240-frame loop of the
ten-layer shot at Full in 7.2 seconds on the processor and 1.9 seconds on the card, faster than
its 10 seconds of playing time.

A first try gave the unpacked drawings only half the memory setting. That made the ten-layer
shot's first pass drop 85 frames instead of 24, so now they have whatever the finished frames
are not using.

The full table is `verification/B-154_timing_table.md`, and the build before this one
`verification/B-154_timing_before.md`.

## What to report

"works", or the number of the step that did something else, what it did, and the frame number.
