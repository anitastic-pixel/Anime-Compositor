# B-153b: blurred or mixed frames go back to the processor, by hand

Built on 2026-09-29 under your "sounds great! let's do 1 through 7 to your discretion." This is
the step D-218 promised if B-153 left such frames slower on the card. It is decision D-220.

B-152 had the graphics card draw frames with motion blur, frame mix or a drawing dissolve. B-153
made that cheaper, but some frames were still slower on the card. These are frames whose only
special layers are blurred or mixed, with no effect the card draws.

Now the viewer sends such a frame back to the processor, which draws it whole, as before B-152.
A frame that also has an effect the card draws, such as a Roughen Edges, stays on the card, where
it is faster. **Nothing you see changes.** Exports and renders to file are untouched.

One more fix came with it. At Draft, a frame with frame mix, a dissolve and a card effect kept
asking the card for new memory each frame. B-153's memory limit for sending pictures was too
small at Draft, because the pictures sent are full size while the Draft frame is half size. The
limit is now an eighth of the card's memory budget.

## The checks

- `verification/B-152_card_whole_frame_table.md`: **893 of 893 pass.** 292 are blurred or mixed
  frames with nothing for the card. Each is the processor's picture exactly, and the only extra
  message is the card's note that the processor drew it. Before this build 292 failed
  (`verification/B-153b_card_whole_frame_before.md`).
- `verification/B-153_kept_memory_table.md`: **12 of 12 pass.** The frame mix shot now has a
  Roughen Edges, so its frames stay on the card. Played twice, the card makes no new memory the
  second time through. Without the memory limit fix, it made 40 pieces at Draft. The plain motion
  blur shot now goes to the processor, so the card makes no memory for it at all.
- Every other card check passes, with its table unchanged.

## Before you start

- Use the release build. It opens on the reference shot.
- Leave the viewer's **Draw on: Auto**.

## What to check

1. **Motion blur alone.** Set up the motion blur as in B-152's sheet, step 1: layer 4 keyed from
   frame 90 to 110, its motion-blur switch on, and the **Motion blur** button on. Stop on frame
   100. Open the messages panel. It says: "The CPU drew this frame: it has motion blur or frame
   blending and no effect for the GPU, which the CPU draws faster."
2. **With a card effect.** Add a **Roughen Edges** to layer 1. Stop on frame 100 again. The
   message from step 1 is gone for this frame: the card draws it.
3. **Same picture.** On frame 100, switch between **Draw on: CPU** and **Draw on: GPU**. The
   smear and the rough edges look the same.
4. **Play.** Press play at **Draft**, then at **Full**. When it stops, read the sentence under
   the viewer: "Played ... frames in real time and dropped ...". It should drop no more frames
   than it did before B-152.

## How long a frame takes

Measured on this machine, three builds turn about. Milliseconds for one frame of the reference
shot, the middle of four runs on each build:

| Shot | Before B-152 | B-153 | Now |
|---|---:|---:|---:|
| Motion blur, Full | 47.5 | 58.5 | 47.6 |
| Frame mix and dissolve, Draft | 40.8 | 59.0 | 44.5 |
| Frame mix and dissolve, Full | 53.2 | 61.2 | 56.5 |
| Motion blur with Roughen Edges, Full | 86.5 | 54.3 | 54.6 |

Motion blur is back to where it was before B-152. Frame mix is faster than B-153. Its middle
number is 3 to 4 ms above before B-152, but it now runs the same steps as then, and its runs
spread from below to above the old ones. The frames with a card effect keep B-153's gain.

The full table is `verification/B-153b_timing_table.md`.

## What to report

"works", or the number of the step that did something else, what it did, and the frame number.
