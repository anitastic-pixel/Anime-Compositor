# B-171: a composition inside another kept

Built on 2026-09-30 under your "sounds great! let's do 1 through 7 to your discretion." This is
item 12 of the GPU plan (G12), which you added to the queue "to your discretion": D-243.

A composition shown inside another (a "precomp") used to be drawn again every time the outer shot
needed it: on every frame, after every edit to the outer shot, and again after the program was
opened. Now the preview keeps each frame of it once drawn, in memory, and on disk too when it was
slow to draw, and draws it again only when something inside it changes: one of its layers, one of
its drawings on disk, or the program itself. **The pixels are byte for byte what they were**:
`verification/B-171_precomp_table.md` compares 42 frames with the export's; all are identical.
Exports never use it.

On the reference shot with two blurs, shown inside another composition, at Full: playing it again
went from 16.1 to 2.0 ms a frame, playing it after an edit to the outer layer from
15.8 to 1.9 ms, and the first play after opening the program again from 31.8 to 13.5 ms
(`verification/B-171_timing_table.md`). The very first play at Full is a few milliseconds slower
(40.5 to 46.7 ms), the time it takes to write each frame to disk.

The copies on disk share the setting in **Preferences, Drawings and compositions on disk**: the
same folder and the same most in GB. Setting it to 0 keeps compositions in memory only.

## What to check

Make a composition that shows another one: make a new composition and drag the reference shot's
composition into it. Give the reference shot a Gaussian Blur on one layer so it is slow to draw.
Set **Draw on: GPU** and **Full resolution**, open the outer composition.

1. **Play twice.** Play the outer composition to the end, then play it again. The second time
   should be smoother, and look exactly the same.
2. **Edit the outside.** Move or fade the layer that shows the inner composition. The picture
   should follow at once, and the inner composition should look as it did.
3. **Edit the inside.** Open the inner composition, change something (a layer's opacity, the
   blur's size), go back to the outer one. The change should show on every frame, not just the
   one you were on.
4. **Close and open.** Close the program, open the project again and play the outer composition.
   It should start smoothly sooner than it did the first time, and look the same.
5. **Change a drawing file.** With the program open, replace one of the inner composition's
   drawings on disk with another picture (or save over it in a paint program). Step to a frame
   that shows it: the new drawing should appear.
6. **Draft.** Switch to **Draft** and repeat step 1.

## If something is wrong

Say which step. The most likely fault would be an old picture of the inner composition shown after
an edit (steps 3 and 5). The checks cover an edit inside the inner composition and a drawing file
dated anew; a file replaced with the same size and the same date is the one change it cannot see,
as for every other copy the preview keeps.
