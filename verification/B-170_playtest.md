# B-170: a held masked drawing kept

Built on 2026-09-30 under your "sounds great! let's do 1 through 7 to your discretion." This is
item 11 of the GPU plan (G11), which you added to the queue "to your discretion": D-242.

Animation is mostly drawings held for two or three frames. When a layer had a mask, every one of
those frames drew the mask again on the processor and sent the result to the graphics card again,
even though the drawing had not changed. Now the masked drawing is kept, so a held frame of a
masked layer costs what an unmasked one does. **The pixels are byte for byte what they were**:
`verification/B-170_held_table.md` draws 72 frames of the reference shot with a masked layer, with
and without effects, at Full and Draft, and compares each with one drawn from scratch; all are
identical, and on every held frame no mask is drawn and nothing is sent to the card. Exports never
use it.

On the reference shot held on twos with one masked layer, a held frame at Full went from
20.5 to 2.3 ms, and at Draft from 18.4 to 0.7 ms
(`verification/B-170_timing_table.md`). A frame with a new drawing costs what it did.

## What to check

Open the reference shot, set **Draw on: GPU** and **Full resolution**. Give one layer a mask (draw
a rectangle on it with the mask tool) and make sure its drawings are held for two frames or more.

1. **Play.** Press play. The masked layer should play smoothly and look as it did before: the
   same edge, the same feather, nothing flickering on held frames.
2. **Step.** Step through a few frames with the arrow keys. The masked edge should stay put on
   held frames and change only where the drawing changes.
3. **Edit the mask.** Drag a mask corner, then change the feather. The picture should update at
   once on the frame you are on, and on every other frame when you step to it.
4. **Add an effect.** Add a Gaussian Blur to the masked layer, then an Exposure. Play again; each
   should look right on held and new frames alike.
5. **Draft.** Switch to **Draft** and repeat step 1.

## If something is wrong

Say which step. The most likely fault would be a held frame showing the mask as it was before an
edit (step 3). The kept drawing is filed under the mask's shape and feather, so an edit should
always make a new one; the byte checks do not edit a mask, so this is the one to look at closely.
