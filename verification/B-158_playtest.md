# B-158: only the part of the picture on screen

Built on 2026-09-30 under your "sounds great! let's do 1 through 7 to your discretion." This is
item 8 of the GPU plan (G8), the part you approved to build: D-229.

When you zoom in so that only part of the picture fits in the viewer, the window now makes and
sends only that part (and a little around it), instead of the whole frame. **The pixels you see are
byte for byte the ones the whole frame has**: `verification/B-158_part_table.md` checks 6,548 parts
against whole frames, on the processor and the card, and all are identical. Exports never use it.

It only saves time when the processor draws the frame: about half the time a frame at Full when
the middle quarter shows (21.6 to 11.3 ms on the reference shot, `verification/B-158_timing_table.md`).
When the graphics card draws, the card still draws the whole frame.

Not built, as agreed: drawing a smaller picture when zoomed out below 100%. That would change the
pixels, so it stays a proposal (D-229) and needs your say.

## What to check

Open the reference shot and set **Draw on: CPU**, then **Full resolution**.

1. **Zoom in.** Drag the zoom slider to about 200%, or roll the mouse wheel over the picture. The
   picture should look exactly as it did, only bigger. No black or empty band should stay at an
   edge of the viewer.
2. **Scroll around.** Scroll the viewer left, right, up and down. For a moment a newly revealed
   strip at an edge may show the viewer's empty background, then fill in with the picture. It must
   fill in each time, and join the rest with no seam, step or shift.
3. **Step frames.** Press the arrow keys to step a few frames while zoomed in. Each frame should
   appear as sharply as before, and quicker.
4. **Zoom back out.** Press **Fit**. The whole picture should come back at once, complete, with no
   missing part.
5. **Play.** Press Play while zoomed in, then stop. Playback sends whole frames as before, so it
   should look and run as it did.
6. **The card.** Set **Draw on: GPU** and repeat 1 to 4. It should look the same as on the CPU.

## If something is wrong

Say which step, and whether you were on CPU or GPU, Full or Draft. The most likely fault would be
an empty strip that does not fill in after a scroll or a zoom (step 2 or 4): the page decides when
to ask again, and that part is not covered by the byte checks.
