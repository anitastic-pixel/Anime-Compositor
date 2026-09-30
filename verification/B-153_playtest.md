# B-153: the graphics card keeps its memory from frame to frame, by hand

Built on 2026-09-29 under your "sounds great! let's do 1 through 7 to your discretion." This is
item 2 of the GPU plan, D-219.

Until now, the card made new memory for every frame it drew:
- for each picture the processor sent it;
- for each drawing it held;
- for each step of its effects' work.

Now it makes that memory once and uses it again on the next frame. **Nothing you see changes.**
The pictures are the same to the byte. Frames with work for the card get faster. Exports and
renders to file are untouched.

`verification/B-153_kept_memory_table.md` is the check. It plays three versions of the reference
shot twice, at Draft and Full: with motion blur; with frame mix and a dissolve; and with motion
blur and a Roughen Edges.

**12 of 12 checks pass:**
- the frames are byte for byte those of a freshly opened card;
- the second time through, the card makes no new memory. Before this build it made about 180
  pieces each time through.

## Before you start

- Use the release build. It opens on the reference shot.
- Leave the viewer's **Draw on: Auto**. To compare, switch between **Draw on: CPU** and **Draw
  on: GPU**. The picture must not change.

## What to check

1. **Motion blur with a card effect.** Set up the motion blur as in B-152's sheet, step 1: layer
   4 keyed from frame 90 to 110, its motion-blur switch on, and the **Motion blur** button on.
   Add a **Roughen Edges** to layer 1. On **Draw on: GPU**, press play at **Full**. When it
   stops, read the sentence under the viewer: "Played ... frames in real time and dropped ...".
   Note how many frames were dropped. There is no "before" to compare with in the app. What
   matters is that it plays, and that steps 2 and 3 hold.
2. **Same picture.** Stop on frame 100. Switch between **Draw on: CPU** and **Draw on: GPU**.
   The Roughen Edges and the smear look the same.
3. **Long play.** Let it loop for a minute or two. The program must not slow down over time or
   run out of memory. Before, it made new memory each frame and let it go. Now it keeps the same
   memory.
4. **Resize.** Switch between **Draft** and **Full** a few times while it plays. The picture is
   right at each size, with no stripes, black patches or old frames showing through.

## How long a frame takes

Measured on this machine. Milliseconds for one frame of the reference shot, the middle of four
runs on each build:

| Shot | Before B-152 | B-152 | Now |
|---|---:|---:|---:|
| Motion blur, Draft | 15.9 | 19.7 | 15.6 |
| Motion blur, Full | 51.2 | 68.8 | 62.2 |
| Frame mix and dissolve, Draft | 48.8 | 61.3 | 56.7 |
| Frame mix and dissolve, Full | 62.0 | 67.0 | 65.2 |
| Motion blur with Roughen Edges, Full | 90.3 | 77.2 | 57.5 |

**Honestly:** a frame whose only special layers are blurred or mixed is still a little slower on
the card than when the processor drew it whole. So the next small step, B-153b, hands such a
frame back to the processor whenever it has no effect the card draws. Frames with a card effect,
like the Roughen Edges one, stay on the card, where they are now much faster.

**An observation.** During these measurements the card mostly sat in its power-saving state. The
viewer's short bursts of work do not wake it fully. The NVIDIA Control Panel's **Power management
mode: Prefer maximum performance** would likely make the card faster here. It is your machine's
setting, so this program leaves it alone. You don't need to change it for this playtest.

The full table is `verification/B-153_card_timing_table.md`.

## What to report

"works", or the number of the step that did something else, what it did, and the frame number.
