# B-171b: a composition inside another saved to disk in the background

Built on 2026-09-30 under your "sounds great! let's do 1 through 7 to your discretion." This is a
follow-up to B-171 (G12, D-243), in the same bit-exact form: D-245.

B-171 made the viewer keep a heavy composition inside another, in memory and on disk, so playing
it again or reopening the program is much quicker. But it wrote each frame's copy to disk while
you waited, so the very first play at Full got slower (55.1 ms a frame against 34.8 before). Now a
helper writes the copies in the background while the next frames are drawn. **The saved copies
are byte for byte what B-171 wrote**, and the pictures byte for byte the export's:
`verification/B-171b_background_table.md` (3 of 3) and `verification/B-171_precomp_table.md`
(42 of 42). Exports never use this and are untouched.

At Full the first play is now 27.5 ms a frame, against 34.8 before B-171 and 55.1 with B-171
(`verification/B-171b_timing_table.md`). Playing again, editing the outer layer and reopening the
program are as quick as B-171 made them.

One trade: if eight frames are already waiting to be written, a new frame is kept in memory only,
not on disk, so the picture never waits for the disk. On a slow disk, reopening the program could
then find fewer frames saved and draw those again.

## What to check

Open the reference shot. Set **Draw on: GPU** and **Full resolution**. Put a strong Gaussian Blur
on two of its layers, then put the whole shot inside another composition (as for B-171).

1. **First play.** Play the outer composition from the start. It should play at least as smoothly
   as before B-171, with no stutter as frames are saved.
2. **Play again.** Play it again. It should be quicker still.
3. **Reopen.** Close the program as soon as the first play ends, open it again with the same
   project, and play. The frames should come back quickly and look the same. (The last few frames
   may not have been saved in time; those are simply drawn again.)
4. **Edit inside.** Change a blur inside the inner composition and play. The picture should show
   the change at once.

## If something is wrong

Say which step. The most likely fault would be step 3: a frame that looks wrong after closing the
program just after the first play, because copies were still being written as it closed. A copy
is written under another name and renamed only when whole, and every copy is checked before use,
so a half-written one should never be read. The check also reads a frame back at once, while it is
still being written, and gets it bit for bit.
