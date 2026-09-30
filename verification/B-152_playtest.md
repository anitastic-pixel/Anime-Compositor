# B-152: motion blur and frame mix no longer push the whole frame to the processor, by hand

Built on 2026-09-29 under your "sounds great! let's do 1 through 7 to your discretion." (item 1 of
the GPU plan, D-218). Until now, one motion-blurred, frame-mixed or dissolved layer made the
processor draw the whole frame in the viewer, every other layer and effect included. Now the
processor builds only that layer's picture, as before, and the graphics card lays it with the rest
of the frame. Exports and renders to file are untouched.

`verification/B-152_card_whole_frame_table.md` is the comparison: every frame of every motion blur
and frame blending fixture, and the reference shot with motion blur on every layer and with frame
mix and a dissolve, at Full and Draft, drawn by the processor and by the card. **882 of 882 checks pass**: the card draws each of these frames itself, never more than 1 level of 255 from the processor's picture, with the same warnings. The worst frame's pictures are in `verification/B-152 pictures/`.

## Before you start

- Use the release build. It opens on the reference shot.
- Leave the viewer's **Draw on: Auto**. To compare, switch it to **Draw on: CPU** and **Draw on:
  GPU**; the picture must not change.

## What to check

1. **Motion blur.** Go to frame 90 and key layer 4's **Position** where it is. Go to frame 110
   and add 400 to its first number. Go to frame 100, turn on layer 4's motion-blur switch (the
   dot with three speed lines, after Shy) and the **Motion blur** button over the timeline.
   Layer 4 smears along its path, as it did before. Switch between **Draw on: CPU** and **Draw
   on: GPU**: the picture does not change.
2. **The message is gone.** On **Draw on: GPU**, with the smear showing, the viewer no longer
   says "The CPU drew this frame: it has motion blur".
3. **Frame mix.** Turn the **Motion blur** button off. Select layer 2, type 150 in **Time
   stretch** and tick **Frame mix**, then turn on **Frame blending** over the timeline. Step
   through a few frames: some show two drawings softly mixed, the same on CPU and GPU, and the
   viewer does not say the CPU drew the frame.
4. **Dissolve.** Select layer 4 and set **Drawing dissolve** to 2. Where one of its drawings is
   held for three or five frames, the last frames of the hold fade into the next drawing, the same
   on CPU and GPU.
5. **With a card effect.** Add a **Glow** to layer 1. With frame mix still on layer 2, the Glow
   looks the same on CPU and GPU, and the frame is still the card's.
6. **Draft.** Switch to **Draft** and repeat steps 1 and 3. CPU and GPU still match.
7. **Adjustment layers** still make the processor draw the whole frame, and say so. That is a
   later item of the plan.

## How long a frame takes

Measured on this machine with nothing else building. Milliseconds for one frame of the reference shot, the middle of several runs:

| Shot | Before | After |
|---|---:|---:|
| Motion blur, Full | 50.9 | 57.9 |
| Frame mix and dissolve, Draft | 50.1 | 63.1 |
| Frame mix and dissolve, Full | 61.7 | 66.1 |
| Motion blur with a Roughen Edges beside it, Full | 86.9 | 72.8 |

**Honestly: on its own this is slower** on a frame whose only special layers are blurred or mixed, because the processor's new picture of that layer is sent to the card every frame. It is faster where the card has its own effect to draw beside the blur. The next item of the plan (G2) makes that sending cheap; this playtest is best done after it, and the two will be measured together. The full table is `verification/B-152_card_whole_frame_timing_table.md`.

## What to report

"works", or which step number did something else and what it did, with the frame number.
