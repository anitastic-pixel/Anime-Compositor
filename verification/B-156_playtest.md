# B-156: adjustment layers drawn on the graphics card

Built on 2026-09-30 under your "sounds great! let's do 1 through 7 to your discretion." This is
the first part of item 5 of the GPU plan, D-225.

Until now, any frame with an adjustment layer was sent whole to the processor, even when the
card could draw everything else in it. Now the card draws the layers beneath, runs the
adjustment layer's effects on that picture, and mixes the result back by what the adjustment
layer covers: its shape, its mask, its opacity and its matte. The processor does the same steps
in the same order.

**Nothing you should see changes.** The pictures may differ from the processor's by at most 1
level of 255 in a channel, the tolerance you already accepted for the card. Exports and renders
to file are untouched: they always use the processor.

Some adjustment layers still go to the processor, with the usual note that the CPU drew the
frame:
- one with a **Bloom, Glow, Paraffin, Kira-kira** or **HSV Key** (the same effects that can only
  begin a run on a layer, D-224), or a **Kaleidoscope** (D-240);
- on a card without double precision.

One finding changed how the card works under an adjustment layer. The card normally keeps each
drawing in half precision. Beneath an adjustment layer, effects such as Posterize, Threshold,
Halftone, a wipe or Light Rays turned that tiny rounding into differences of up to 255 levels:
a pixel just on one side of a threshold went to the other. So under an adjustment layer the card
now keeps every drawing at full precision, and the differences went away. It uses twice the
card memory for those drawings.

`verification/B-156_gpu_adjust_table.md` is the check. Each of the 64 effects the card draws is
put alone on an adjustment layer over the reference shot. Then runs of three: over the whole
frame, over part of it (scaled, turned, at 60% opacity), and between the second and third
layers. The ones that stay on the processor must give the processor's picture byte for byte.

**308 of 308 checks pass** (32 before the build):
- on the card, no pixel more than 1 level apart, at Draft and Full, frames 0 and 100;
- kept on the processor, the processor's own picture exactly;
- the same warnings on both.

The pictures of the closest case (Hue/Saturation alone, frame 100, Full) are in
`verification/B-156 pictures/`: the processor's, the card's, and the difference.

## Before you start

- Use the release build. It opens on the reference shot.
- To compare, switch between **Draw on: CPU** and **Draw on: GPU**. The picture must not change.

## What to check

1. **Over everything.** Select the top layer and add a **New adjustment layer** (Ctrl+Alt+Y).
   Put **Levels**, **Gaussian Blur** (10) and **Hue/Saturation** (hue 60) on it. Go to frame
   100 at **Full**. Switch between **Draw on: CPU** and **Draw on: GPU**. The picture looks the
   same, and on GPU the label above the viewer ends "drawn on GPU".
2. **Part of the frame.** Scale the adjustment layer to 60% and turn it a little; set its
   opacity to 60%. Switch between CPU and GPU. The same picture, including the soft edge where
   the adjustment stops.
3. **In the middle.** Drag the adjustment layer down between the second and third layers. The
   layers above it are not blurred. Switch between CPU and GPU: the same.
4. **A threshold.** Replace the three effects with a **Posterize** (6 levels), then a
   **Threshold**. Switch between CPU and GPU each time. The same picture, no speckles.
5. **Kept on the processor.** Add a **Glow** to the adjustment layer. On GPU the label now says
   the CPU drew the frame. Take the Glow away: it says GPU again.
6. **Play.** Press play at **Full** on **Draw on: GPU** and let it loop. Nothing flickers, and no
   stripes, black patches or old frames show.

## How long a frame takes

Measured on this machine, release build, the median of three runs
(`verification/B-156_timing_table.md`):

| Shot | Quality | Before ms | After ms |
|---|---|---:|---:|
| Levels, Gaussian Blur and Hue/Saturation over the whole frame | Draft | 18.1 | 13.8 |
| Levels, Gaussian Blur and Hue/Saturation over the whole frame | Full | 61.4 | 23.2 |
| the same, over part of it | Draft | 20.3 | 15.0 |
| the same, over part of it | Full | 62.1 | 23.2 |
| a moving Noise, then the same, over the whole frame | Draft | 19.1 | 15.5 |
| a moving Noise, then the same, over the whole frame | Full | 75.5 | 23.6 |

At Full a frame with an adjustment layer is about two and a half to three times quicker; at
Draft about a quarter quicker.

## What to report

"works", or the number of the step that did something else, what it did, and the frame number.
