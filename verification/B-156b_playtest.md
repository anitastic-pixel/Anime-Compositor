# B-156b: motion blur added up on the graphics card

Built on 2026-09-30 under your "sounds great! let's do 1 through 7 to your discretion." This is
the second part of item 5 of the GPU plan, D-226.

Motion blur is made by drawing a moving layer at several moments between two frames and taking
the average. Since B-152 the card drew such frames, but the processor still did the averaging and
sent the result over. Now the card draws every moment itself and adds them up. The processor does
the same sum.

**Nothing you should see changes.** The pictures may differ from the processor's by at most 1
level of 255 in a channel, the tolerance you already accepted for the card. Exports and renders
to file are untouched: they always use the processor.

One case still goes to the processor, with the usual note that the CPU drew the frame: a layer
used as a **matte** that is itself motion-blurred.

`verification/B-156b_gpu_motion_blur_table.md` is the check: every frame of every motion blur
fixture and the reference shot with motion blur on every layer, at Draft and Full.
**175 of 175 checks pass** (44 before the build):
- on the card, no pixel more than 1 level apart;
- the blurred-matte fixture (fx_mb_025) kept on the processor, the processor's own picture exactly;
- every checked frame of the reference shot with motion blur drawn on the card;
- the same warnings on both.

The pictures of the closest case (the reference shot with motion blur, frame 150, Draft: 14,368
pixels 1 level apart) are in `verification/B-156b pictures/`: the processor's, the card's, and the
difference.

## Before you start

- Use the release build. It opens on the reference shot.
- To compare, switch between **Draw on: CPU** and **Draw on: GPU**. The picture must not change.

## What to check

1. **Motion blur.** Go to frame 90 and key layer 4's **Position** where it is. Go to frame 110
   and add 400 to its first number. Go to frame 100, turn on layer 4's motion-blur switch (the
   dot with three speed lines, after Shy) and the **Motion blur** button over the timeline.
   Layer 4 smears along its path. Switch between **Draw on: CPU** and **Draw on: GPU**: the
   picture does not change, and on GPU the label above the viewer ends "drawn on GPU".
2. **Turned.** Key layer 4's **Rotation** at frame 90 and set it to 90 at frame 110. At frame
   100 the smear curves. Switch between CPU and GPU: the same.
3. **With an effect.** Add a **Roughen Edges** to layer 4. The roughened, smeared layer looks the
   same on CPU and GPU.
4. **Draft.** Switch to **Draft** and repeat step 1. CPU and GPU still match.
5. **Play.** Press play at **Full** on **Draw on: GPU** and let it loop. Nothing flickers, and no
   stripes, black patches or old frames show. It should feel smoother than before.

## How long a frame takes

Measured on this machine, release build, the median of three runs
(`verification/B-156b_timing_table.md`):

| Shot | Quality | Before ms | After ms |
|---|---|---:|---:|
| the reference shot with motion blur | Draft | 15.4 | 8.6 |
| the reference shot with motion blur | Full | 47.4 | 15.2 |
| with motion blur and Roughen Edges | Draft | 25.8 | 12.5 |
| with motion blur and Roughen Edges | Full | 53.2 | 16.3 |

At Full a motion-blurred frame is about three times quicker; at Draft about twice as quick.

## What to report

"works", or the number of the step that did something else, what it did, and the frame number.
